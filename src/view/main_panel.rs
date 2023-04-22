use std::ops::Add;
use cursive::views::TextView;
use cursive::{Cursive, CursiveExt};
use crate::models::todo_model::Todo;

pub fn todo_list(todo_list: &Vec<Todo>, give_finished: bool) {
    let mut siv = Cursive::new();

    let mut final_string = "ID | todo | Category | finished\n".to_string();
    for todo in todo_list {
        if let Some( string) = todo.to_string(give_finished) {
            final_string += string.as_str();
            final_string.push('\n');
        }
    }
    siv.add_layer(TextView::new(final_string));

    siv.add_global_callback('q', |s| s.quit());

    siv.run();
}