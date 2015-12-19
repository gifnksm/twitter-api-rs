#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results)]

extern crate twitter_api as twitter;
extern crate rustc_serialize as rustc_serialize;
extern crate oauth_client as oauth;

use std::convert::AsRef;
use std::io;
use std::io::prelude::*;
use std::env;
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::path::PathBuf;
use rustc_serialize::Decodable;
use rustc_serialize::json::{self, Json};
use oauth::Token;

const TWITTER_CONF_FILENAME: &'static str = ".twitter.conf";

fn get_home_dir() -> PathBuf {
    match env::home_dir() {
        Some(p) => p,
        None => {
            panic!("Impossible to get your home dir!");
        }
    }
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Config {
    pub consumer_key: String,
    pub consumer_secret: String,
    pub access_key: String,
    pub access_secret: String,
}

impl Config {
    pub fn read(path_file: &Path) -> Option<Config> {
        let mut file = match File::open(path_file) {
            Ok(f) => f,
            Err(_) => return None,
        };
        let conf = Json::from_reader(&mut file).unwrap();
        Decodable::decode(&mut json::Decoder::new(conf)).ok()
    }

    pub fn write(&self, path_file: &Path) {
        let mut file = match OpenOptions::new().write(true).open(path_file) {
            Ok(f) => f,
            Err(e) => panic!("{}", e),
        };
        let _ = write!(&mut file, "{}\n", &json::encode(self).unwrap());
    }

    pub fn create(path_file: &Path) {
        match File::create(path_file) {
            Ok(_) => println!("File created!"),
            Err(_) => panic!("Problem to create the file...\nProgram aborted!"),
        }
    }
}

fn console_input(prompt: &str) -> String {
    println!("{} : ", prompt);
    let mut line = String::new();
    let _ = io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn help() -> () {
    println!("update status : update your status.");
    println!("get timeline : get your personal timeline in your console.")
}

fn main() {
    // Get the full path of the Twitter configuration path
    let mut twitter_conf_file_path: PathBuf = get_home_dir();
    twitter_conf_file_path.push(Path::new(TWITTER_CONF_FILENAME));

    println!("#####################");
    println!("#Welcome to Rwitter!#");
    println!("#####################");


    let conf = match Config::read(&twitter_conf_file_path) {
        Some(c) => c,
        None => {

            Config::create(&twitter_conf_file_path);

            let consumer_key = console_input("input your consumer key:");
            let consumer_secret = console_input("input your consumer secret:");
            let consumer = Token::new(consumer_key, consumer_secret);

            let request = twitter::get_request_token(&consumer).unwrap();
            println!("open the following url:");
            println!("\t{}", twitter::get_authorize_url(&request));
            let pin = console_input("input pin:");
            let access = twitter::get_access_token(&consumer, &request, &pin).unwrap();

            let c = Config {
                consumer_key: consumer.key.to_string(),
                consumer_secret: consumer.secret.to_string(),
                access_key: access.key.to_string(),
                access_secret: access.secret.to_string(),
            };

            c.write(&twitter_conf_file_path);
            c
        }
    };

    let consumer = Token::new(conf.consumer_key, conf.consumer_secret);
    let access = Token::new(conf.access_key, conf.access_secret);

    loop {
        let make_your_choice = console_input("What do you want to do?");

        match make_your_choice.as_ref() {
            "update status" => {
                let status = console_input("What's happening?");
                twitter::update_status(&consumer, &access, &status).unwrap();
            }
            "get timeline" => {
                let ts = twitter::get_last_tweets(&consumer, &access).unwrap();
                if ts.is_empty() {
                    println!("No tweet in your timeline...");
                } else {
                    for t in ts {
                        println!("{} - {}", t.created_at, t.text)
                    }
                }
            }
            "help" => {
                help();
            }
            _ => {
                println!("Bye!");
                break;
            }
        }
    }
}
