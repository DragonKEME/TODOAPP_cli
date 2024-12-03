use std::{error, fmt};
use crate::models::error_response::ErrorResponse;

#[derive(Debug)]
pub enum Error {
    PASSWORDLEN,
    EMPTYUSERNAME,
    EMPTYEMAIL,
    EmptyTodo,
    NonExistentTodo(usize),
    ServerError(ErrorResponse),
    ParamsErrorNotFound(String),
    InputDateError(String),
    EmptyCategoru,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::PASSWORDLEN => write!(f,"The password must be longer than 8 characters"),
            Error::EMPTYUSERNAME => write!(f,"The username can't be empty"),
            Error::EMPTYEMAIL => write!(f,"The email can't be empty"),
            Error::EmptyTodo => write!(f,"This todo as no content"),
            Error::NonExistentTodo(id) => write!(f,"The todo {} doesn't exist", id),
            Error::ServerError(err) => write!(f, "Server error: {}", err),
            Error::ParamsErrorNotFound(key) => write!(f, "Parameter {} not found in route",key),
            Error::InputDateError(s) => write!(f, "Invalid Date: {}",s),
            Error::EmptyCategoru => write!(f, "No category available on the server, contact administrator"),
        }
    }
}

impl error::Error for Error {}

