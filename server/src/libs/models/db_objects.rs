use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct DirectUser {
    pub id: i64,
    pub username: String,
    pub display_name: String,
    pub password_hash: String,
    pub soft_delete: bool,
    pub deleted_at: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct DirectChat {
    pub id: i64,
    pub chat_name: String,
    pub chat_desc: String,
    pub soft_delete: bool,
    pub deleted_at: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct DirectMessage {
    pub id: i64,
    pub owner_id: i64,
    pub chat_id: i64,
    pub content: String,
    pub soft_delete: bool,
    pub deleted_at: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct BasicUser {
    pub id: i64,
    pub username: String,
    pub display_name: String,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct BasicChat {
    pub id: i64,
    pub chat_name: String,
    pub chat_desc: String,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct BasicMessage {
    pub id: i64,
    pub owner_id: i64,
    pub chat_id: i64,
    pub content: String,
}

impl From<DirectUser> for BasicUser {
    fn from(user: DirectUser) -> Self {
        BasicUser {
            id: user.id,
            username: user.username,
            display_name: user.display_name,
        }
    }
}

impl From<DirectChat> for BasicChat {
    fn from(chat: DirectChat) -> Self {
        BasicChat {
            id: chat.id,
            chat_name: chat.chat_name,
            chat_desc: chat.chat_desc,
        }
    }
}

impl From<DirectMessage> for BasicMessage {
    fn from(message: DirectMessage) -> Self {
        BasicMessage {
            id: message.id,
            owner_id: message.owner_id,
            chat_id: message.chat_id,
            content: message.content,
        }
    }
}
