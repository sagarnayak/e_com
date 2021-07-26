use rocket::Route;

use crate::controllers::authentication_route::authenticate;

pub fn get_routes() -> Vec<Route> {
    routes![
        authenticate,
    ]
}