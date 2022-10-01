use rocket::serde::json::Json;

use crate::utils::auth::{hash, is_user_registered};

#[get("/login?<email>&<password>")]
pub fn login(email: &str, password: &str) -> Json<String> {
    let token = hash(email.to_string(), password.to_string());

    if is_user_registered(token.as_str()) {
        return Json(token);
    }
    Json("".to_string())
}
