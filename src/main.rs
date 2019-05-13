#[macro_use] extern crate serde_derive;
use actix_web::{
    App, middleware::Logger, HttpRequest, server, http::Method,
    Result, Json
};
use std::env::set_var;

#[derive(Serialize)] 
struct User {
    name: String
}

impl User {
    fn new(n: &str) -> Self {
        Self { name: n.to_string() }
    }
}

fn index(_req: &HttpRequest) -> Result<Json<User>> {
    Ok(Json(User::new("Username example")))
}

fn main() {
    set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    server::new(|| App::new()
        .middleware(Logger::default())
        .resource("/", |res| res.method(Method::GET).f(index)))
        .bind("127.0.0.1:8080")
        .unwrap()
        .run();
}
