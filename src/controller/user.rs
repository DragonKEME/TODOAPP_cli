use http::status::StatusCode;
use crate::models::user_model::User;
use crate::models::error_response;
use crate::error::Error;
use crate::routes;
use crate::routes::Route;
use serde::{Serialize, Deserialize};
use crate::models::todo_model::Todo;
use crate::view::main_panel;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserDto {
    id: usize,
    username: String,
    email: String,
    todos: Vec<Todo>,
}


pub async fn get_user_todo() -> Result<(User,Vec<Todo>) , Box<dyn std::error::Error>> {

    let res = Route::get_reqwest(routes::USER_INFO).send().await?;

    let user_todo = match res.status() {
        StatusCode::OK => res.json::<UserDto>().await?,
        _ => return Err(Error::ServerError(res.json::<error_response::ErrorResponse>().await?).into()),
    };
    let user = User::new(user_todo.id,user_todo.username,user_todo.email);

    main_panel::todo_list(&user_todo.todos);
    Ok((user,user_todo.todos))
}