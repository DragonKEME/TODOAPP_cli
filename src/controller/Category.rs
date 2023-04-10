use http::StatusCode;
use crate::error::Error;
use crate::models::category_model::Category;
use crate::models::error_response;
use crate::routes;
use crate::routes::Route;

pub async fn get_categories() -> Result<Vec<Category>,Box<dyn std::error::Error>> {
    let res = Route::get_reqwest(routes::CATEGORY).send().await?;

    let user_todo = match res.status() {
        StatusCode::OK => res.json::<Vec<Category>>().await?,
        _ => return Err(Error::ServerError(res.json::<error_response::ErrorResponse>().await?).into()),
    };

    Ok(user_todo)
}