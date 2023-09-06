use std::fmt::{Display, Formatter, Result as FmtResult};

use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;
use serde_json::{json, to_string_pretty};

#[derive(Debug, Serialize)]
pub struct WebServerError {
    pub msg: String,
    pub status: u16,
}

impl Display for WebServerError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", to_string_pretty(self).unwrap())
    }
}

impl ResponseError for WebServerError {
    // builds the actual response to send back when an error occurs
    fn error_response(&self) -> HttpResponse {
        let err_json = json!({ "error": self.msg });
        HttpResponse::build(StatusCode::from_u16(self.status).unwrap()).json(err_json)
    }
}
