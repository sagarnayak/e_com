use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PageResponse<T> {
    pub data: Vec<T>,
    pub previous_page_available: bool,
    pub next_page_available: bool,
    pub page_size: u32,
    pub page_number: u32,
    pub total_items: u32,
}