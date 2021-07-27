use crate::database::db_pool::DbPool;
use uuid::Uuid;

pub async fn enter_seed_data_to_roles(db_pool: &DbPool, uuid: &Uuid) {
    let client = match db_pool.pool.get().await {
        Ok(client_positive) => client_positive,
        Err(error) => {
            println!("failed to insert seed data to DB :: {}", error.to_string());
            panic!();
        }
    };

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
                uuid
            )
        )
        .await {
        Ok(statement_positive) => statement_positive,
        Err(error) => panic!(),
    };

    let results = match client.execute(
        &statement,
        &[],
    ).await {
        Ok(positive) => positive,
        Err(error) => panic!()
    };
}

pub async fn enter_seed_data_to_users(db_pool: &DbPool) {
    let client = match db_pool.pool.get().await {
        Ok(client_positive) => client_positive,
        Err(error) => {
            println!("failed to insert seed data to DB :: {}", error.to_string());
            panic!();
        }
    };

    let statement = match client
        .prepare_cached(
            &format!(
                "INSERT INTO users (\
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
                'admin',\
                true,\
                '*',\
                true,\
                true,\
                true,\
                true,\
                false,\
                true\
                )"
            )
        )
        .await {
        Ok(statement_positive) => statement_positive,
        Err(error) => panic!(),
    };

    let results = match client.execute(
        &statement,
        &[],
    ).await {
        Ok(positive) => positive,
        Err(error) => panic!()
    };
}