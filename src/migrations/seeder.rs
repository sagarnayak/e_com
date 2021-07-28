use bcrypt::hash;
use uuid::Uuid;

use crate::config_controller::ConfigData;
use crate::core::constants::B_CRYPT_COST;
use crate::database::database_master::resolve_client;
use crate::database::db_pool::DbPool;

async fn should_proceed_inserting_seed_data(db_pool: &DbPool, table_name: &str) -> bool {
    let client = resolve_client(db_pool).await;

    let statement = match client.prepare_cached(
        &format!(
            "SELECT * FROM {} LIMIT 1",
            table_name
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

    result.len() == 0
}

pub async fn enter_seed_data_to_roles(db_pool: &DbPool, id: &Uuid) {
    let client = resolve_client(db_pool).await;

    println!("trying to insert seed data to roles");

    if !should_proceed_inserting_seed_data(db_pool, "roles").await {
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
                path,\
                read,\
                write,\
                edit,\
                delete,\
                identifier_required,\
                enabled\
                ) \
                VALUES (\
                '{}',\
                'admin',\
                true,\
                '*',\
                true,\
                true,\
                true,\
                true,\
                false,\
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

    if !should_proceed_inserting_seed_data(db_pool, "users").await {
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
                name,\
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
        Err(error) => {
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