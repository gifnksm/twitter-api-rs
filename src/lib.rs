#![warn(bad_style)]
// #![warn(missing_docs)]
#![warn(unused)]
#![warn(unused_extern_crates)]
#![warn(unused_import_braces)]
#![warn(unused_qualifications)]
#![warn(unused_results)]

use oauth::RequestBuilder;
use oauth_client::Token;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, collections::HashMap, str};
use thiserror::Error;

pub use oauth_client as oauth;
pub use serde_json;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    #[error("OAuth error: {0}")]
    Oauth(#[from] oauth::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("decode string error: {0}")]
    FromUtf8(#[from] str::Utf8Error),
}

mod api_twitter_oauth {
    pub const REQUEST_TOKEN: &str = "https://api.twitter.com/oauth/request_token";
    pub const AUTHORIZE: &str = "https://api.twitter.com/oauth/authorize";
    pub const ACCESS_TOKEN: &str = "https://api.twitter.com/oauth/access_token";
}

mod api_twitter_soft {
    pub const UPDATE_STATUS: &str = "https://api.twitter.com/1.1/statuses/update.json";
    pub const HOME_TIMELINE: &str = "https://api.twitter.com/1.1/statuses/home_timeline.json";
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tweet {
    pub created_at: String,
    pub text: String,
}

impl Tweet {
    pub fn parse_timeline(json_str: impl AsRef<str>) -> Result<Vec<Tweet>> {
        let tweets = serde_json::from_str(json_str.as_ref())?;
        Ok(tweets)
    }
}

fn split_query(query: &str) -> HashMap<Cow<'_, str>, Cow<'_, str>> {
    let mut param = HashMap::new();
    for q in query.split('&') {
        let (k, v) = q.split_once('=').unwrap();
        let _ = param.insert(k.into(), v.into());
    }
    param
}

pub fn get_request_token<RB>(
    consumer: &Token<'_>,
    client: &RB::ClientBuilder,
) -> Result<Token<'static>>
where
    RB: RequestBuilder,
    RB::ReturnValue: AsRef<[u8]>,
{
    let bytes: RB::ReturnValue = oauth::get::<RB>(
        api_twitter_oauth::REQUEST_TOKEN,
        consumer,
        None,
        None,
        client,
    )?;
    let resp = str::from_utf8(bytes.as_ref())?;
    let param = split_query(resp);
    let token = Token::new(
        param.get("oauth_token").unwrap().to_string(),
        param.get("oauth_token_secret").unwrap().to_string(),
    );
    Ok(token)
}

pub fn get_authorize_url(request: &Token<'_>) -> String {
    format!(
        "{}?oauth_token={}",
        api_twitter_oauth::AUTHORIZE,
        request.key
    )
}

pub fn get_access_token<RB>(
    consumer: &Token<'_>,
    request: &Token<'_>,
    pin: &str,
    client: &RB::ClientBuilder,
) -> Result<Token<'static>>
where
    RB: RequestBuilder,
    RB::ReturnValue: AsRef<[u8]>,
{
    let mut param = HashMap::new();
    let _ = param.insert("oauth_verifier".into(), pin.into());
    let bytes = oauth::get::<RB>(
        api_twitter_oauth::ACCESS_TOKEN,
        consumer,
        Some(request),
        Some(&param),
        client,
    )?;
    let resp = str::from_utf8(bytes.as_ref())?;
    let param = split_query(resp);
    let token = Token::new(
        param.get("oauth_token").unwrap().to_string(),
        param.get("oauth_token_secret").unwrap().to_string(),
    );
    Ok(token)
}

/// function to update the status
/// This function takes as arguments the consumer key, the access key, and the status (obviously)
pub fn update_status<RB>(
    consumer: &Token<'_>,
    access: &Token<'_>,
    status: &str,
    client: &RB::ClientBuilder,
) -> Result<()>
where
    RB: RequestBuilder,
    RB::ReturnValue: AsRef<[u8]>,
{
    let mut param = HashMap::new();
    let _ = param.insert("status".into(), status.into());
    let _ = oauth::post::<RB>(
        api_twitter_soft::UPDATE_STATUS,
        consumer,
        Some(access),
        Some(&param),
        client,
    )?;
    Ok(())
}

pub fn get_last_tweets<RB>(
    consumer: &Token<'_>,
    access: &Token<'_>,
    client: &RB::ClientBuilder,
) -> Result<Vec<Tweet>>
where
    RB: RequestBuilder,
    RB::ReturnValue: AsRef<[u8]>,
{
    let bytes = oauth::get::<RB>(
        api_twitter_soft::HOME_TIMELINE,
        consumer,
        Some(access),
        None,
        client,
    )?;
    let last_tweets_json = str::from_utf8(bytes.as_ref())?;
    let ts = Tweet::parse_timeline(&last_tweets_json)?;
    Ok(ts)
}
