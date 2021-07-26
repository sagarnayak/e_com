#[macro_use]
extern crate rocket;


use crate::core::rocket_master::rocket;
use crate::database::database_master::may_execute_migrations;

mod migrations;
mod database;
mod config_controller;
mod controllers;
mod core;
mod jwt_master;
mod contracts;
mod model;

#[launch]
async fn init_main() -> _ {
    may_execute_migrations().await;
    rocket()
}