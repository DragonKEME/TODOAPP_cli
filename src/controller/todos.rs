use std::collections::HashMap;
use http::{StatusCode};
use serde::{Serialize};
use crate::error::Error;
use crate::models::category_model::Category;
use crate::models::error_response;
use crate::models::todo_model::{Todo, TodoDto};
use crate::routes;
use crate::routes::Route;
use once_cell::sync::OnceCell;
use std::sync::Mutex;


#[derive(Serialize, Debug)]
struct TodoForm {
    content: String,
    idCategory: usize,
}

static TODOS: OnceCell<Mutex<Vec<Todo>>> = OnceCell::new();

fn ensure_todos() -> &'static Mutex<Vec<Todo>> {
    TODOS.get_or_init(|| Mutex::new(Vec::new()))
}

pub fn get_todos() -> Vec<Todo> {
    ensure_todos().lock().unwrap().clone()
}

pub fn set_todos(todos: Vec<Todo>) {
    *ensure_todos().lock().unwrap() = todos;
}

pub fn reset_todos(){
    *ensure_todos().lock().unwrap() = Vec::new()
}

pub fn make_new_todo(content: String, category: &Category) -> Result<(),Box<dyn std::error::Error>> {
    let mut todos = get_todos();
    let todo_form = TodoForm {
        content,
        idCategory: category.get_id(),
    };

    let res = Route::get_reqwest(routes::ADD_TODO)
        .body(serde_json::to_string(&todo_form)?).send()?;

    todos.push(res.json::<Todo>()?);
    set_todos(todos);
    Ok(())
}

pub fn finished_todo(todo_id: usize) -> Result<(),Box<dyn std::error::Error>> {
    let mut todos = get_todos();
    let mut params=  HashMap::new();
    params.insert("idtodo".to_string(), todo_id.to_string());

    let res = Route::get_reqwest_param(routes::COMPLETE_TODO, &params)?
        .send()?;

    let todo_dto = match res.status() {
        StatusCode::OK => res.json::<TodoDto>()?,
        _ => return Err(Error::ServerError(res.json::<error_response::ErrorResponse>()?).into()),
    };

    match find_one_mut_todo(&mut todos,todo_id) {
        Some(todo) => todo.update_from_dto(todo_dto),
        None => return Err(Error::NonExistentTodo(todo_id).into())
    }
    set_todos(todos);
    Ok(())
}

pub fn find_one_mut_todo(todos: &mut Vec<Todo>, todo_id: usize) -> Option<&mut Todo> {
    for user_todo in todos{
        if user_todo.get_id() == todo_id {
            return Some(user_todo)
        }
    }
    None
}