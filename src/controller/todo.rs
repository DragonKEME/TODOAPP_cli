use std::collections::HashMap;
use http::{StatusCode};
use serde::{Serialize};
use crate::error::Error;
use crate::models::category_model::Category;
use crate::models::error_response;
use crate::models::todo_model::{Todo, TodoDto};
use crate::routes;
use crate::routes::Route;

#[derive(Serialize, Debug)]
struct TodoForm {
    content: String,
    idCategory: usize,
}

pub async fn make_new_todo(todos: &mut Vec<Todo>, content: String, category: &Category) -> Result<(),Box<dyn std::error::Error>> {
    let todo_form = TodoForm {
        content,
        idCategory: category.get_id(),
    };

    let res = Route::get_reqwest(routes::ADD_TODO)
        .body(serde_json::to_string(&todo_form)?).send().await?;

    todos.push(res.json::<Todo>().await?);
    Ok(())
}

pub async fn finished_todo(todos: &mut Vec<Todo>, todo_id: usize) -> Result<(),Box<dyn std::error::Error>> {

    let mut params=  HashMap::new();
    params.insert("idtodo".to_string(), todo_id.to_string());

    let res = Route::get_reqwest_param(routes::COMPLETE_TODO, &params)?
        .send().await?;

    let todo_dto = match res.status() {
        StatusCode::OK => res.json::<TodoDto>().await?,
        _ => return Err(Error::ServerError(res.json::<error_response::ErrorResponse>().await?).into()),
    };

    match find_one_mut_todo(todos,todo_id) {
        Some(todo) => todo.update_from_dto(todo_dto),
        None => return Err(Error::NonExistentTodo(todo_id).into())
    }

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