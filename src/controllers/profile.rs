#[get("/me")]
pub fn me() -> &'static str {
    "Hello!"
}