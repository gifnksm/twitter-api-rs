#![warn(bad_style)]
// #![warn(missing_docs)]
#![warn(unused)]
#![warn(unused_extern_crates)]
#![warn(unused_import_braces)]
#![warn(unused_qualifications)]
#![warn(unused_results)]

extern crate oauth_client as oauth;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub use error::Error;
use oauth::Token;
use std::borrow::Cow;
use std::collections::HashMap;

mod error;

mod api_twitter_oauth {
    pub const REQUEST_TOKEN: &'static str = "https://api.twitter.com/oauth/request_token";
    pub const AUTHORIZE: &'static str = "https://api.twitter.com/oauth/authorize";
    pub const ACCESS_TOKEN: &'static str = "https://api.twitter.com/oauth/access_token";
}

mod api_twitter_soft {
    pub const UPDATE_STATUS: &'static str = "https://api.twitter.com/1.1/statuses/update.json";
    pub const HOME_TIMELINE: &'static str = "https://api.twitter.com/1.1/statuses/home_timeline.\
                                             json";
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tweet {
    pub created_at: String,
    pub text: String,
}

impl Tweet {
    pub fn parse_timeline(json_string: String) -> Result<Vec<Tweet>, Error> {
        let conf = serde_json::from_str(&json_string)?;
        Ok(conf)
    }
}

fn split_query<'a>(query: &'a str) -> HashMap<Cow<'a, str>, Cow<'a, str>> {
    let mut param = HashMap::new();
    for q in query.split('&') {
        let mut s = q.splitn(2, '=');
        let k = s.next().unwrap();
        let v = s.next().unwrap();
        let _ = param.insert(k.into(), v.into());
    }
    param
}

pub fn get_request_token(consumer: &Token) -> Result<Token<'static>, Error> {
    let bytes = oauth::get(api_twitter_oauth::REQUEST_TOKEN, consumer, None, None)?;
    let resp = String::from_utf8(bytes)?;
    let param = split_query(&resp);
    let token = Token::new(param.get("oauth_token").unwrap().to_string(),
                           param.get("oauth_token_secret").unwrap().to_string());
    Ok(token)
}

pub fn get_authorize_url(request: &Token) -> String {
    format!("{}?oauth_token={}",
            api_twitter_oauth::AUTHORIZE,
            request.key)
}

pub fn get_access_token(consumer: &Token,
                        request: &Token,
                        pin: &str)
                        -> Result<Token<'static>, Error> {
    let mut param = HashMap::new();
    let _ = param.insert("oauth_verifier".into(), pin.into());
    let bytes = oauth::get(api_twitter_oauth::ACCESS_TOKEN,
                           consumer,
                           Some(request),
                           Some(&param))?;
    let resp = String::from_utf8(bytes)?;
    let param = split_query(&resp);
    let token = Token::new(param.get("oauth_token").unwrap().to_string(),
                           param.get("oauth_token_secret").unwrap().to_string());
    Ok(token)
}

/// function to update the status
/// This function takes as arguments the consumer key, the access key, and the status (obviously)
pub fn update_status(consumer: &Token, access: &Token, status: &str) -> Result<(), Error> {
    let mut param = HashMap::new();
    let _ = param.insert("status".into(), status.into());
    let _ = oauth::post(api_twitter_soft::UPDATE_STATUS,
                        consumer,
                        Some(access),
                        Some(&param))?;
    Ok(())
}

pub fn get_last_tweets(consumer: &Token, access: &Token) -> Result<Vec<Tweet>, Error> {
    let bytes = oauth::get(api_twitter_soft::HOME_TIMELINE,
                           consumer,
                           Some(access),
                           None)?;
    let last_tweets_json = String::from_utf8(bytes)?;
    let ts = Tweet::parse_timeline(last_tweets_json)?;
    Ok(ts)
}
