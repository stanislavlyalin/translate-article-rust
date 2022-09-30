#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;
use std::fs::File;
use std::io::{self, BufRead, Error};
use std::path::Path;

fn hash(email: String, password: String) -> String {
    let salt = "my_awesome_service";
    let digest = md5::compute(email + password.as_str() + salt);
    format!("{:x}", digest)
}

fn is_user_registered(token: &str) -> bool {
    fn file_contains(filepath: &str, token: &str) -> Result<bool, Error> {
        if Path::new(filepath).exists() {
            let file = File::open(filepath)?;
            let lines = io::BufReader::new(file).lines();
            return Ok(lines.filter(|l| l.as_ref().unwrap().eq(token)).count() > 0);
        }
        Ok(false)
    }

    return match file_contains("user_hashes.txt", token) {
        Ok(r) => r,
        Err(_) => false,
    };
}

#[get("/login?<email>&<password>")]
fn login(email: &str, password: &str) -> Json<String> {
    let token = hash(email.to_string(), password.to_string());

    if is_user_registered(token.as_str()) {
        return Json(token);
    }
    Json("".to_string())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api/v1", routes![login])
}
