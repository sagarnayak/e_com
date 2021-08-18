use bcrypt::hash;
use uuid::Uuid;

use crate::config_controller::ConfigData;
use crate::contracts::path_contracts::PathContracts;
use crate::contracts::role_contracts::RoleContracts;
use crate::controllers::routes::get_paths;
use crate::core::constants::B_CRYPT_COST;
use crate::database::database_master::resolve_client;
use crate::database::db_pool::DbPool;
use crate::model::path::Path;
use crate::model::role::Role;

async fn should_proceed_inserting_seed_data(db_pool: &DbPool, table_name: &str, min_threshold_to_skip: usize) -> bool {
    let client = resolve_client(db_pool).await;

    let statement = match client.prepare_cached(
        &format!(
            "SELECT * FROM {} LIMIT {}",
            table_name,
            &min_threshold_to_skip
        )
    ).await {
        Ok(positive) => positive,
        Err(_) => panic!()
    };

    let result = match client.query(
        &statement,
        &[],
    ).await {
        Ok(positive) => {
            positive
        }
        Err(_) => panic!()
    };

    result.len() < min_threshold_to_skip
}

pub async fn enter_seed_data_to_paths(db_pool: &DbPool) {
    let client = resolve_client(db_pool).await;

    println!("trying to insert seed data to paths");

    let paths = get_paths();

    if !should_proceed_inserting_seed_data(db_pool, "paths", paths.len()).await {
        println!("rejected inserting seed data to paths");
        return;
    }

    let statement = &format!(
        "DELETE FROM paths"
    );

    let statement = match client
        .prepare_cached(statement)
        .await {
        Ok(statement_positive) => statement_positive,
        Err(error) => {
            println!("error  ::: {}", error.to_string());
            panic!();
        }
    };

    let _ = match client.execute(
        &statement,
        &[],
    ).await {
        Ok(positive) => positive,
        Err(_) => panic!()
    };

    let mut values_string: String = " VALUES ".to_owned();

    for path in get_paths() {
        let value = format!(
            " ('{}','{}',{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}) ",
            path.path,
            path.readable_path,
            path.get_available,
            path.post_available,
            path.put_available,
            path.delete_available,
            path.can_delegate_get,
            path.can_delegate_post,
            path.can_delegate_put,
            path.can_delegate_delete,
            path.force_delegate_get,
            path.force_delegate_post,
            path.force_delegate_put,
            path.force_delegate_delete,
            path.can_access_for_children_get,
            path.can_access_for_children_post,
            path.can_access_for_children_put,
            path.can_access_for_children_delete,
        );
        if values_string == " VALUES " {
            values_string.push_str(&value);
        } else {
            values_string.push_str(",");
            values_string.push_str(&value);
        }
    }

    let statement = &format!(
        "INSERT INTO paths (\
                path,\
                readable_path,\
                get_available,\
                post_available,\
                put_available,\
                delete_available,\
                can_delegate_get,\
                can_delegate_post,\
                can_delegate_put,\
                can_delegate_delete,\
                force_delegate_get,\
                force_delegate_post,\
                force_delegate_put,\
                force_delegate_delete,\
                can_access_for_children_get,\
                can_access_for_children_post,\
                can_access_for_children_put,\
                can_access_for_children_delete\
                ) \
                {}",
        values_string
    );

    let statement = match client
        .prepare_cached(statement)
        .await {
        Ok(statement_positive) => statement_positive,
        Err(error) => {
            println!("error  ::: {}", error.to_string());
            panic!();
        }
    };

    let _ = match client.execute(
        &statement,
        &[],
    ).await {
        Ok(positive) => positive,
        Err(_) => panic!()
    };

    println!("seed data inserted to paths");
}

pub async fn enter_seed_data_to_roles(db_pool: &DbPool, id: &Uuid) {
    let client = resolve_client(db_pool).await;

    println!("trying to insert seed data to roles");

    if !should_proceed_inserting_seed_data(db_pool, "roles", 1).await {
        println!("rejected inserting seed data to roles");
        return;
    }

    let statement = match client
        .prepare_cached(
            &format!(
                "INSERT INTO roles (\
                id,\
                name,\
                can_delegate,\
                can_access_for_children,\
                enabled\
                ) \
                VALUES (\
                '{}',\
                'admin',\
                true,\
                true,\
                true\
                )",
                id
            )
        )
        .await {
        Ok(statement_positive) => statement_positive,
        Err(_) => panic!(),
    };

    let _ = match client.execute(
        &statement,
        &[],
    ).await {
        Ok(positive) => positive,
        Err(_) => panic!()
    };

    println!("seed data inserted to roles");
}

pub async fn enter_seed_data_to_users(db_pool: &DbPool, role_id: &Uuid) {
    let client = resolve_client(db_pool).await;

    println!("trying to insert seed data to users");

    if !should_proceed_inserting_seed_data(db_pool, "users", 1).await {
        println!("rejected inserting seed data to users");
        return;
    }

    let config_data = ConfigData::new();

    let hashed = hash(config_data.admin_data.admin_password, B_CRYPT_COST);
    let hashed = match hashed {
        Ok(hashed_positive) => hashed_positive,
        Err(_) => panic!(),
    };

    let statement = match client
        .prepare_cached(
            &format!(
                "INSERT INTO users (\
                role,\
                password,\
                first_name,\
                email_id\
                ) \
                VALUES (\
                '{}',\
                '{}',\
                '{}',\
                '{}'\
                )",
                role_id,
                hashed,
                config_data.admin_data.admin_name,
                config_data.admin_data.admin_email
            )
        )
        .await {
        Ok(statement_positive) => statement_positive,
        Err(_) => {
            panic!();
        }
    };

    let _ = match client.execute(
        &statement,
        &[],
    ).await {
        Ok(positive) => positive,
        Err(_) => panic!()
    };

    println!("seed data inserted to users");
}

pub async fn enter_seed_data_to_auth_roles_cross_paths(db_pool: &DbPool) {
    let client = resolve_client(db_pool).await;

    println!("trying to insert seed data to auth_roles_cross_paths");

    let paths = get_paths();

    if !should_proceed_inserting_seed_data(db_pool, "auth_roles_cross_paths", paths.len()).await {
        println!("rejected inserting seed data to auth_roles_cross_paths");
        return;
    }

    let statement = &format!(
        "DELETE FROM auth_roles_cross_paths"
    );

    let statement = match client
        .prepare_cached(statement)
        .await {
        Ok(statement_positive) => statement_positive,
        Err(error) => {
            println!("error  ::: {}", error.to_string());
            panic!();
        }
    };

    let _ = match client.execute(
        &statement,
        &[],
    ).await {
        Ok(positive) => positive,
        Err(_) => panic!()
    };

    let admin_role = Role::find_role_for_admin(db_pool).await.unwrap();
    let paths = Path::fetch_all(db_pool).await.unwrap();

    let mut values_string: String = " VALUES ".to_owned();

    for path in paths {
        let value = format!(
            " ('{}','{}','{}',{},{},{},{},{},{},{},{},{},{},{},{}) ",
            admin_role.id,
            path.id.unwrap(),
            path.path,
            path.get_available,
            path.post_available,
            path.put_available,
            path.delete_available,
            path.get_available,
            path.post_available,
            path.put_available,
            path.delete_available,
            path.get_available,
            path.post_available,
            path.put_available,
            path.delete_available
        );
        if values_string == " VALUES " {
            values_string.push_str(&value);
        } else {
            values_string.push_str(",");
            values_string.push_str(&value);
        }
    }

    let statement = match client
        .prepare_cached(
            &format!(
                "INSERT INTO auth_roles_cross_paths (\
                auth_role,\
                path_id,\
                path,\
                get_allowed,\
                post_allowed,\
                put_allowed,\
                delete_allowed,\
                can_delegate_get,\
                can_delegate_post,\
                can_delegate_put,\
                can_delegate_delete,\
                can_access_for_children_get,\
                can_access_for_children_post,\
                can_access_for_children_put,\
                can_access_for_children_delete\
                ) \
                {}",
                values_string
            )
        )
        .await {
        Ok(statement_positive) => statement_positive,
        Err(error) => {
            println!("error  ::: {}", error.to_string());
            panic!();
        }
    };

    let _ = match client.execute(
        &statement,
        &[],
    ).await {
        Ok(positive) => positive,
        Err(_) => panic!()
    };

    println!("seed data inserted to auth_roles_cross_paths");
}