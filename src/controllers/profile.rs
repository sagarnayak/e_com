use std::thread::sleep;
use std::time::Duration;

use rand::Rng;
use rocket::http::Status;
use rocket::response::content;
use rocket::response::status;
use rocket::serde::json::Json;

use crate::model::user::User;

#[get("/me")]
pub fn me() -> &'static str {
    "Hello!"
}