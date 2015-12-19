//! Provides utility methods for basic interaction with Twitter API.

#![warn(bad_style)]
#![warn(missing_docs)]
#![warn(unused)]
#![warn(unused_extern_crates)]
#![warn(unused_import_braces)]
#![warn(unused_qualifications)]
#![warn(unused_results)]

extern crate oauth_client as oauth;

use std::borrow::Cow;
use std::collections::HashMap;
use oauth::Token;

mod api {
    pub const REQUEST_TOKEN: &'static str   = "https://api.twitter.com/oauth/request_token";
    pub const AUTHORIZE: &'static str       = "https://api.twitter.com/oauth/authorize";
    pub const ACCESS_TOKEN: &'static str    = "https://api.twitter.com/oauth/access_token";
    pub const STATUSES_UPDATE: &'static str = "https://api.twitter.com/1.1/statuses/update.json";
}

/// Creates a map from query parameter key value pairs.
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

/// Fetch token and token secret and construct a new request token from query params
pub fn get_request_token(consumer: &Token) -> Token<'static> {
    let resp = oauth::get(api::REQUEST_TOKEN, consumer, None, None);
    let param = split_query(&resp);
    Token::new(param.get("oauth_token").unwrap().to_string(),
               param.get("oauth_token_secret").unwrap().to_string())
}

/// Format authorize url, combining auth url and request key
pub fn get_authorize_url(request: &Token) -> String {
    format!("{}?oauth_token={}", api::AUTHORIZE, request.key)
}

/// Retrieve access token by use of pin, request token
pub fn get_access_token(consumer: &Token, request: &Token, pin: &str) -> Token<'static> {
    let mut param = HashMap::new();
    let _ = param.insert("oauth_verifier".into(), pin.into());
    let resp = oauth::get(api::ACCESS_TOKEN, consumer, Some(request), Some(&param));
    let param = split_query(&resp);
    Token::new(param.get("oauth_token").unwrap().to_string(),
               param.get("oauth_token_secret").unwrap().to_string())
}

/// With key, secret, and access token, post status to Twitter
pub fn tweet(consumer: &Token, access: &Token, status: &str) {
    let mut param = HashMap::new();
    let _ = param.insert("status".into(), status.into());
    let _ = oauth::post(api::STATUSES_UPDATE, consumer, Some(access), Some(&param));
}
