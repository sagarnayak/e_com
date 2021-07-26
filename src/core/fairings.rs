use rocket::{Data, Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

#[derive(Default, Clone)]
pub struct CounterFairing;

#[rocket::async_trait]
impl Fairing for CounterFairing {
    // This is a request and response fairing named "GET/POST Counter".
    fn info(&self) -> Info {
        Info {
            name: "GET/POST Counter",
            kind: Kind::Request | Kind::Response,
        }
    }

    // Increment the counter for `GET` and `POST` requests.
    async fn on_request(&self, _: &mut Request<'_>, _: &mut Data<'_>) {
    }

    async fn on_response<'r>(&self, _: &'r Request<'_>, _: &mut Response<'r>) {
    }
}