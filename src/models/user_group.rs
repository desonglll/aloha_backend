use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserGroup {
    id: Uuid,
    group_name: String,
}
