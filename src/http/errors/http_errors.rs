use thiserror::Error;


#[derive(Error, Debug, PartialEq, Eq)]
pub enum HttpError
{
    #[error("An error occured")]
    DefaultError,

    #[error("Not found: {0}")]
    NotFoundError(String),

    #[error("Authorization Error: {0}")]
    UnauthorizedError(String),

    #[error("Bad Request: {0}")]
    BadRequest(String)

}