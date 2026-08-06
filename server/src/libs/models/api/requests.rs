use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CreateChatRequest {
    pub chat_name: String,
}

#[derive(Deserialize, Debug)]
pub struct GetChatMessagesRequest {
    pub limit: i64,
    pub offset: i64,
}

#[derive(Deserialize, Debug)]
pub struct SendMessageRequest {
    pub content: String,
}

#[derive(Deserialize, Debug)]
pub struct SendUserRequest {
    pub username: String,
    pub password: String,
    pub display_name: String,
}

#[derive(Deserialize, Debug)]
pub struct SimpleSendUserRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct UpdateProfileRequest {
    pub user: SendUserRequest,
}

#[derive(Deserialize, Debug)]
pub struct UpdateChatRequest {
    pub chat_name: String,
    pub chat_desc: String,
}
