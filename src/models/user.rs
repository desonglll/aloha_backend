use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::dto::response::get_time_formatter;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, ToSchema)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    #[serde(skip_serializing, skip_deserializing)]
    pub created_at: Option<OffsetDateTime>,
    pub user_group_id: Option<Uuid>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, ToSchema)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub created_at: Option<String>,
    pub user_group_id: Option<Uuid>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            created_at: Some(
                user.created_at
                    .unwrap()
                    .format(&get_time_formatter())
                    .unwrap(),
            ),
            user_group_id: user.user_group_id,
        }
    }
}

impl User {
    pub fn default_test() -> Self {
        Self {
            id: Uuid::new_v4(),
            username: String::from("test_user"),
            password_hash: String::from("test_password_hash"),
            created_at: Some(OffsetDateTime::now_utc()),
            user_group_id: None,
        }
    }

    pub fn default_vec_test(number: Option<i32>) -> Vec<Self> {
        let number = number.unwrap_or(2);
        let mut result = Vec::<Self>::new();
        (0..number).for_each(|i| {
            let new = Self {
                id: Uuid::new_v4(),
                username: format!("test_user_{}", i),
                password_hash: String::from("test_password_hash"),
                created_at: Some(OffsetDateTime::now_utc()),
                user_group_id: None,
            };
            result.push(new);
        });
        result
    }

    pub fn new(username: String, password_hash: String, user_group_id: Option<Uuid>) -> Self {
        let id = Uuid::new_v4();
        Self {
            id,
            username,
            password_hash,
            created_at: Some(OffsetDateTime::now_utc()),
            user_group_id,
        }
    }
}
