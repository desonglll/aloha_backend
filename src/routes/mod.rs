use serde::Deserialize;

pub mod health_check;
pub mod user_group;

#[derive(Clone, Debug, Deserialize)]
pub struct Routes {
    pub user_groups: String,
}
