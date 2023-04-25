use http::status::StatusCode;
use crate::models::user_model::User;
use crate::models::error_response;
use crate::error::Error;
use crate::routes;
use crate::routes::Route;
use serde::{Serialize, Deserialize};
use crate::controller::todos::set_todos;
use crate::models::todo_model::Todo;
use crate::view::main_panel;
use once_cell::sync::OnceCell;
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserDto {
    id: usize,
    username: String,
    email: String,
    todos: Vec<Todo>,
}

static USER: OnceCell<Mutex<User>> = OnceCell::new();

fn ensure_todos() -> &'static Mutex<User> {
    USER.get_or_init(|| Mutex::new(User::new(0,"".to_string(),"".to_string())))
}

pub fn get_user() -> User {
    ensure_todos().lock().unwrap().clone()
}

pub fn set_user(user: User) {
    *ensure_todos().lock().unwrap() = user;
}

pub fn get_user_todo() -> Result<(User,Vec<Todo>) , Box<dyn std::error::Error>> {

    let res = Route::get_reqwest(routes::USER_INFO).send()?;

    let user_todo = match res.status() {
        StatusCode::OK => res.json::<UserDto>()?,
        _ => return Err(Error::ServerError(res.json::<error_response::ErrorResponse>()?).into()),
    };
    let user = User::new(user_todo.id,user_todo.username,user_todo.email);
    set_todos(user_todo.todos.clone());
    set_user(user.clone());
    Ok((user,user_todo.todos))
}