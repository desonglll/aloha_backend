use serde::{Deserialize, Serialize};
use sqlx::types::time::OffsetDateTime;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct UserGroup {
    pub id: Uuid,
    pub group_name: String,
    pub created_at: Option<OffsetDateTime>,
}

impl UserGroup {
    pub fn default_test() -> Self {
        Self {
            id: Uuid::new_v4(),
            group_name: String::from("Default Group"),
            created_at: Some(OffsetDateTime::now_utc()),
        }
    }

    pub fn default_vec_test(number: Option<i32>) -> Vec<Self> {
        let number = number.unwrap_or(2);
        let mut result = Vec::<Self>::new();
        (0..number).for_each(|_| {
            let new = Self {
                id: Uuid::new_v4(),
                group_name: String::from(Uuid::new_v4()),
                created_at: Some(OffsetDateTime::now_utc()),
            };
            result.push(new);
        });
        result
    }
    pub fn new(group_name: String) -> Self {
        let id = Uuid::new_v4();
        Self {
            id,
            group_name,
            created_at: Some(OffsetDateTime::now_utc()),
        }
    }
}
