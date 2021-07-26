use std::io::Cursor;

use rocket::http::ContentType;
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StatusMessage {
    pub code: u16,
    pub message: String,
}

impl StatusMessage {
    pub fn bad_req(message: String) -> StatusMessage {
        StatusMessage {
            code: Status::BadRequest.code,
            message,
        }
    }
    pub fn bad_req_default() -> StatusMessage {
        StatusMessage {
            code: Status::BadRequest.code,
            message: "Getting some error ...".to_string(),
        }
    }
}

impl<'r> Responder<'r, 'static> for StatusMessage {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let json_string = rocket::serde::json::serde_json::to_string(&self).unwrap();
        Response::build()
            .sized_body(json_string.len(), Cursor::new(json_string))
            .header(ContentType::JSON)
            .ok()
    }
}