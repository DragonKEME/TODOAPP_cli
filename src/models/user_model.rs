use crate::models::todo_model::Todo;

#[derive(Debug)]
pub struct User {
    id: usize,
    username: String,
    email: String,
}

pub type UserTodo = (User,Vec<Todo>);

impl User {
    pub fn new(id: usize, username: String, email: String) -> User{
        User { id, username, email }
    }

}
