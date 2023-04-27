use http::StatusCode;
use crate::error::Error;
use crate::models::category_model::Category;
use crate::models::error_response;
use crate::{routes, routes::Route};
use once_cell::sync::OnceCell;
use std::sync::Mutex;

static USER: OnceCell<Mutex<Vec<Category>>> = OnceCell::new();

fn ensure_categories() -> &'static Mutex<Vec<Category>> {
    USER.get_or_init(|| Mutex::new(get_server_categories().unwrap()))
}

pub fn get_categories() -> Vec<Category> {
    ensure_categories().lock().unwrap().clone()
}

pub fn set_categories(user: Vec<Category>) {
    *ensure_categories().lock().unwrap() = user;
}

pub fn get_server_categories() -> Result<Vec<Category>,Box<dyn std::error::Error>> {
    let res = Route::get_reqwest(routes::CATEGORY).send()?;

    let categories = match res.status() {
        StatusCode::OK => res.json::<Vec<Category>>()?,
        _ => return Err(Error::ServerError(res.json::<error_response::ErrorResponse>()?).into()),
    };

    Ok(categories)
}