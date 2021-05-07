#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use color_eyre::{
    eyre::{bail, eyre, Error, Result, WrapErr as _},
    owo_colors::OwoColorize as _,
};
use oauth_client::Token;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{
    fmt::Display,
    fs::{self, File},
    io::{self, prelude::*, BufReader, BufWriter},
    path::Path,
};
use twitter_api as twitter;

const APP_NAME: &str = "Rwitter";
const TWITTER_CONF_FILENAME: &str = "rwitter.conf";

macro_rules! error {
    ($($args:tt)*) => { eprintln!("{} {}", "ERROR".red(), format_args!($($args)*))};
}

fn main() -> Result<()> {
    color_eyre::install()?;

    eprintln!("Welcome to {}!", APP_NAME.bold());
    eprintln!();

    let conf_path = conf_file::path()?;
    let mut conf = conf_file::read_or_create(&conf_path)?;

    loop {
        let make_your_choice =
            console::input("What do you want to do? (input `help` to show help)")?;

        match make_your_choice.as_str() {
            "update status" => {
                if let Err(err) = command::update_status(&conf) {
                    error!("Failed to update status");
                    eprintln!("Error detail: {:?}", err);
                }
            }
            "get timeline" => {
                if let Err(err) = command::get_timeline(&conf) {
                    error!("Failed to update status");
                    eprintln!("Error detail: {:?}", err);
                }
            }
            "update config" => {
                if let Err(err) = command::update_config(&conf_path, &mut conf) {
                    error!("Failed to update config");
                    eprintln!("Error detail: {:?}", err);
                }
            }
            "help" => {
                command::help();
            }
            "bye" | "" => {
                eprintln!("Bye!");
                break;
            }
            input => {
                error!("unknown input: {}", input);
                command::help();
            }
        }
        eprintln!();
    }

    Ok(())
}

mod console {
    use super::*;

    pub(super) fn input_common(
        prompt: impl Display,
        default_value: Option<impl Display>,
    ) -> Result<String> {
        eprintln!();
        match &default_value {
            Some(default_value) => {
                eprintln!("{}: [default: {}]", prompt.underline(), default_value)
            }
            None => eprintln!("{}:", prompt.underline()),
        }
        eprint!("  > ");

        let mut line = String::new();
        let _ = io::stdin()
            .read_line(&mut line)
            .wrap_err("failed to get user input")?;
        let trimmed = line.trim();
        let input = match (trimmed.is_empty(), default_value) {
            (true, Some(default_value)) => default_value.to_string(),
            _ => trimmed.to_string(),
        };
        Ok(input)
    }

    pub(super) fn input(prompt: impl Display) -> Result<String> {
        input_common(prompt, None::<String>)
    }

    pub(super) fn input_with_default_value(
        prompt: impl Display,
        default_value: impl Display,
    ) -> Result<String> {
        input_common(prompt, Some(default_value))
    }

    pub(super) fn yes_no(prompt: impl Display) -> Result<bool> {
        let input = input(format_args!("{} [yes/no]", prompt))?;
        match input.to_ascii_lowercase().as_str() {
            "yes" | "y" => Ok(true),
            _ => Ok(false),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    consumer_key: String,
    consumer_secret: String,
    access_key: String,
    access_secret: String,
}

impl Config {
    fn from_user_input(old_value: Option<&Self>) -> Result<Self> {
        let input = |prompt, old_value| {
            match old_value {
                Some(value) => console::input_with_default_value(prompt, value),
                None => console::input(prompt),
            }
            .and_then(|s| {
                if s.is_empty() {
                    bail!("cancelled by user")
                } else {
                    Ok(s)
                }
            })
        };

        loop {
            let consumer_key = input(
                "Input your `consumer key`",
                old_value.map(|c| &c.consumer_key),
            )?;
            let consumer_secret = input(
                "Input your `consumer secret`",
                old_value.map(|c| &c.consumer_secret),
            )?;
            let consumer_token = Token::new(consumer_key, consumer_secret);

            let request_token = match twitter::get_request_token(&consumer_token) {
                Ok(token) => token,
                Err(err) => {
                    error!("Failed to get `request token`: {:?}", err);
                    continue;
                }
            };

            eprintln!();
            eprintln!(
                "{}",
                "Open the following URL and authorize this application:".underline()
            );
            eprintln!("  {}", twitter::get_authorize_url(&request_token));

            let pin = input("Input PIN:", None)?;
            let access_token =
                match twitter::get_access_token(&consumer_token, &request_token, &pin) {
                    Ok(token) => token,
                    Err(err) => {
                        error!("Failed to get `access token`: {:?}", err);
                        continue;
                    }
                };

            let conf = Self {
                consumer_key: consumer_token.key.to_string(),
                consumer_secret: consumer_token.secret.to_string(),
                access_key: access_token.key.to_string(),
                access_secret: access_token.secret.to_string(),
            };
            return Ok(conf);
        }
    }

    fn from_reader(mut reader: impl Read) -> Result<Self> {
        let conf = serde_json::from_reader(&mut reader)?;
        Ok(conf)
    }

    fn write(&self, mut writer: impl Write) -> Result<()> {
        serde_json::to_writer_pretty(&mut writer, self)?;
        Ok(())
    }

    fn consumer_token(&self) -> Token {
        Token::new(&self.consumer_key, &self.consumer_secret)
    }

    fn access_token(&self) -> Token {
        Token::new(&self.access_key, &self.access_secret)
    }
}

mod conf_file {
    use super::*;

    pub(super) fn path() -> Result<PathBuf> {
        let conf_dir =
            dirs::config_dir().ok_or_else(|| eyre!("failed to get your config directory path"))?;
        let conf_path = conf_dir.join(TWITTER_CONF_FILENAME);
        Ok(conf_path)
    }

    pub(super) fn read_or_create(path: impl AsRef<Path>) -> Result<Config> {
        let path = path.as_ref();

        match read(&path) {
            // config read from the file successfully
            Ok(Some(conf)) => Ok(conf),

            // config file does not exist
            Ok(None) => create_by_user_input(&path),

            // config file exists, but cannot be read successfully
            Err(err) => {
                error!("Failed to read the existing config file");
                eprintln!("Error detail: {:?}", err);
                confirm_and_recreate(&path)
            }
        }
    }

    pub(super) fn save(path: impl AsRef<Path>, conf: &Config) -> Result<()> {
        let path = path.as_ref();

        let mut file = create(&path)?;
        write(&path, &mut file, conf)?;
        Ok(())
    }

    fn create(path: impl AsRef<Path>) -> Result<File> {
        let path = path.as_ref();

        File::create(path)
            .wrap_err_with(|| format!("failed to create a config file: {}", path.display()))
    }

    fn read(path: impl AsRef<Path>) -> Result<Option<Config>> {
        let path = path.as_ref();

        let file = match File::open(&path) {
            Ok(file) => file,
            Err(err) if err.kind() == io::ErrorKind::NotFound => return Ok(None),
            Err(err) => {
                bail!(Error::from(err)
                    .wrap_err(format!("failed to open a config file: {}", path.display())))
            }
        };

        let mut file = BufReader::new(file);
        let conf = Config::from_reader(&mut file)
            .wrap_err_with(|| format!("failed to read a config file: {}", path.display()))?;
        Ok(Some(conf))
    }

    fn write(path: impl AsRef<Path>, file: &mut File, conf: &Config) -> Result<()> {
        let path = path.as_ref();

        let mut file = BufWriter::new(file);
        conf.write(&mut file)
            .wrap_err_with(|| format!("failed to save config file: {}", path.display()))?;
        file.flush()
            .wrap_err_with(|| format!("failed to save config file: {}", path.display()))?;
        Ok(())
    }

    fn create_by_user_input(path: impl AsRef<Path>) -> Result<Config> {
        let path = path.as_ref();
        assert!(path.is_file());
        let conf_dir = path.parent().unwrap();

        // Ensure config directory exists
        fs::create_dir_all(&conf_dir).wrap_err_with(|| {
            format!(
                "failed to create a directory for storing config file: {}",
                conf_dir.display()
            )
        })?;

        // Test the config file creatable before user inputs the auth info.
        let mut file = create(&path)?;

        let res = Config::from_user_input(None).and_then(|conf| {
            write(&path, &mut file, &conf)?;
            Ok(conf)
        });
        drop(file);

        if res.is_err() {
            // delete config file
            let _ = fs::remove_file(&path);
        }

        res
    }

    fn confirm_and_recreate(path: impl AsRef<Path>) -> Result<Config> {
        let recreate = console::yes_no("Recreate a config file?")?;
        if !recreate {
            bail!("canceled by user");
        }
        create_by_user_input(path)
    }
}

mod command {
    use super::*;

    pub(super) fn update_status(config: &Config) -> Result<()> {
        let status = console::input("What's happening?")?;
        if status.is_empty() {
            return Ok(());
        }

        let consumer = config.consumer_token();
        let access = config.access_token();
        twitter::update_status(&consumer, &access, &status)
            .wrap_err("failed to invoking update_status API")?;

        Ok(())
    }

    pub(super) fn get_timeline(config: &Config) -> Result<()> {
        let consumer = config.consumer_token();
        let access = config.access_token();
        let ts = twitter::get_last_tweets(&consumer, &access)
            .wrap_err("failed to invoking get_timeline API")?;
        if ts.is_empty() {
            eprintln!("No tweet in your timeline...");
        } else {
            for t in ts {
                eprintln!("  {} - {}", t.created_at, t.text)
            }
        }
        Ok(())
    }

    pub(super) fn update_config(conf_path: impl AsRef<Path>, config: &mut Config) -> Result<()> {
        let conf_path = conf_path.as_ref();

        let new_config = Config::from_user_input(Some(config))?;
        conf_file::save(conf_path, &new_config)
            .wrap_err_with(|| format!("failed to save a config file: {}", conf_path.display()))?;

        *config = new_config;

        Ok(())
    }

    pub(super) fn help() {
        eprintln!();
        let commands = &[
            ("update status", "update your status"),
            (
                "get timeline",
                "get your personal timeline in your console.",
            ),
            ("update config", "update auth configurations."),
            ("bye", "exit this program"),
        ];

        eprintln!("{}", "Available commands:".underline());
        for (command, description) in commands.iter() {
            eprintln!("  {:20} : {}", command.bold(), description);
        }
    }
}
