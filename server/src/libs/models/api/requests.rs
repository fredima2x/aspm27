use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateChatRequest {
    pub chat_name: String,
}

#[derive(Deserialize)]
pub struct GetChatMessagesRequest {
    pub limit: i64,
    pub offset: i64,
}

#[derive(Deserialize)]
pub struct SendMessageRequest {
    pub content: String,
}

#[derive(Deserialize)]
pub struct SendUserRequest {
    pub username: String,
    pub password: String,
    pub display_name: String,
}

#[derive(Deserialize)]
pub struct SimpleSendUserRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UpdateProfileRequest {
    pub user: SendUserRequest,
}

#[derive(Deserialize)]
pub struct UpdateChatRequest {
    pub chat_name: String,
    pub chat_desc: String,
}
