use std::collections::HashMap;
use http::method::Method;
use crate::security::token::get_token;
use crate::config;
use crate::error::Error;

pub struct  Route<'a> {
    method: Method,
    path: &'a str,
    require_token: bool,
}

impl<'a> Route<'a> {
    fn new(method: Method,path: &'a str, require_token: bool) -> Route<'a>{
        Route { method, path, require_token}
    }

    pub fn get_reqwest(self) -> reqwest::RequestBuilder{
        let mut url: String = config::TODOAPP_API_URL.to_string();
        url.push_str(self.path);
        let rb = reqwest::Client::new().request(self.method,url)
            .header("Content-Type", "application/json");

        if self.require_token {
            return rb.header("Authorization", get_token())
        }
        rb
    }

    pub fn get_reqwest_param(self ,params: &HashMap<String,String>) -> Result<reqwest::RequestBuilder,Box<dyn std::error::Error>> {
        let mut str = self.path.to_string();
        for (key,value) in params{
            let str_replace = "{".to_string() + key + "}";
            
            match str.find(key) { 
                Some(t) => t,
                None => return Err(Error::ParamsErrorNotFound(key.to_string()).into())
            };
            
            str = str.replace(str_replace.as_str(),value);

        }
        let mut url: String = config::TODOAPP_API_URL.to_string();
        url.push_str(str.as_str());
        let rb = reqwest::Client::new().request(self.method,url)
            .header("Content-Type", "application/json");

        if self.require_token {
            return Ok(rb.header("Authorization", get_token()))
        }
        Ok(rb)
    }
}

pub const USER_INFO: Route = Route {method: Method::GET, path: "/users/infos",require_token: true};
pub const CATEGORY: Route = Route {method: Method::GET, path: "/categories",require_token: true};
pub const ADD_TODO: Route = Route {method: Method::POST, path: "/todos",require_token: true};
pub const COMPLETE_TODO: Route = Route {method: Method::PUT, path: "/todos/{idtodo}",require_token: true};
pub const LOGIN: Route = Route {method:Method::POST, path: "/auth/login",require_token: false};
pub const REGISTER: Route = Route {method:Method::POST, path: "/auth/register",require_token: false};
