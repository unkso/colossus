use domain::user_types::User as DUser;
use rocket::{get, post, serde::json::Json};
use sqlx::types::{chrono::Utc, Uuid};

use crate::{
    services::users::UserService,
    types::{NewUser, User},
};

#[get("/<uuid>")]
pub async fn get(user_svc: &UserService, uuid: Uuid) -> Json<User> {
    let user = user_svc.get_user(uuid).await;

    Json(user)
}

#[post("/", data = "<new_user>")]
pub async fn create(user_svc: &UserService, new_user: Json<NewUser<'_>>) -> Option<Json<User>> {
    let user = DUser {
        id: Uuid::new_v4(),
        username: new_user.username.into(),
        password: new_user.password.into(),
        email: new_user.email.into(),
        last_active: Utc::now().naive_utc(),
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    user_svc.create_user(user).await.map(Json)
}
