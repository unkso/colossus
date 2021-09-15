#![allow(dead_code)]
use libservices::{AbstractService, AttachableService, ServiceError};

struct DummyRepo;

type DummyService = Service<DummyRepo>;

#[derive(AttachableService)]
#[service(alias = DummyService)]
struct Service<S: Send + Sync + 'static> {
    repo: S,
}

impl<S: Send + Sync + 'static> Service<S> {
    pub fn new(repo: S) -> Self {
        Self { repo }
    }
}

#[rocket::async_trait]
impl AbstractService for DummyService {
    type Error = ServiceError;

    async fn init(_rocket: &rocket::Rocket<rocket::Build>) -> Result<Self, Self::Error> {
        Ok(Self::new(DummyRepo))
    }
}

fn main() {}
