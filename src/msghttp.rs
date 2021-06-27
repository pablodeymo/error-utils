use actix_http::body::Body;
use actix_http::Response;
use actix_web::http::StatusCode;
use actix_web::{web, ResponseError};
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
    pub fn new(msg: String, status: u16) -> MsgHttp {
        MsgHttp { msg, status }
    }

    pub fn send_ok() -> Result<web::HttpResponse<Body>> {
        Ok(Response::build(StatusCode::OK)
            .append_header(("Content-Type", "application/json"))
            .body(
                serde_json::to_string(&MsgHttp {
                    msg: "OK".to_owned(),
                    status: 200,
                })
                .unwrap(),
            ))
    }
}

impl Display for MsgHttp {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", to_string_pretty(self).unwrap())
    }
}

impl ResponseError for MsgHttp {
    // builds the actual response to send back when an error occurs
    fn error_response(&self) -> actix_web::HttpResponse<Body> {
        let err_json = json!({ "error": self.msg });
        actix_web::HttpResponse::build(StatusCode::from_u16(self.status).unwrap()).json(err_json)
    }
}
