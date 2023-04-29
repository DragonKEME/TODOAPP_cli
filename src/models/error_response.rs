use serde::{Deserialize};
use std::fmt;

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct ErrorResponse {
    statusCode : usize,
    message : String,
}


impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Code: {}, message: {}", self.statusCode , self.message)
    }
}