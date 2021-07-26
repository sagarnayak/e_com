use rocket::serde::json::Json;
use rocket::State;

use crate::contracts::user_contracts::UserContracts;
use crate::database::database_master::resolve_client;
use crate::database::db_pool::DbPool;
use crate::model::status_message::StatusMessage;
use crate::model::user::TABLE_NAME_USER;
use crate::model::user::User;

// #[async_trait]
// impl UserContracts for User {
//     async fn fetch_all_user(db_pool: &State<DbPool>) -> Result<Json<Vec<User>>, Json<StatusMessage>> {
//         let client = match resolve_client(db_pool).await {
//             Ok(client_positive) => client_positive,
//             Err(error) => return Err(error),
//         };
//
//         let statement = match client
//             .prepare_cached(&format!("SELECT * FROM {}", TABLE_NAME_USER))
//             .await {
//             Ok(statement_positive) => statement_positive,
//             Err(error) => return Err(Json(StatusMessage::bad_req(error.to_string()))),
//         };
//
//         let result_for_users = match client.query(&statement, &[]).await {
//             Ok(result_positive) => result_positive,
//             Err(error) => return Err(Json(StatusMessage::bad_req(error.to_string()))),
//         };
//
//         let mut users = vec![];
//
//         for row in result_for_users {
//             let id: i64 = match row.try_get(0) {
//                 Ok(additional_field_positive) => match additional_field_positive {
//                     Some(additional_field_positive_inner) => additional_field_positive_inner,
//                     None => return Err(Json(StatusMessage::bad_req("failed to get id ".to_string().to_string()))),
//                 },
//                 Err(error) => return Err(Json(StatusMessage::bad_req(error.to_string()))),
//             };
//             let name: String = match row.try_get(1) {
//                 Ok(additional_field_positive) => match additional_field_positive {
//                     Some(additional_field_positive_inner) => additional_field_positive_inner,
//                     None => return Err(Json(StatusMessage::bad_req("failed to get name ".to_string().to_string()))),
//                 },
//                 Err(error) => return Err(Json(StatusMessage::bad_req(error.to_string()))),
//             };
//             let email: String = match row.try_get(2) {
//                 Ok(additional_field_positive) => match additional_field_positive {
//                     Some(additional_field_positive_inner) => additional_field_positive_inner,
//                     None => return Err(Json(StatusMessage::bad_req("failed to get email ".to_string().to_string()))),
//                 },
//                 Err(error) => return Err(Json(StatusMessage::bad_req(error.to_string()))),
//             };
//
//             let user = User {
//                 id,
//                 name,
//                 email_id: email,
//             };
//
//             users.push(user);
//         }
//
//         Ok(
//             Json(
//                 users
//             )
//         )
//     }
// }