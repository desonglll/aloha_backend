use crate::dto::response::get_time_formatter;
use crate::routes::user_group::CreateUserGroupFormData;
use serde::{Deserialize, Serialize};
use sqlx::types::time::OffsetDateTime;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, ToSchema)]
pub struct UserGroup {
    pub id: Uuid,
    pub group_name: String,
    #[schema(value_type = String)]
    pub created_at: Option<OffsetDateTime>,
}
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, ToSchema)]
pub struct UserGroupResponse {
    pub id: Uuid,
    pub group_name: String,
    pub created_at: Option<String>,
}

impl From<UserGroup> for UserGroupResponse {
    fn from(value: UserGroup) -> Self {
        Self {
            id: value.id,
            group_name: value.group_name,
            created_at: Some(
                value
                    .created_at
                    .unwrap()
                    .format(&get_time_formatter())
                    .unwrap(),
            ),
        }
    }
}

impl From<CreateUserGroupFormData> for UserGroup {
    fn from(value: CreateUserGroupFormData) -> Self {
        Self {
            id: Uuid::new_v4(),
            group_name: value.group_name,
            created_at: Some(OffsetDateTime::now_utc()),
        }
    }
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
    pub fn new(id: Uuid, group_name: String) -> Self {
        Self {
            id,
            group_name,
            created_at: Some(OffsetDateTime::now_utc()),
        }
    }
}
