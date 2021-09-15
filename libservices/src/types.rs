use std::marker::PhantomData;

use rocket::{
    error,
    fairing::{self, Fairing, Info, Kind},
    Build, Rocket,
};

#[rocket::async_trait]
pub trait AbstractService: Send + Sync + Sized + 'static {
    type Error: std::error::Error;

    async fn init(rocket: &Rocket<Build>) -> Result<Self, Self::Error>;
}

pub trait AttachableService: From<Self::Service> + Send + Sync + 'static {
    type Service: AbstractService;

    fn init() -> Initializer<Self> {
        Initializer::new()
    }
}

pub struct Initializer<A: AttachableService>(Option<&'static str>, PhantomData<fn() -> A>);

impl<A: AttachableService> Initializer<A> {
    pub fn new() -> Self {
        Self(None, PhantomData)
    }

    pub fn with_name(name: &'static str) -> Self {
        Self(Some(name), PhantomData)
    }
}

impl<A: AttachableService> Default for Initializer<A> {
    fn default() -> Self {
        Self(None, PhantomData)
    }
}

#[rocket::async_trait]
impl<A: AttachableService> Fairing for Initializer<A> {
    fn info(&self) -> Info {
        Info {
            name: self.0.unwrap_or_else(std::any::type_name::<Self>),
            kind: Kind::Ignite,
        }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> fairing::Result {
        match <A::Service>::init(&rocket).await {
            Ok(service) => Ok(rocket.manage(A::from(service))),
            Err(e) => {
                error!("failed to initialize service: {}", e);
                Err(rocket)
            }
        }
    }
}
