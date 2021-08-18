use rocket::Route;

use crate::controllers::auth_roles_cross_paths_route::get_available_paths;
use crate::controllers::authentication_route::authenticate;
use crate::controllers::index::index;
use crate::controllers::profile::me;
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
            path: "^(/)$".to_string(),
            readable_path: "/".to_owned(),
            get_available: true,
            post_available: false,
            put_available: false,
            delete_available: false,
            can_delegate_get: true,
            can_delegate_post: false,
            can_delegate_put: false,
            can_delegate_delete: false,
            force_delegate_get: true,
            force_delegate_post: false,
            force_delegate_put: false,
            force_delegate_delete: false,
            can_access_for_children_get: false,
            can_access_for_children_post: false,
            can_access_for_children_put: false,
            can_access_for_children_delete: false,
            created: None,
            modified: None,
        }
    );

    paths.push(
        Path {
            id: None,
            path: "^(/authenticate)$".to_string(),
            readable_path: "/authenticate".to_owned(),
            get_available: false,
            post_available: true,
            put_available: false,
            delete_available: false,
            can_delegate_get: false,
            can_delegate_post: true,
            can_delegate_put: false,
            can_delegate_delete: false,
            force_delegate_get: false,
            force_delegate_post: true,
            force_delegate_put: false,
            force_delegate_delete: false,
            can_access_for_children_get: false,
            can_access_for_children_post: false,
            can_access_for_children_put: false,
            can_access_for_children_delete: false,
            created: None,
            modified: None,
        }
    );

    paths.push(
        Path {
            id: None,
            path: "^(/paths)$".to_string(),
            readable_path: "/paths".to_owned(),
            get_available: true,
            post_available: false,
            put_available: false,
            delete_available: false,
            can_delegate_get: true,
            can_delegate_post: false,
            can_delegate_put: false,
            can_delegate_delete: false,
            force_delegate_get: true,
            force_delegate_post: false,
            force_delegate_put: false,
            force_delegate_delete: false,
            can_access_for_children_get: false,
            can_access_for_children_post: false,
            can_access_for_children_put: false,
            can_access_for_children_delete: false,
            created: None,
            modified: None,
        }
    );

    paths.push(
        Path {
            id: None,
            path: "^(/role)$".to_string(),
            readable_path: "/role".to_string(),
            get_available: true,
            post_available: false,
            put_available: false,
            delete_available: false,
            can_delegate_get: true,
            can_delegate_post: false,
            can_delegate_put: false,
            can_delegate_delete: false,
            force_delegate_get: true,
            force_delegate_post: false,
            force_delegate_put: false,
            force_delegate_delete: false,
            can_access_for_children_get: false,
            can_access_for_children_post: false,
            can_access_for_children_put: false,
            can_access_for_children_delete: false,
            created: None,
            modified: None,
        }
    );

    paths.push(
        Path {
            id: None,
            path: "^(/roles)/?.*$".to_string(),
            readable_path: "/roles".to_string(),
            get_available: true,
            post_available: false,
            put_available: false,
            delete_available: false,
            can_delegate_get: true,
            can_delegate_post: false,
            can_delegate_put: false,
            can_delegate_delete: false,
            force_delegate_get: true,
            force_delegate_post: false,
            force_delegate_put: false,
            force_delegate_delete: false,
            can_access_for_children_get: false,
            can_access_for_children_post: false,
            can_access_for_children_put: false,
            can_access_for_children_delete: false,
            created: None,
            modified: None,
        }
    );

    paths.push(
        Path {
            id: None,
            path: "^(/me)$".to_string(),
            readable_path: "/me".to_string(),
            get_available: true,
            post_available: false,
            put_available: false,
            delete_available: false,
            can_delegate_get: true,
            can_delegate_post: false,
            can_delegate_put: false,
            can_delegate_delete: false,
            force_delegate_get: true,
            force_delegate_post: false,
            force_delegate_put: false,
            force_delegate_delete: false,
            can_access_for_children_get: false,
            can_access_for_children_post: false,
            can_access_for_children_put: false,
            can_access_for_children_delete: false,
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
    ]
}