#[get("/")]
pub async fn index() -> String {
    "Hi from Rust".to_owned()
}