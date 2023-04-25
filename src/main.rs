
mod models;
mod controller;
mod error;
mod config;
mod routes;
mod security;
mod view;

use crate::view::main_panel;

fn main() -> Result<(), Box<dyn std::error::Error>>{


    //Api test
    //let (user, mut todos) = login("nem".to_string(), "proutprout".to_string()).await?;
    main_panel::todo_list()?;
    //println!("Login and get todo");
    //println!("Distant -> local Data: {:#?}",user);
    //println!("{:#?}", todos);

    /*
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
    */

    Ok(())
}
