use rocket::{get, routes, Build, Rocket};

pub fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![index])
}

#[get("/")]
fn index() -> &'static str {
    "Hello, Rocket!"
}
