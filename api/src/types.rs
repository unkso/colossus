use chrono::NaiveDateTime;
use domain::user_types::User as DomainUser;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub last_active: NaiveDateTime,
}

impl From<DomainUser> for User {
    fn from(du: DomainUser) -> Self {
        Self {
            id: du.id,
            username: du.username,
            last_active: du.last_active,
        }
    }
}

#[derive(Deserialize, PartialEq, Eq, Debug)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub email: &'a str,
}
