use crate::dto::response::get_time_formatter;
use crate::routes::permission::CreatePermissionFormData;
use serde::{Deserialize, Serialize};
use sqlx::types::time::OffsetDateTime;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, ToSchema)]
pub struct Permission {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    #[schema(value_type = String)]
    pub created_at: Option<OffsetDateTime>,
}
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, ToSchema)]
pub struct PermissionResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: Option<String>,
}

impl From<Permission> for PermissionResponse {
    fn from(value: Permission) -> Self {
        Self {
            id: value.id,
            name: value.name,
            description: value.description,
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

impl From<CreatePermissionFormData> for Permission {
    fn from(value: CreatePermissionFormData) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: value.name,
            description: value.description,
            created_at: Some(OffsetDateTime::now_utc()),
        }
    }
}

impl Permission {
    pub fn default_test() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: String::from("Default Permission"),
            description: Some(String::from("Default permission description")),
            created_at: Some(OffsetDateTime::now_utc()),
        }
    }

    pub fn default_vec_test(number: Option<i32>) -> Vec<Self> {
        let number = number.unwrap_or(2);
        let mut result = Vec::<Self>::new();
        (0..number).for_each(|_| {
            let new = Self {
                id: Uuid::new_v4(),
                name: String::from(Uuid::new_v4()),
                description: Some(String::from("Test permission description")),
                created_at: Some(OffsetDateTime::now_utc()),
            };
            result.push(new);
        });
        result
    }

    pub fn new(name: String, description: Option<String>) -> Self {
        let id = Uuid::new_v4();
        Self {
            id,
            name,
            description,
            created_at: Some(OffsetDateTime::now_utc()),
        }
    }
}
