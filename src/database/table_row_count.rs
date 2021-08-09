use chrono::{DateTime, Utc};
use postgres::Row;
use rocket::State;
use uuid::Uuid;

use crate::contracts::table_rows_count_contracts::TableRowsCountContracts;
use crate::database::database_master::resolve_client;
use crate::database::db_pool::DbPool;
use crate::model::status_message::StatusMessage;
use crate::model::table_rows_count::TableRowsCount;

impl TableRowsCount {
    async fn convert_results_to_models(results: &Vec<Row>) -> Result<Vec<TableRowsCount>, StatusMessage> {
        let mut results_to_send: Vec<TableRowsCount> = vec![];

        for row in results {
            let id: Uuid = match row.try_get(0) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => return StatusMessage::bad_request_400_in_result("failed to get id ".to_string()),
                },
                Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
            };
            let table_name: String = match row.try_get(1) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => return StatusMessage::bad_request_400_in_result("failed to get table_name ".to_string()),
                },
                Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
            };
            let rows: i32 = match row.try_get(2) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => return StatusMessage::bad_request_400_in_result("failed to get rows ".to_string()),
                },
                Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
            };
            let where_condition: Option<String> = match row.try_get(3) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => None,
                },
                Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
            };
            let created: DateTime<Utc> = match row.try_get(4) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => return StatusMessage::bad_request_400_in_result("failed to get enabled ".to_string()),
                },
                Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
            };
            let modified: Option<DateTime<Utc>> = match row.try_get(5) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => None,
                },
                Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
            };

            let parsed_rows = rows as u32;

            let res = TableRowsCount {
                id: id.to_hyphenated().to_string(),
                table_name,
                rows: parsed_rows,
                where_condition,
                created,
                modified,
            };

            results_to_send.push(res);
        }

        Ok(results_to_send)
    }
}

#[async_trait]
impl TableRowsCountContracts for TableRowsCount {
    async fn insert_count(
        table_name: &str,
        where_condition: &Option<String>,
        db_pool: &State<DbPool>,
    ) -> Result<u32, StatusMessage> {
        let client = resolve_client(db_pool).await;

        let mut statement_to_send = format!("SELECT count(*) FROM {}", &table_name);

        match where_condition.clone() {
            Some(positive) => {
                statement_to_send.push_str(
                    &format!(" WHERE {}", &positive)
                );
            }
            None => {}
        };

        let statement = match client
            .prepare_cached(&statement_to_send)
            .await {
            Ok(statement_positive) => statement_positive,
            Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
        };

        let results = match client.query(&statement, &[]).await {
            Ok(result_positive) => {
                let count_row = match result_positive.get(0) {
                    Some(positive_inner) => {
                        positive_inner
                    }
                    None => {
                        return StatusMessage::bad_request_400_in_result("Can not find the length".to_string());
                    }
                };

                let count: i64 = match count_row.try_get(0) {
                    Ok(count_positive) => count_positive,
                    Err(error) => {
                        return StatusMessage::bad_request_400_in_result(error.to_string());
                    }
                };

                count
            }
            Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
        };

        let statement_to_send = {
            let mut columns_string = "INSERT INTO table_rows_counts (table_name,rows".to_owned();
            let mut values_string = format!("VALUES ('{}',{}", &table_name, &results);
            match where_condition.clone() {
                Some(positive) => {
                    columns_string.push_str(
                        &format!(",where_condition")
                    );
                    let positive = positive.replace("'", "''");
                    values_string.push_str(
                        &format!(",'{}'", &positive)
                    );
                }
                None => {}
            }
            columns_string.push_str(
                &format!(")")
            );
            values_string.push_str(
                &format!(")")
            );
            let initial = format!(
                "{} {}",
                &columns_string,
                &values_string
            );
            initial
        };

        let statement = match client
            .prepare_cached(&statement_to_send)
            .await {
            Ok(statement_positive) => statement_positive,
            Err(error) => {
                return StatusMessage::bad_request_400_in_result(error.to_string());
            }
        };

        let results = match client.execute(&statement, &[]).await {
            Ok(result_positive) => result_positive,
            Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
        };

        Ok(
            results as u32
        )
    }

    async fn find_count(
        table_name: &str,
        where_condition: &Option<String>,
        db_pool: &State<DbPool>,
    ) -> Result<u32, StatusMessage> {
        let client = resolve_client(db_pool).await;
        let mut statement_to_send = format!(
            "SELECT * from table_rows_counts where table_name = '{}'",
            &table_name
        );
        match where_condition.clone() {
            Some(positive) => {
                let positive = positive.replace("'", "''");
                statement_to_send.push_str(
                    &format!(" AND where_condition = '{}'", &positive)
                )
            }
            None => {}
        }

        let statement = match client
            .prepare_cached(&statement_to_send)
            .await {
            Ok(statement_positive) => statement_positive,
            Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
        };

        let results = match client.query(&statement, &[]).await {
            Ok(result_positive) => result_positive,
            Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
        };

        let results_to_send = match TableRowsCount::convert_results_to_models(&results).await {
            Ok(positive) => {
                positive
            }
            Err(error) => {
                return Err(error);
            }
        };

        if results_to_send.len() != 0 {
            Ok(
                results_to_send.get(0).unwrap().rows
            )
        } else {
            match TableRowsCount::insert_count(
                &table_name,
                &where_condition,
                &db_pool,
            ).await {
                Ok(_) => {
                    match TableRowsCount::find_count(
                        &table_name,
                        &where_condition,
                        &db_pool,
                    ).await {
                        Ok(positive_inner) => {
                            Ok(
                                positive_inner
                            )
                        }
                        Err(_) => {
                            StatusMessage::bad_request_400_in_result(
                                format!("2 can not get count for {}", &table_name)
                            )
                        }
                    }
                }
                Err(_) => {
                    StatusMessage::bad_request_400_in_result(
                        format!("1 can not insert new count for {}", &table_name)
                    )
                }
            }
        }
    }
    async fn clear_data_for_table(table_name: &str, db_pool: &State<DbPool>) -> Result<bool, StatusMessage> {
        let client = resolve_client(db_pool).await;
        let statement_to_send = format!(
            "DELETE * from table_rows_counts where table_name = '{}'",
            &table_name
        );

        let statement = match client
            .prepare_cached(&statement_to_send)
            .await {
            Ok(statement_positive) => statement_positive,
            Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
        };

        let _ = match client.execute(&statement, &[]).await {
            Ok(result_positive) => result_positive,
            Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
        };

        Ok(true)
    }
}