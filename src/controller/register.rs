use serde::{Serialize, Deserialize};
use http::status::StatusCode;
use crate::controller::login;
use crate::models::error_response;
use crate::error::Error;
use crate::routes;
use crate::routes::Route;

#[derive(Serialize, Debug)]
struct RegisterForm {
    username: String,
    email: String,
    password: String,
}

#[derive(Deserialize, Debug)]
struct RegisterResponse {
    success: bool,
    message: String,
}

pub fn register(username: String, email: String, password: String) -> Result<(), Box<dyn std::error::Error>> {

    if username.is_empty() {
        return Err(Error::PASSWORDLEN.into());
    }

    if email.is_empty() {
        return Err(Error::EMPTYEMAIL.into());
    }
    if password.len() < 8 {
        return Err(Error::PASSWORDLEN.into());
    }

    let register_form = RegisterForm {
        username: username.clone(),
        email,
        password: password.clone(),
    };

    let res = Route::get_reqwest(routes::REGISTER)
        .body(serde_json::to_string(&register_form)?).send()?;

    match res.status() {
        StatusCode::CREATED => res.json::<RegisterResponse>()?,
        _ => return Err(Error::ServerError(res.json::<error_response::ErrorResponse>()?).into()),
    };

    login::login(username,password)
}
