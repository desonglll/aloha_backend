use crate::dto::pagination::Pagination;
use serde::{Deserialize, Serialize};
use time::{format_description, formatting::Formattable};

#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct DtoResponse<T> {
    pub pagination: Option<Pagination>,
    pub data: T,
}
impl<T> DtoResponse<T> {
    pub fn new(data: T, pagination: Option<Pagination>) -> Self {
        DtoResponse { pagination, data }
    }
}

pub fn get_time_formatter() -> impl Formattable {
    format_description::parse(
        "[year]-[month]-[day] [hour]:[minute]:[second] [offset_hour \
         sign:mandatory]:[offset_minute]:[offset_second]",
    )
    .unwrap()
}
