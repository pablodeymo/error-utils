use actix_web::{body, http::StatusCode, HttpResponse, ResponseError};
use anyhow::Result;
use serde::Serialize;
use serde_json::{json, to_string_pretty};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Serialize)]
pub struct MsgHttp {
    pub msg: String,
    #[serde(skip)] // no mostrar en la respuesta
    pub status: u16,
}

impl MsgHttp {
    #[must_use]
    pub fn new(msg: String, status: u16) -> MsgHttp {
        MsgHttp { msg, status }
    }

    /// Builds an `HttpResponse` with OK `StatusCode` and the body provided.
    ///
    /// # Errors
    ///
    /// Emit an error in case the body could not be serialized
    pub fn send_ok() -> Result<HttpResponse<body::BoxBody>> {
        Ok(HttpResponse::Ok()
            .append_header(("Content-Type", "application/json"))
            .body(serde_json::to_string(&MsgHttp {
                msg: "OK".to_owned(),
                status: 200,
            })?))
    }
}

impl Display for MsgHttp {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(
            f,
            "{}",
            match to_string_pretty(self) {
                Ok(s) => s,
                Err(_) => "error".to_owned(),
            }
        )
    }
}

impl ResponseError for MsgHttp {
    // builds the actual response to send back when an error occurs
    fn error_response(&self) -> HttpResponse<body::BoxBody> {
        let err_json = json!({ "error": self.msg });
        HttpResponse::build(
            StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
        )
        .json(err_json)
    }
}
