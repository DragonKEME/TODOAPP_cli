use serde::{Serialize, Deserialize};
use crate::models::category_model::Category;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Todo {
    id: usize,
    content: String,
    finished: bool,
    created_at: String,
    category: Category,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TodoDto {
    id: usize,
    content: String,
    finished: bool,
    created_at: String,
}

impl Todo {
    #![allow(dead_code)]
    pub fn new(id: usize, content: String, finished: bool, created_at: String, category: Category ) -> Todo{
        Todo {
            id, content, finished, created_at, category
        }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }
    pub fn get_content(&self) -> String{
        self.content.clone()
    }
    pub fn is_terminated(&self) -> bool{
        self.finished
    }

    pub fn update_from_dto(&mut self, todo_dto: TodoDto){
        self.created_at = todo_dto.created_at;
        self.finished = todo_dto.finished;
        self.content = todo_dto.content;
    }

    /**
    * The option is only if give_finished = false
    * Else function always return string
    */
    pub fn to_string(&self, give_finished: bool) -> Option<String> {
        if give_finished || !self.finished{
            Some(self.id.to_string()
                + "," + self.content.as_str()
                + "," + self.category.to_string().as_str()
                + "," + self.finished_str())
        }else{
            None
        }
    }
    pub fn to_format_string(&self,content_size: usize,  give_finished: bool) -> Option<String> {
        if give_finished || !self.finished{
            if give_finished{
                Some(format!("{:<5}|{:<content_size$}|{:>18}|{:>8}",self.id.to_string(),self.content,self.category.to_string(),self.finished_str()))
            }else{
                Some(format!("{:<5}|{:<content_size$}|{:>18}",self.id.to_string(),self.content,self.category.to_string()))
            }
        }else{
            None
        }
    }

    fn finished_str(&self) -> &str{
        if self.finished{
            "yes"
        }else{
            "no"
        }
    }
}