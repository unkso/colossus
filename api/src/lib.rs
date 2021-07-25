#![warn(
    clippy::all,
    // clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]

//! Some crate documentation

pub(crate) mod db;
pub(crate) mod handlers;
pub(crate) mod services;
pub(crate) mod types;

use domain::user_types::UserRepository;
use rocket::{fairing::AdHoc, get, routes, Build, Rocket};
use rocket_db_pools::Database;

use crate::{
    db::Primary,
    handlers::users::{create, get},
    services::users::UserService,
};

/// Constructs a `Rocket<Build>` instance
#[allow(clippy::option_if_let_else)]
#[must_use]
pub fn rocket() -> Rocket<Build> {
    rocket::build()
        .attach(Primary::init())
        .attach(AdHoc::on_ignite("UserService", |rocket| {
            Box::pin(async move {
                if let Some(db) = Primary::fetch(&rocket).cloned() {
                    rocket.manage(UserService::new(UserRepository::new(db.0)))
                } else {
                    rocket
                }
            })
        }))
        .mount("/", routes![index])
        .mount("/users", routes![create, get])
}

#[get("/")]
const fn index() -> &'static str {
    "Hello, Rocket!"
}
