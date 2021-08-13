use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::State;

use crate::contracts::user_contracts::UserContracts;
use crate::database::db_pool::DbPool;
use crate::guards::authentication_authorization_guard::AuthenticationAuthorizationGuard;
use crate::model::status_message::StatusMessage;
use crate::model::user::User;

#[get("/redis")]
pub async fn redis() -> status::Custom<Result<String, String>> {
    /*let client = match redis::Client::open("rediss://connectskn.ddns.net/redis/") {
        Ok(positive) => {
            positive
        }
        Err(error) => {
            println!("got error :: {}", error);
            return status::Custom(
                Status::BadRequest,
                Err("failed".to_owned()),
            );
        }
    };*/
    // let mut con = client.get_async_connection().await?;
    // throw away the result, just make sure it does not fail
    // let _ : () = con.set("my_key", 42).unwrap();
    // read back the key and return it.  Because the return value
    // from the function is a result for integer this will automatically
    // convert into one.

    // con.get("my_key");
    status::Custom(
        Status::Ok,
        Ok(
            "redis".to_owned(),
        ),
    )
}