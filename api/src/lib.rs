#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]

//! Some crate documentation

pub(crate) mod db;
pub(crate) mod handlers;
pub(crate) mod services;
pub(crate) mod types;

use libservices::AttachableService;
use rocket::{get, routes, Build, Rocket};
use rocket_db_pools::Database;

use crate::{
    db::Primary,
    handlers::users::{create, get},
    services::users::UserService,
};

/// Constructs a `Rocket<Build>` instance
#[must_use]
pub fn rocket() -> Rocket<Build> {
    rocket::build()
        .attach(Primary::init())
        .attach(UserService::init())
        .mount("/", routes![index])
        .mount("/users", routes![create, get])
}

#[get("/")]
const fn index() -> &'static str {
    "Hello, Rocket!"
}
