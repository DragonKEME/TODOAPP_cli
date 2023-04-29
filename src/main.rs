
mod models;
mod controller;
mod error;
mod config;
mod routes;
mod security;
mod view;

use crate::view::main_panel;

fn main(){
    main_panel::todo_list();
}
