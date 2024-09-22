use std::str::FromStr;

use derive_more::From;
use rocket::request::FromParam;
use serde::{Deserialize, Serialize};

use crate::domain::clip::ClipError;

// derive_more From will automatically implement From trait to convert a String into ShortCode
#[derive(Debug, Clone, Deserialize, Serialize, From)]
pub struct ShortCode(String);

impl ShortCode {
    pub fn new() -> Self {
        use rand::prelude::*;
        let allowed_chars = [
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '1', '2', '3', '4', '5', '6', '7', '8',
            '9', '0',
        ];

        let mut rng = thread_rng();
        let mut shortcode = String::with_capacity(10);
        for _ in 0..10 {
            shortcode.push(
                *allowed_chars
                    .choose(&mut rng)
                    .expect("sampling array should have values"),
            )
        }
        Self(shortcode)
    }

    pub fn into_inner(self) -> String {
        self.0
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl Default for ShortCode {
    fn default() -> Self {
        Self::new()
    }
}

// Since we are going to use ShortCode frequently within the web portion
// so below implement some From for easily convertion between ShortCode and String
impl From<ShortCode> for String {
    fn from(value: ShortCode) -> Self {
        value.0
    }
}

impl From<&str> for ShortCode {
    fn from(value: &str) -> Self {
        ShortCode(value.to_owned())
    }
}

// Turn data in URL into parameter
impl<'r> FromParam<'r> for ShortCode {
    type Error = &'r str;

    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        Ok(ShortCode::from(param))
    }
}

// This one is done for completeness, won't be used later due to the complexity
// introduce by the fact it returns Result instead of ShortCode as the From<&str> does
impl FromStr for ShortCode {
    type Err = ClipError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // This .into() functionality is implemented above at From<&str>: &str -> ShortCode
        Ok(Self(s.into()))
    }
}
