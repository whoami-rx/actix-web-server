use actix_web::{App, middleware::Logger, HttpRequest, server};
use std::env::set_var;

fn index(_req: &HttpRequest) -> &'static str {
    "Hello, World!"
}

fn main() {
    set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    server::new(|| App::new()
        .middleware(Logger::default())
        .resource("/", |res| res.f(index)))
        .bind("127.0.0.1:8080")
        .unwrap()
        .run();
}
