use std::thread::sleep;
use std::time::Duration;

use rand::Rng;
use rocket::http::Status;
use rocket::response::content;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::State;

use crate::database::db_pool::DbPool;
use crate::migrations::migration_contracts::MigrationContracts;
use crate::migrations::migrations::MigrationStruct;
use crate::model::status_message::StatusMessage;
use crate::model::user::User;

#[get("/")]
pub async fn index(db_pool: &State<DbPool>) -> &'static str {
    "done"
}

#[get("/world")]
pub fn world() -> &'static str {
    "hello, world!"
}

#[get("/mayGet")]
pub fn may_get() -> Result<String, String> {
    let result_to_return = rand::thread_rng().gen_bool(0.5);
    if result_to_return {
        Ok("result ...".to_string())
    } else {
        Err("error...".to_string())
    }
}

#[get("/blockingTask")]
pub async fn blocking_task() -> Result<String, String> {
    sleep(Duration::from_secs(rand::thread_rng().gen_range(2..6)));
    Ok("Done .".to_string())
}

#[get("/me/<name>")]
pub fn me(name: &str) -> String {
    let result_to_return = format!("hi, {}", name);
    result_to_return
}

#[get("/testNumberException/<number>")]
pub fn test_number_exception(number: Result<u8, &str>) -> String {
    match number {
        Ok(number_positive) => format!("success {}", number_positive),
        Err(error) => format!("error : {}", error),
    }
}

#[get("/getAccepted")]
pub fn get_accepted() -> status::Accepted<String> {
    status::Accepted(Some(format!("id:")))
}

#[get("/getStatusCode")]
pub fn get_status_code() -> status::Custom<content::Json<&'static str>> {
    status::Custom(Status::ImATeapot, content::Json("{ \"hi\": \"world\" }"))
}

#[get("/getStatusCodeTwo")]
pub fn get_status_code_dfvdfb() -> status::Custom<content::Json<User>> {
    status::Custom(Status::BadRequest, content::Json(User { id: 34, name: "sagar".to_string(), email_id: "sgar@gmail.com".to_string() }))
}

#[get("/getBlankArray")]
pub fn get_blank_array() -> status::Custom<rocket::serde::json::Json<Vec<User>>> {
    status::Custom(Status::Ok, Json(vec![
        User { id: 1, name: "test user".to_string(), email_id: "test@testing.com".to_string() },
        User { id: 2, name: "test user two".to_string(), email_id: "testtwo@testing.com".to_string() },
    ]))
}