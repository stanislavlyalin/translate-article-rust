use rocket::serde::json::Json;

use crate::utils::auth::{hash, is_user_registered};

#[get("/login?<email>&<password>")]
pub fn login(email: &str, password: &str) -> Json<String> {
    let token = hash(email, password);

    if is_user_registered(&token) {
        return Json(token);
    }
    Json("".to_string())
}
