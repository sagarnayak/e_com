use rocket::Route;

use crate::controllers::authentication_route::authenticate;
use crate::controllers::profile::me;
use crate::model::path::Path;

pub fn get_paths() -> Vec<Path> {
    let mut paths: Vec<Path> = vec![];

    paths.push(
        Path {
            id: None,
            path: "/authenticate".to_string(),
            get: false,
            post: true,
            put: false,
            delete: false,
            created: None,
            modified: None,
        }
    );

    paths
}

pub fn get_routes() -> Vec<Route> {
    routes![
       authenticate
    ]
}