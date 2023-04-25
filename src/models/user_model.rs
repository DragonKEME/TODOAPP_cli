use crate::models::todo_model::Todo;

#[derive(Debug,Clone)]
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

    pub fn get_id(&self) -> usize {
        self.id
    }
    pub fn get_username(&self) -> &String{
        &self.username
    }
    pub fn get_email(&self) -> &String{
        &self.email
    }
}
