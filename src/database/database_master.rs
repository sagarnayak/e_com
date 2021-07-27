use deadpool_postgres::{Client, Config, ManagerConfig, Pool, PoolError, RecyclingMethod};
use rocket::serde::json::Json;
use rocket::State;
use tokio_postgres::NoTls;

use crate::config_controller::ConfigData;
use crate::database::database_master;
use crate::database::db_pool::DbPool;
use crate::migrations::migration_contracts::MigrationContracts;
use crate::migrations::migrations::MigrationStruct;
use crate::model::status_message::StatusMessage;

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

// pub async fn resolve_client(db_pool: &State<DbPool>) -> Result<Client, Json<StatusMessage>> {
//     let client: Result<Client, PoolError> = db_pool.pool.get().await;
//
//     let client: Client = match client {
//         Ok(client_positive) => client_positive,
//         Err(pool_error) => {
//             println!("we are getting an error {}", pool_error);
//             return Err(
//                 Json(
//                     StatusMessage {
//                         code: 400,
//                         message: pool_error.to_string(),
//                     }
//                 )
//             );
//         }
//     };
//
//     Ok(client)
// }

pub async fn may_execute_migrations() {
    let db_pool = database_master::get_db_pools();
    match MigrationStruct::may_create_users_table(&db_pool).await {
        Ok(positive) => println!("user table result : {:?}", positive),
        Err(error) => println!("user table error error is {:?}", error),
    }
    match MigrationStruct::may_create_roles_table(&db_pool).await {
        Ok(positive) => {
            println!("role table result : {:?}", positive);
            enter_seed_data_to_roles(&db_pool).await;
        }
        Err(error) => println!("role table error error is {:?}", error),
    }
}

async fn enter_seed_data_to_roles(db_pool: &DbPool) {
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