#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

extern crate "twitter-api" as twitter;
extern crate "rustc-serialize" as rustc_serialize;
extern crate "oauth-client" as oauth;

use std::io;
use std::io::prelude::*;
use std::fs::{File, OpenOptions};
use std::path::Path;
use rustc_serialize::Decodable;
use rustc_serialize::json::{self, Json};
use oauth::Token;

const PATH: &'static str = "./tweet.conf";

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Config {
    pub consumer_key: String,
    pub consumer_secret: String,
    pub access_key: String,
    pub access_secret: String
}

impl Config {
    pub fn read() -> Option<Config> {
        let path = Path::new(PATH);
        let mut file = match File::open(&path) {
            Ok(f) => f,
            Err(_) => return None
        };
        let conf = Json::from_reader(&mut file).unwrap();
        Decodable::decode(&mut json::Decoder::new(conf)).ok()
    }

    pub fn write(&self) {
        let path = Path::new(PATH);
        let mut file = match OpenOptions::new().write(true).open(&path) {
            Ok(f) => f,
            Err(e) => panic!("{}", e)
        };
        let _ = write!(&mut file, "{}\n", &json::encode(self).unwrap());
    }
}

fn console_input(prompt: &str) -> String {
    print!("{}\n\t", prompt);
    let mut line = String::new();
    let _ = io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn main() {
    let conf = match Config::read() {
        Some(c) => c,
        None => {
            let consumer_key    = console_input("input your consumer key:");
            let consumer_secret = console_input("input your consumer secret:");
            let consumer = Token::new(consumer_key, consumer_secret);

            let request = twitter::get_request_token(&consumer);
            println!("open the following url:");
            println!("\t{}", twitter::get_authorize_url(&request));
            let pin = console_input("input pin:");
            let access = twitter::get_access_token(&consumer, &request, &pin);

            let c = Config {
                consumer_key: consumer.key.to_string(),
                consumer_secret: consumer.secret.to_string(),
                access_key: access.key.to_string(),
                access_secret: access.secret.to_string()
            };
            c.write();
            c
        }
    };

    let consumer = Token::new(conf.consumer_key, conf.consumer_secret);
    let access = Token::new(conf.access_key, conf.access_secret);

    let status = console_input("What's happening?");
    twitter::tweet(&consumer, &access, &status);
}

