use serde::Deserialize;

pub mod group_permission;
pub mod health_check;
pub mod permission;
pub mod user;
pub mod user_group;

#[derive(Clone, Debug, Deserialize)]
pub struct Routes {
    pub user_groups: String,
    pub users: String,
    pub permissions: String,
    pub group_permissions: String,
}
