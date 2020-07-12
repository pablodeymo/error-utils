use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorMessage {
    pub message: String,
}

impl ErrorMessage {
    pub fn from_error(e: anyhow::Error) -> ErrorMessage {
        match e.downcast_ref::<String>() {
            Some(msg) => ErrorMessage {
                message: String::from(msg),
            },
            None => ErrorMessage {
                message: String::from("Unknown error"),
            },
        }
    }
}

impl FromStr for ErrorMessage {
    type Err = String;

    fn from_str(msg: &str) -> Result<Self, Self::Err> {
        Ok(ErrorMessage {
            message: String::from(msg),
        })
    }
}
