#[macro_use]
extern crate rocket;

mod utils;
mod views;

use crate::views::login::login;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api/v1", routes![login])
}
