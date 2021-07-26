use rocket::{Build, Rocket};

use crate::controllers::routes;
use crate::core::fairings::CounterFairing;
use crate::database::database_master;

pub fn rocket() -> Rocket<Build> {
    rocket::build()
        .attach(CounterFairing::default())
        // .register("/", catchers![not_found])
        .mount("/", routes::get_routes())
        .manage(database_master::get_db_pools())
}