use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Category {
    id: usize,
    title: String,
}

impl Category {
    pub fn get_id(&self) -> usize {
        self.id
    }
    pub fn clone(&self) -> Category {
        Category{
            id: self.id,
            title: self.title.clone(),
        }
    }
    //TODO : Clone trait
}