#![allow(dead_code)]
use libservices::{AbstractService, AttachableService, ServiceError};

#[derive(AttachableService)]
#[service(alias = 42)]
struct Service;

impl Service {
    pub fn new() -> Self {
        Self
    }
}

#[rocket::async_trait]
impl AbstractService for Service {
    type Error = ServiceError;

    async fn init(_rocket: &rocket::Rocket<rocket::Build>) -> Result<Self, Self::Error> {
        Ok(Self::new())
    }
}

fn main() {}
