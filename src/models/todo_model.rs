use std::ops::Add;
use serde::{Serialize, Deserialize};
use tokio::task_local;
use crate::controller::user::UserDto;
use crate::models::category_model::Category;

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    id: usize,
    content: String,
    finished: bool,
    createdAt: String,
    category: Category,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoDto {
    id: usize,
    content: String,
    finished: bool,
    createdAt: String,
}

impl Todo {
    pub fn new(id: usize, content: String, finished: bool, created_at: String, category: Category ) -> Todo{
        Todo {
            id, content, finished, createdAt: created_at, category
        }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn update_from_dto(&mut self, todo_dto: TodoDto){
        self.createdAt = todo_dto.createdAt;
        self.finished = todo_dto.finished;
        self.content = todo_dto.content;
    }

    pub fn to_string(&self) -> String {
        self.id.to_string() + self.content.as_str() + self.category.to_string().as_str()
    }
}