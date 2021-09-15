use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum ServiceError {
    #[error("data layer missing in shared state")]
    MissingRepository,
}
