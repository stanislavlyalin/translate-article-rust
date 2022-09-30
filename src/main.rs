#[macro_use]
extern crate rocket;

#[get("/login?<email>&<password>")]
fn login(email: &str, password: &str) -> String {
    format!("{} {}", email, password)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api/v1", routes![login])
}
