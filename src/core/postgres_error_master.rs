use postgres::error::DbError;

pub fn get_postgres_error_string(db_error: Option<&DbError>) -> String {
    return match db_error {
        Some(positive) => {
            parse_description(positive)
        }
        None => "Database error.".to_owned()
    };
}

fn parse_description(db_error: &DbError) -> String {
    match db_error.detail() {
        Some(positive) => {
            positive.to_owned()
        }
        None => {
            db_error.message().to_owned()
        }
    }
}