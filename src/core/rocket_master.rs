use rocket::{Build, Rocket};

use crate::config_controller::ConfigData;
use crate::controllers::catchers::for_404;
use crate::controllers::catchers::not_found;
use crate::controllers::routes;
use crate::core::fairings::CounterFairing;
use crate::database::database_master;

pub fn rocket(config_data: ConfigData) -> Rocket<Build> {
    rocket::build()
        .attach(CounterFairing::default())
        .register("/", catchers![for_404,not_found])
        .mount("/", routes::get_routes())
        .manage(database_master::get_db_pools(config_data.clone()))
        .manage(config_data.clone())
}