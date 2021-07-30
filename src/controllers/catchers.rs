use rocket::http::Status;
use rocket::Request;
use rocket::response::status;
use rocket::serde::json::Json;

use crate::core::strings::{GENERIC, NOT_FOUND};
use crate::model::status_message::StatusMessage;

#[catch(404)]
pub fn for_404(_: Status, req: &Request) -> status::Custom<Json<StatusMessage>> {
    StatusMessage::not_found_404_with_status_code(
        format!("{} : {}", NOT_FOUND, req.uri())
    )
}

#[catch(default)]
pub fn not_found(status: Status, req: &Request) -> status::Custom<Json<StatusMessage>> {
    StatusMessage::custom_with_status_code(
        format!("{} :: {}", GENERIC, req.uri()),
        status,
    )
}