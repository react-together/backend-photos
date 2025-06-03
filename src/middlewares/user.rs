use crate::entities::user;
use crate::middlewares::keycloak::Token;
use crate::persistances::db;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub from_token: Token,
    pub from_db: user::Model,
}

impl User {
    async fn find_or_create(token: &Token) -> Result<user::Model, String> {
        let db = db::get();

        // Try to find existing user
        if let Ok(Some(user)) = Self::find_user_by_sub(&token.sub, &db).await {
            return Ok(user);
        }

        // Create new user if not found
        Self::create_user(token, &db).await
    }

    async fn find_user_by_sub(
        sub: &str,
        db: &sea_orm::DatabaseConnection,
    ) -> Result<Option<user::Model>, String> {
        user::Entity::find()
            .filter(user::Column::KeycloakSub.eq(sub))
            .one(db)
            .await
            .map_err(|e| e.to_string())
    }

    async fn create_user(
        token: &Token,
        db: &sea_orm::DatabaseConnection,
    ) -> Result<user::Model, String> {
        let new_user = user::ActiveModel {
            keycloak_sub: Set(token.sub.clone()),
            email: Set(token.sub.clone()), // Using sub as email for now
            ..Default::default()
        };

        new_user.insert(db).await.map_err(|e| e.to_string())
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = String;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Get token from request
        let from_token = match Token::from_request(req).await {
            Outcome::Success(token) => token,
            Outcome::Error((status, err)) => return Outcome::Error((status, err)),
            Outcome::Forward(status) => return Outcome::Forward(status),
        };

        // Find or create user
        let from_db = match User::find_or_create(&from_token).await {
            Ok(user) => user,
            Err(e) => return Outcome::Error((Status::InternalServerError, e)),
        };

        Outcome::Success(User {
            from_token,
            from_db,
        })
    }
}
