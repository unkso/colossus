use domain::{repositories::Repository, user_types::UserRepository};
use libservices::{AbstractService, AttachableService, ServiceError};
use rocket::{Build, Rocket};
use rocket_db_pools::Database;

use crate::{db::Primary, types::User};

pub type UserService = Service<UserRepository>;

#[derive(AttachableService)]
#[service(alias = UserService)]
pub struct Service<U: Repository + Send + 'static> {
    user_repo: U,
}

impl<U> Service<U>
where
    U: Repository + 'static,
    User: From<<U as Repository>::Entity>,
{
    pub fn new(user_repo: U) -> Self {
        Self { user_repo }
    }

    pub async fn get_user(&self, id: <U as Repository>::Key) -> User {
        User::from(self.user_repo.find_by_id(id).await.unwrap())
    }

    pub async fn create_user(&self, user: <U as Repository>::Entity) -> Option<User> {
        if let Ok(user) = self.user_repo.create(user).await {
            Some(User::from(user))
        } else {
            None
        }
    }
}

#[rocket::async_trait]
impl AbstractService for UserService {
    type Error = ServiceError;

    async fn init(rocket: &Rocket<Build>) -> Result<Self, Self::Error> {
        let pool = Primary::fetch(rocket)
            .cloned()
            .ok_or(ServiceError::MissingRepository)?;
        Ok(Self::new(UserRepository::new(pool.0)))
    }
}

#[cfg(test)]
mod test {
    use chrono::Utc;
    use domain::user_types::{User, UserRepository};
    use faux::when;
    use uuid::Uuid;

    use super::super::super::types::User as ApiUser;
    use super::Service;

    #[rocket::async_test]
    async fn get_user() {
        let mock_user = User {
            id: Uuid::parse_str("57e99047-68fb-44ca-81b8-dd49d905d950").unwrap(),
            username: "John Doe".to_string(),
            password: "password".to_string(),
            email: "john@doe.com".to_string(),
            last_active: Utc::now().naive_utc(),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        let expected_user = ApiUser::from(mock_user.clone());
        let mut user_repo = UserRepository::faux();
        when!(user_repo.find_by_id(expected_user.id)).then_return(Some(mock_user.clone()));

        let service = Service::new(user_repo);

        assert_eq!(expected_user, service.get_user(expected_user.id).await);
    }

    #[rocket::async_test]
    async fn create_user() {
        let mock_user = User {
            id: Uuid::parse_str("57e99047-68fb-44ca-81b8-dd49d905d950").unwrap(),
            username: "John Doe".to_string(),
            password: "password".to_string(),
            email: "john@doe.com".to_string(),
            last_active: Utc::now().naive_utc(),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };

        let input_user = mock_user.clone();
        let expected_user = ApiUser::from(mock_user.clone());
        let mut user_repo = UserRepository::faux();
        when!(user_repo.create(mock_user.clone())).then_return(Ok(mock_user.clone()));

        let service = Service::new(user_repo);

        assert_eq!(
            expected_user,
            service.create_user(input_user).await.unwrap()
        );
    }
}
