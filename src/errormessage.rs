use serde::{Deserialize, Serialize};

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

    pub fn from_str(msg: &str) -> ErrorMessage {
        ErrorMessage {
            message: String::from(msg),
        }
    }
}
