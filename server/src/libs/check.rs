pub async fn check_username(username: &str) -> bool {
    username.len() >= 3 && username.len() <= 24
}
pub async fn check_display_name(display_name: &str) -> bool {
    display_name.len() >= 2 && display_name.len() <= 32
}
pub async fn check_password(password: &str) -> bool {
    password.len() >= 8 && password.len() <= 64
}
pub async fn check_chat_name(chat_name: &str) -> bool {
    chat_name.len() >= 3 && chat_name.len() <= 32
}
pub async fn check_chat_desc(chat_desc: &str) -> bool {
    chat_desc.len() <= 256
}
