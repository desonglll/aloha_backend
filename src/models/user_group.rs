use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct UserGroup {
    pub id: Uuid,
    pub group_name: String,
}

impl UserGroup {
    pub fn default_test() -> Self {
        Self {
            id: Uuid::new_v4(),
            group_name: String::from("Default Group"),
        }
    }

    pub fn new(group_name: String) -> Self {
        let id = Uuid::new_v4();
        Self { id, group_name }
    }
}
