use rocket::request::Request;
use rocket::response::{self, Responder, Response};

use crate::model::authentication_response::AuthenticationResponse;

#[rocket::async_trait]
impl<'r> Responder<'r, 'static> for AuthenticationResponse {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {

        println!("here ...");

        Response::build()
            .raw_header("Authorization", format!("Bearer {}", self.jwt))
            .ok()
    }
}