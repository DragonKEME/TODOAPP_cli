use crate::controller::Category::get_categories;
use crate::controller::login::login;
use crate::controller::todo::{finished_todo, make_new_todo};

mod models;
mod controller;
mod error;
mod config;
mod routes;
mod security;

use serde::{Serialize, Deserialize};
use crate::controller::user::get_user_todo;

#[derive(Serialize, Deserialize, Debug)]
struct Test {
    nombre: usize,
    texte: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{

    let (user, mut todos) = login("newtest".to_string(), "proutprout".to_string()).await?;
    println!("Login and get todo");
    println!("Distant -> local Data: {:#?}",user);
    println!("{:#?}", todos);


    let categories = get_categories().await?;
    //println!("{:#?}",categories);

    make_new_todo(&mut todos, "test".to_string(), categories.first().unwrap()).await?;
    println!("Insert new Todo: ");
    println!("Local Data: {:#?}",todos);

    let todo_id = todos.first().unwrap().get_id();
    finished_todo(&mut todos, todo_id).await?;
    println!("Finished a todo: ");
    println!("Local Data: {:#?}",todos);
    println!("Server Data{:#?}",get_user_todo().await?);

    Ok(())
}
