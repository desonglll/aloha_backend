use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DtoQuery<T> {
    pub page: Option<usize>,
    pub size: Option<usize>,
    pub sort: Option<String>,
    pub order: Option<String>,
    pub filter: Option<T>,
}

impl<T> DtoQuery<T> {
    pub fn default_query() -> Self {
        DtoQuery {
            page: Some(1),
            size: Some(10),
            sort: None,
            order: None,
            filter: None,
        }
    }
    pub fn page(&self) -> usize {
        self.page.unwrap_or(1)
    }

    pub fn size(&self) -> usize {
        self.size.unwrap_or(10)
    }

    pub fn offset(&self) -> usize {
        (self.page() - 1) * self.size()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserFilterQuery {
    #[serde(rename = "user_group_id")]
    pub user_group_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPermissionFilterQuery {}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserGroupFilterQuery {}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupPermissionFilterQuery {}

#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionFilterQuery {}
