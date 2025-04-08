use actix_web::http::StatusCode;
use actix_web::ResponseError;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use std::fmt::{Display, Formatter};

#[derive(Debug, utoipa::ToSchema)]
pub enum AlohaError {
    DatabaseError(String),
}

impl ResponseError for AlohaError {
    fn status_code(&self) -> StatusCode {
        match self {
            AlohaError::DatabaseError(_) => StatusCode::BAD_REQUEST,
        }
    }
}

impl Display for AlohaError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AlohaError::DatabaseError(msg) => write!(f, "{}", msg),
        }
    }
}

impl Serialize for AlohaError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("AlohaError", 1)?;
        match self {
            AlohaError::DatabaseError(_) => {
                s.serialize_field("code", &StatusCode::BAD_REQUEST.as_u16())?
            }
        };
        s.serialize_field("error", &format!("{}", self))?;
        s.end()
    }
}
