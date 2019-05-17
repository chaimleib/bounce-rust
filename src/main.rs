extern crate actix_web;
use actix_web::{http, server, App, HttpRequest};

fn index(_req: &HttpRequest) -> &'static str {
    "success"
}

fn main() {
    let origin = format!("localhost:{}", 8080);
    let bounce = server::new(|| App::new()
                .resource("/", |r| r.method(http::Method::GET).f(index))
                );
    println!("Listening on http://{}", origin);
    bounce.bind(&origin).unwrap().run();
}
