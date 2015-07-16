#![warn(bad_style, missing_docs,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results)]

#![feature(into_cow)]

extern crate oauth_client as oauth;

use std::borrow::{Cow, IntoCow};
use std::collections::HashMap;
use oauth::Token;

mod api_twitter_oauth {
    pub const REQUEST_TOKEN: &'static str   = "https://api.twitter.com/oauth/request_token";
    pub const AUTHORIZE: &'static str       = "https://api.twitter.com/oauth/authorize";
    pub const ACCESS_TOKEN: &'static str    = "https://api.twitter.com/oauth/access_token";
}

mod api_twitter_soft {
    pub const UPDATE_STATUS: &'static str = "https://api.twitter.com/1.1/statuses/update.json";
    pub const HOME_TIMELINE: &'static str = "https://api.twitter.com/1.1/statuses/home_timeline.json";
}

fn split_query<'a>(query: &'a str) -> HashMap<Cow<'a, str>, Cow<'a, str>> {
    let mut param = HashMap::new();
    for q in query.split('&') {
        let mut s = q.splitn(2, '=');
        let k = s.next().unwrap();
        let v = s.next().unwrap();
        let _ = param.insert(k.into_cow(), v.into_cow());
    }
    param
}

pub fn get_request_token(consumer: &Token) -> Token<'static> {
    let resp = oauth::get(api_twitter_oauth::REQUEST_TOKEN, consumer, None, None);
    let param = split_query(&resp);
    Token::new(param.get("oauth_token").unwrap().to_string(),
               param.get("oauth_token_secret").unwrap().to_string())
}

pub fn get_authorize_url(request: &Token) -> String {
    format!("{}?oauth_token={}", api_twitter_oauth::AUTHORIZE, request.key)
}

pub fn get_access_token(consumer: &Token, request: &Token, pin: &str) -> Token<'static> {
    let mut param = HashMap::new();
    let _ = param.insert("oauth_verifier".into_cow(), pin.into_cow());
    let resp = oauth::get(api_twitter_oauth::ACCESS_TOKEN, consumer, Some(request), Some(&param));
    let param = split_query(&resp);
    Token::new(param.get("oauth_token").unwrap().to_string(),
               param.get("oauth_token_secret").unwrap().to_string())
}

/// function to update the status
/// This function takes as arguments the consumer key, the access key, and the status (obviously)
pub fn update_status(consumer: &Token, access: &Token, status: &str) {
    let mut param = HashMap::new();
    let _ = param.insert("status".into_cow(), status.into_cow());
    let _ = oauth::post(api_twitter_soft::UPDATE_STATUS, consumer, Some(access), Some(&param));
}

pub fn get_last_tweets(consumer: &Token, access: &Token) {
    let mut param = HashMap::new();
    println!("{:?}", oauth::get(api_twitter_soft::HOME_TIMELINE, consumer, Some(access), Some(&param)));
}
