use uuid::Uuid;

#[allow(dead_code)]
#[derive(Debug)]
pub struct AuthenticatedUser {
    pub id: Uuid,
    pub session_id: i64,
}
