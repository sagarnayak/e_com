#[macro_use]
extern crate rocket;


use crate::core::rocket_master::rocket;
use crate::database::database_master;
use crate::database::database_master::may_execute_migrations;
use crate::migrations::migration_contracts::MigrationContracts;

mod migrations;
mod database;
mod config_controller;
mod controllers;
mod core;
mod contracts;
mod model;

#[launch]
async fn init_main() -> _ {
    may_execute_migrations().await;
    rocket()
}