use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TableRowsCount {
    pub id: String,
    pub table_name: String,
    pub rows: u32,
    pub where_condition: Option<String>,
    pub created: DateTime<Utc>,
    pub modified: Option<DateTime<Utc>>,
}