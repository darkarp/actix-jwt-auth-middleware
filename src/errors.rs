use actix_web::{body::BoxBody, http::StatusCode, HttpResponse, ResponseError};
use jwt_compact::{CreationError, ParseError, ValidationError};

pub type AuthResult<T> = Result<T, AuthError>;

/// if #[cfg(debug_assertions)] is true the wrapped errors (TokenCreation, TokenValidation, TokenParse) are in included in the error message
#[derive(Debug)]
pub enum AuthError {
    /// returned if there is no cookie under the `Authority::cookie_name` name
    NoCookie,
    /// returned if the guard function returns false
    Unauthorized,
    TokenCreation(CreationError),
    TokenValidation(ValidationError),
    TokenParse(ParseError),
}

impl Into<AuthError> for CreationError {
    fn into(self) -> AuthError {
        AuthError::TokenCreation(self)
    }
}

impl Into<AuthError> for ParseError {
    fn into(self) -> AuthError {
        AuthError::TokenParse(self)
    }
}

impl Into<AuthError> for ValidationError {
    fn into(self) -> AuthError {
        AuthError::TokenValidation(self)
    }
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[cfg(not(debug_assertions))]
        {
            f.write_str(&match self {
                AuthError::NoCookie => {
                    "you have provided no cookie, or your cookie has the wrong name"
                }
                AuthError::Unauthorized => "you are not authorized to interact with this scope",
                _ => "something went wrong parsing your token",
                AuthError::TokenCreation(_) => "there was an internal error creating your token",
                AuthError::TokenValidation(_) => "it seems your token could not be verified",
                AuthError::TokenParse(_) => "it seems there has been an error parsing your token",
            })
        }

        #[cfg(debug_assertions)]
        match self {
            AuthError::NoCookie => {
                f.write_str("you have provided no cookie, or your cookie has the wrong name")
            }
            AuthError::Unauthorized => {
                f.write_str("you are not authorized to interact with this scope")
            }
            AuthError::TokenCreation(err) => f.write_fmt(format_args!(
                "there was an internal error creating your token.\n\n Error: \"{err}\""
            )),
            AuthError::TokenValidation(err) => f.write_fmt(format_args!(
                "it seems your token could not be verified.\n\n Error: \"{err}\""
            )),
            AuthError::TokenParse(err) => f.write_fmt(format_args!(
                "it seems there hasebeen an error parsing your token.\n\n Error: \"{err}\""
            )),
        }
    }
}

impl ResponseError for AuthError {
    fn status_code(&self) -> StatusCode {
        match self {
            AuthError::NoCookie => StatusCode::UNAUTHORIZED,
            AuthError::Unauthorized => StatusCode::UNAUTHORIZED,
            AuthError::TokenCreation(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AuthError::TokenValidation(_) => StatusCode::UNAUTHORIZED,
            AuthError::TokenParse(_) => StatusCode::BAD_REQUEST,
        }
    }
    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code()).body(self.to_string())
    }
}
