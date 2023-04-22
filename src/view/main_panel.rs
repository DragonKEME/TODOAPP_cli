use std::ops::Add;
use cursive::views::TextView;
use cursive::{Cursive, CursiveExt};
use crate::models::todo_model::Todo;

pub fn todo_list(todo_list: &Vec<Todo>) {
    let mut siv = Cursive::new();

    let mut final_string = "".to_string();
    for todo in todo_list {
        final_string += todo.to_string().as_str();
        final_string.push('\n');
    }
    siv.add_layer(TextView::new(final_string));

    siv.add_global_callback('q', |s| s.quit());

    siv.run();
}