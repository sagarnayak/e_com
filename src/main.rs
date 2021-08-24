#[macro_use]
extern crate rocket;

use crate::config_controller::ConfigData;
use crate::core::rocket_master::rocket;
use crate::database::database_master::may_execute_migrations;

mod migrations;
mod database;
mod utils;
mod config_controller;
mod controllers;
mod core;
mod jwt_master;
mod contracts;
mod responder;
mod model;
mod guards;

#[launch]
async fn init_main() -> _ {
    let config_data = ConfigData::new();
    may_execute_migrations(config_data.clone()).await;
    rocket(config_data.clone())
}