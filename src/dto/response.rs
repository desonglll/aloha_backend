use crate::dto::pagination::Pagination;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DtoResponse<T> {
    pub pagination: Option<Pagination>,
    pub data: T,
}
impl<T> DtoResponse<T> {
    pub fn new(data: T, pagination: Option<Pagination>) -> Self {
        DtoResponse { pagination, data }
    }
}
