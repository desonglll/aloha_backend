use serde::{Deserialize, Serialize};
use sqlx::types::time::OffsetDateTime;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct GroupPermission {
    pub group_id: Uuid,
    pub permission_id: Uuid,
    pub created_at: Option<OffsetDateTime>,
}

impl GroupPermission {
    pub fn default_test() -> Self {
        Self {
            group_id: Uuid::new_v4(),
            permission_id: Uuid::new_v4(),
            created_at: Some(OffsetDateTime::now_utc()),
        }
    }

    pub fn default_vec_test(number: Option<i32>) -> Vec<Self> {
        let number = number.unwrap_or(2);
        let mut result = Vec::<Self>::new();
        (0..number).for_each(|_| {
            let new = Self {
                group_id: Uuid::new_v4(),
                permission_id: Uuid::new_v4(),
                created_at: Some(OffsetDateTime::now_utc()),
            };
            result.push(new);
        });
        result
    }

    pub fn new(group_id: Uuid, permission_id: Uuid) -> Self {
        Self {
            group_id,
            permission_id,
            created_at: Some(OffsetDateTime::now_utc()),
        }
    }
}
