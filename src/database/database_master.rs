use deadpool_postgres::{Client, Config, ManagerConfig, Pool, PoolError, RecyclingMethod};
use tokio_postgres::NoTls;
use uuid::Uuid;

use crate::config_controller::ConfigData;
use crate::database::database_master;
use crate::database::db_pool::DbPool;
use crate::migrations::migration_contracts::MigrationContracts;
use crate::migrations::migrations::MigrationStruct;
use crate::migrations::seeder::{enter_seed_data_to_auth_roles_cross_paths, enter_seed_data_to_paths, enter_seed_data_to_roles, enter_seed_data_to_users};

fn get_pool() -> Pool {
    let config = ConfigData::new();
    let database = config.database;

    let host = database.host;
    let port = database.port;
    let user = database.user;
    let password = database.password;
    let database_name = database.database_name;

    println!("host - {} post - {} user - {} password - {} db_name - {}", &host, &port, &user, &password, &database_name);

    let mut cfg = Config::new();
    cfg.host = Some(host);
    cfg.port = Some(port);
    cfg.user = Some(user);
    cfg.password = Some(password);
    cfg.dbname = Some(database_name);
    cfg.manager = Some(ManagerConfig { recycling_method: RecyclingMethod::Fast });
    cfg.create_pool(NoTls).unwrap()
}

pub fn get_db_pools() -> DbPool {
    DbPool {
        pool: get_pool()
    }
}

pub async fn resolve_client(db_pool: &DbPool) -> Client {
    let client: Result<Client, PoolError> = db_pool.pool.get().await;

    let client: Client = match client {
        Ok(client_positive) => client_positive,
        Err(pool_error) => {
            println!("we are getting an error {}", pool_error);
            panic!();
        }
    };

    client
}

pub async fn may_execute_migrations() {
    let db_pool = database_master::get_db_pools();
    match MigrationStruct::may_create_table_rows_count_table(&db_pool).await {
        Ok(_) => {
            println!("may create table rows count completed.");
        }
        Err(error) => println!("table rows count creation error error is {:?}", error),
    }
    match MigrationStruct::may_create_paths_table(&db_pool).await {
        Ok(_) => {
            println!("may create table paths completed.");
            enter_seed_data_to_paths(&db_pool).await;
        }
        Err(error) => println!("role table creation error error is {:?}", error),
    }
    let my_uuid = Uuid::new_v4();
    match MigrationStruct::may_create_roles_table(&db_pool).await {
        Ok(_) => {
            println!("may create table roles completed.");
            enter_seed_data_to_roles(&db_pool, &my_uuid).await;
        }
        Err(error) => println!("role table creation error error is {:?}", error),
    }
    match MigrationStruct::may_create_auth_roles_cross_paths_table(&db_pool).await {
        Ok(_) => {
            println!("may create table auth_roles_cross_paths completed.");
            enter_seed_data_to_auth_roles_cross_paths(&db_pool).await;
        }
        Err(error) => println!("auth_roles_cross_paths table creation error error is {:?}", error),
    }
    match MigrationStruct::may_create_mobile_numbers_table(&db_pool).await {
        Ok(_) => {
            println!("may create table mobile_numbers completed.");
        }
        Err(error) => println!("mobile_numbers table creation error error is {:?}", error),
    }
    match MigrationStruct::may_create_users_table(&db_pool).await {
        Ok(_) => {
            println!("may create table users completed.");
            enter_seed_data_to_users(&db_pool, &my_uuid).await;
        }
        Err(error) => println!("user table creation error error is {:?}", error),
    }
    match MigrationStruct::may_create_expired_blocked_tokens_table(&db_pool).await {
        Ok(_) => {
            println!("may create table expired_blocked_tokens completed.");
        }
        Err(error) => println!("expired_blocked_tokens table creation error error is {:?}", error),
    }
    match MigrationStruct::may_create_authorization_exceptions_table(&db_pool).await {
        Ok(_) => {
            println!("may create table authorization_exceptions completed.");
        }
        Err(error) => println!("authorization_exceptions table creation error error is {:?}", error),
    }
    match MigrationStruct::may_create_blocked_for_platform_authorization_table(&db_pool).await {
        Ok(_) => {
            println!("may create table blocked_for_platform_authorization completed.");
        }
        Err(error) => println!("blocked_for_platform_authorization table creation error error is {:?}", error),
    }
}