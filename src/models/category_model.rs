use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Category {
    id: usize,
    title: String,
}

impl Category {
    #![allow(dead_code)]
    pub fn new() -> Category{
        Category{
            id:0, title: "".to_string(),
        }
    }
    pub fn get_id(&self) -> usize {
        self.id
    }
    pub fn clone(&self) -> Category {
        Category{
            id: self.id,
            title: self.title.clone(),
        }
    }
    pub fn to_string(&self) -> String{
        self.title.clone()
    }
}