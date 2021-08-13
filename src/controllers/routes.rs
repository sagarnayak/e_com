use rocket::Route;

use crate::controllers::auth_roles_cross_paths_route::get_available_paths;
use crate::controllers::authentication_route::authenticate;
use crate::controllers::index::index;
use crate::controllers::profile::me;
use crate::controllers::redis::redis;
use crate::controllers::roles_route::create_role;
use crate::controllers::roles_route::find_roles_created_by_me;
use crate::controllers::roles_route::find_roles_created_by_specific_user;
use crate::controllers::roles_route::get_my_role;
use crate::model::path::Path;

pub fn get_paths() -> Vec<Path> {
    let mut paths: Vec<Path> = vec![];

    paths.push(
        Path {
            id: None,
            path: "/".to_string(),
            get_available: true,
            post_available: false,
            put_available: false,
            delete_available: false,
            created: None,
            modified: None,
        }
    );

    paths.push(
        Path {
            id: None,
            path: "/authenticate".to_string(),
            get_available: false,
            post_available: true,
            put_available: false,
            delete_available: false,
            created: None,
            modified: None,
        }
    );

    paths.push(
        Path {
            id: None,
            path: "/paths".to_string(),
            get_available: true,
            post_available: false,
            put_available: false,
            delete_available: false,
            created: None,
            modified: None,
        }
    );

    paths.push(
        Path {
            id: None,
            path: "/role".to_string(),
            get_available: true,
            post_available: true,
            put_available: false,
            delete_available: false,
            created: None,
            modified: None,
        }
    );

    paths.push(
        Path {
            id: None,
            path: "/roles".to_string(),
            get_available: true,
            post_available: false,
            put_available: false,
            delete_available: false,
            created: None,
            modified: None,
        }
    );

    paths.push(
        Path {
            id: None,
            path: "/me".to_string(),
            get_available: true,
            post_available: false,
            put_available: false,
            delete_available: false,
            created: None,
            modified: None,
        }
    );

    paths
}

pub fn get_routes() -> Vec<Route> {
    routes![
       authenticate,
        get_available_paths,
        get_my_role,
        create_role,
        find_roles_created_by_me,
        find_roles_created_by_specific_user,
        me,
        index,
        redis,
    ]
}