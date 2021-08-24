use chrono::{DateTime, Utc};
use rocket::State;
use uuid::Uuid;

use crate::contracts::mobile_number_contracts::MobileNumberContracts;
use crate::database::database_master::resolve_client;
use crate::database::db_pool::DbPool;
use crate::model::mobile_number::MobileNumber;
use crate::model::status_message::StatusMessage;
use postgres::Row;

impl MobileNumber {
    async fn convert_results_to_models(results: &Vec<Row>) -> Result<Vec<MobileNumber>, StatusMessage> {
        let mut results_to_send: Vec<MobileNumber> = vec![];

        for row in results {
            let id: Uuid = match row.try_get(0) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => return StatusMessage::bad_request_400_in_result("failed to get id ".to_string()),
                },
                Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
            };
            let country_code: String = match row.try_get(1) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => return StatusMessage::bad_request_400_in_result("failed to get country_code ".to_string()),
                },
                Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
            };
            let number: String = match row.try_get(2) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => return StatusMessage::bad_request_400_in_result("failed to get number ".to_string()),
                },
                Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
            };
            let created: DateTime<Utc> = match row.try_get(3) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => return StatusMessage::bad_request_400_in_result("failed to get created ".to_string()),
                },
                Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
            };
            let modified: Option<DateTime<Utc>> = match row.try_get(4) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => None,
                },
                Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
            };

            let mobile_number = MobileNumber {
                id: id.to_hyphenated().to_string(),
                country_code,
                number,
                created,
                modified,
            };

            results_to_send.push(mobile_number);
        }

        Ok(results_to_send)
    }
}

#[async_trait]
impl MobileNumberContracts for MobileNumber {
    async fn find_mobile_number_with_id(mobile_number_id: &str, db_pool: &State<DbPool>) -> Result<MobileNumber, StatusMessage> {
        let client = resolve_client(db_pool).await;

        let statement_to_send = &format!("SELECT * FROM mobile_numbers WHERE id = '{}'", mobile_number_id);

        let statement = match client
            .prepare_cached(statement_to_send)
            .await {
            Ok(statement_positive) => statement_positive,
            Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
        };

        let results = match client.query(&statement, &[]).await {
            Ok(result_positive) => result_positive,
            Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
        };

        let results_to_send = match MobileNumber::convert_results_to_models(&results).await {
            Ok(positive) => {
                positive
            }
            Err(error) => {
                return Err(error);
            }
        };

        if results_to_send.len() != 0 {
            let res_to_return = results_to_send[0].clone();
            Ok(
                res_to_return
            )
        } else {
            StatusMessage::bad_request_400_in_result(
                "Failed to get mobile_number".to_owned()
            )
        }
    }
}