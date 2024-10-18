use crate::models::user::User;
use serde_json::Error as SerdeError;

pub fn get_id(request: &str) -> &str {
    request.split("/").nth(2).unwrap_or_default().split_whitespace().next().unwrap_or_default()
}

pub fn get_user_request_body(request: &str) -> Result<User, SerdeError> {
    serde_json::from_str(request.split("\r\n\r\n").last().unwrap_or_default())
}
