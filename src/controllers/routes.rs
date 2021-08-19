use rocket::Route;

use crate::controllers::auth_roles_cross_paths_route::get_available_paths;
use crate::controllers::authentication_route::authenticate;
use crate::controllers::index::index;
use crate::controllers::profile::me;
use crate::controllers::roles_route::create_role;
use crate::controllers::roles_route::find_roles_created_by_me;
use crate::controllers::roles_route::find_roles_created_by_specific_user;
use crate::controllers::roles_route::get_my_role;
use crate::controllers::token_renew::renew_token;
use crate::model::path::Path;

pub fn get_paths() -> Vec<Path> {
    let mut paths: Vec<Path> = vec![];

    paths.push(
        Path::new(
            "^(/)$",
            "/",
        )
            .get_available()
            .can_delegate_get()
            .force_delegate_get()
    );

    paths.push(
        Path::new(
            "^(/authenticate)$",
            "/authenticate",
        )
            .post_available()
            .can_delegate_post()
            .force_delegate_post()
    );

    paths.push(
        Path::new(
            "^(/paths)$",
            "/paths",
        )
            .get_available()
            .can_delegate_get()
            .force_delegate_get()
    );

    paths.push(
        Path::new(
            "^(/role)$",
            "/role",
        )
            .get_available()
            .can_delegate_get()
            .force_delegate_get()
            .post_available()
            .can_delegate_post()
    );

    paths.push(
        Path::new(
            "^(/roles)/?.*$",
            "/roles",
        )
            .get_available()
            .can_delegate_get()
            .force_delegate_get()
    );

    paths.push(
        Path::new(
            "^(/me)$",
            "/me",
        )
            .get_available()
            .can_delegate_get()
            .force_delegate_get()
    );

    paths.push(
        Path::new(
            "^(/renewToken)$",
            "/renewToken",
        )
            .post_available()
            .can_delegate_post()
            .force_delegate_post()
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
        renew_token,
    ]
}