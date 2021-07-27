use bcrypt::hash;
use uuid::Uuid;

use crate::config_controller::ConfigData;
use crate::core::constants::B_CRYPT_COST;
use crate::database::database_master::resolve_client;
use crate::database::db_pool::DbPool;

pub async fn enter_seed_data_to_roles(db_pool: &DbPool, id: &Uuid) {
    let client = resolve_client(db_pool).await;

    let statement = match client.prepare_cached(
        &format!(
            "SELECT * FROM roles LIMIT 1"
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

    println!("the result is ::: {:?}", result.len());

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
}

pub async fn enter_seed_data_to_users(db_pool: &DbPool, role_id: &Uuid) {
    let client = resolve_client(db_pool).await;
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
            println!("got an error while seeding user table : {}", error.to_string());
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
}