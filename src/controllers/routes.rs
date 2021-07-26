use rocket::Route;

use crate::controllers::generic_apis::index;

pub fn get_routes() -> Vec<Route> {
    routes![
        index,
    ]
}