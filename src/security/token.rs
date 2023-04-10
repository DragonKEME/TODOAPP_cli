use once_cell::sync::OnceCell;
use std::sync::Mutex;

static JWT: OnceCell<Mutex<String>> = OnceCell::new();

fn ensure_token() -> &'static Mutex<String> {
    JWT.get_or_init(|| Mutex::new(String::new()))
}

pub fn get_token() -> String {
    ensure_token().lock().unwrap().clone()
}

pub fn set_token(jwt: String) {
    let mut jwt = jwt;
    jwt.insert_str(0,"Bearer ");
    *ensure_token().lock().unwrap() = jwt;
}