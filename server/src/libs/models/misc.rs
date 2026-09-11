#[derive(Debug)]
pub struct AuthenticatedUser {
    pub id: String,
    pub session_id: i64,
}
