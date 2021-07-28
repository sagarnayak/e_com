use deadpool_postgres::{Client, Config, ManagerConfig, Pool, PoolError, RecyclingMethod};
use tokio_postgres::NoTls;
use uuid::Uuid;

use crate::config_controller::ConfigData;
use crate::database::database_master;
use crate::database::db_pool::DbPool;
use crate::migrations::migration_contracts::MigrationContracts;
use crate::migrations::migrations::MigrationStruct;
use crate::migrations::seeder::{enter_seed_data_to_roles, enter_seed_data_to_users};

fn get_pool() -> Pool {
    let config = ConfigData::new();
    let database = config.database;

    let host = database.host;
    let port = database.port;
    let user = database.user;
    let password = database.password;
    let database_name = database.database_name;

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
    let my_uuid = Uuid::new_v4();
    match MigrationStruct::may_create_roles_table(&db_pool).await {
        Ok(positive) => {
            println!("may create table roles completed.");
            enter_seed_data_to_roles(&db_pool, &my_uuid).await;
        }
        Err(error) => println!("role table creation error error is {:?}", error),
    }
    match MigrationStruct::may_create_users_table(&db_pool).await {
        Ok(positive) => {
            println!("may create table users completed.");
            enter_seed_data_to_users(&db_pool, &my_uuid).await;
        }
        Err(error) => println!("user table creation error error is {:?}", error),
    }
}