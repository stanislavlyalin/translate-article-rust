#[macro_use]
extern crate rocket;

fn hash(email: String, password: String) -> String {
    let digest = md5::compute(email + password.as_str() + "my_awesome_service");
    return format!("{:x}", digest);
}

#[get("/login?<email>&<password>")]
fn login(email: &str, password: &str) -> String {
    println!("{}", hash(email.to_string(), password.to_string()));
    format!("{} {}", email, password)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api/v1", routes![login])
}
