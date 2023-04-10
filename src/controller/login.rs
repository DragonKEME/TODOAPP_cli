use serde::{Serialize, Deserialize};
use http::status::StatusCode;
use crate::controller::user;
use crate::models::user_model::UserTodo;
use crate::models::error_response;
use crate::error::Error;
use crate::routes;
use crate::routes::Route;
use crate::security::token;

#[derive(Serialize, Debug)]
struct LoginForm {
    username: String,
    password: String,
}

#[derive(Deserialize, Debug)]
struct LoginResponse {
    expiresIn: String,
    accessToken: String,
}

pub async fn login(username: String, password: String) -> Result<UserTodo, Box<dyn std::error::Error>> {
    if username.is_empty() {
        return Err(Error::PASSWORDLEN.into());
    }

    if password.len() < 8 {
        return Err(Error::EMPTYUSERNAME.into());
    }

    let login_form = LoginForm {
        username,
        password,
    };

    let res = Route::get_reqwest(routes::LOGIN)
        .body(serde_json::to_string(&login_form)?).send().await?;

    let login_response = match res.status() {
        StatusCode::CREATED => res.json::<LoginResponse>().await?,
        _ => return Err(Error::ServerError(res.json::<error_response::ErrorResponse>().await?).into()),
    };

    token::set_token(login_response.accessToken);


    user::get_user_todo().await
}