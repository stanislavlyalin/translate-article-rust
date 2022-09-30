#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;

fn hash(email: String, password: String) -> String {
    let digest = md5::compute(email + password.as_str() + "my_awesome_service");
    return format!("{:x}", digest);
}

#[get("/login?<email>&<password>")]
fn login(email: &str, password: &str) -> Json<String> {
    let token = hash(email.to_string(), password.to_string());
    println!("{}", token);
    Json(token)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api/v1", routes![login])
}
