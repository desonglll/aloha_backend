use actix_web::http::StatusCode;
use actix_web::ResponseError;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use std::fmt::{Display, Formatter};

#[derive(Debug, utoipa::ToSchema)]
pub enum AlohaError {
    RequestParameterInvalid(String),
    DatabaseError(String),
    UserIdInvalid,
    UserPasswordInvalid,
    UserNameInvalid,
    UserUnauthentication,
}

impl std::error::Error for AlohaError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl ResponseError for AlohaError {
    fn status_code(&self) -> StatusCode {
        match self {
            AlohaError::RequestParameterInvalid(_) => StatusCode::BAD_REQUEST,
            AlohaError::DatabaseError(_) => StatusCode::BAD_REQUEST,
            AlohaError::UserIdInvalid => StatusCode::BAD_REQUEST,
            AlohaError::UserPasswordInvalid => StatusCode::BAD_REQUEST,
            AlohaError::UserNameInvalid => StatusCode::BAD_REQUEST,
            AlohaError::UserUnauthentication => StatusCode::UNAUTHORIZED,
        }
    }
}

impl Display for AlohaError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AlohaError::RequestParameterInvalid(msg) => write!(f, "{}", msg),
            AlohaError::DatabaseError(msg) => write!(f, "{}", msg),
            AlohaError::UserIdInvalid => write!(f, "User ID is invalid."),
            AlohaError::UserPasswordInvalid => write!(f, "User password is invalid."),
            AlohaError::UserNameInvalid => write!(f, "User name is invalid."),
            AlohaError::UserUnauthentication => write!(f, "User is unauthenticated."),
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
            AlohaError::RequestParameterInvalid(_) => {
                s.serialize_field("code", &StatusCode::BAD_REQUEST.as_u16())?
            }
            AlohaError::DatabaseError(_) => {
                s.serialize_field("code", &StatusCode::BAD_REQUEST.as_u16())?
            }
            AlohaError::UserIdInvalid => {
                s.serialize_field("code", &StatusCode::BAD_REQUEST.as_u16())?
            }
            AlohaError::UserPasswordInvalid => {
                s.serialize_field("code", &StatusCode::BAD_REQUEST.as_u16())?
            }
            AlohaError::UserNameInvalid => {
                s.serialize_field("code", &StatusCode::BAD_REQUEST.as_u16())?
            }
            AlohaError::UserUnauthentication => {
                s.serialize_field("code", &StatusCode::UNAUTHORIZED.as_u16())?
            }
        };
        s.serialize_field("error", &format!("{}", self))?;
        s.end()
    }
}
