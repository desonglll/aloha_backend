use crate::dto::response::get_time_formatter;
use crate::routes::group_permission::CreateGroupPermissionFormData;
use serde::{Deserialize, Serialize};
use sqlx::types::time::OffsetDateTime;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, utoipa::ToSchema)]
pub struct GroupPermission {
    pub group_id: Uuid,
    pub permission_id: Uuid,
    #[serde(skip)]
    #[schema(value_type = String)]
    pub created_at: Option<OffsetDateTime>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, utoipa::ToSchema)]
pub struct GroupPermissionResponse {
    pub group_id: Uuid,
    pub permission_id: Uuid,
    pub created_at: Option<String>,
}

impl From<CreateGroupPermissionFormData> for GroupPermission {
    fn from(value: CreateGroupPermissionFormData) -> Self {
        Self {
            group_id: value.group_id,
            permission_id: value.permission_id,
            created_at: Some(OffsetDateTime::now_utc()),
        }
    }
}

impl From<GroupPermission> for GroupPermissionResponse {
    fn from(value: GroupPermission) -> Self {
        Self {
            group_id: value.group_id,
            permission_id: value.permission_id,
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
