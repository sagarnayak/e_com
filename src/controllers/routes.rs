use rocket::Route;

use crate::controllers::authentication_route::authenticate;
use crate::controllers::profile::me;

pub fn get_routes() -> Vec<Route> {
    routes![
        authenticate,
        me,
    ]
}