use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing, Router, Server,
};
// use std::convert::Infallible;
use tera::Tera;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", routing::get(root))
        .route("/about", routing::get(about_page))
        .fallback(not_found);
    println!("Listening on http://localhost:3000");
    Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn common_context() -> tera::Context {
    let mut context = tera::Context::new();
    context.insert("title", "axum-tera");
    context
}

async fn root() -> Html<String> {
    let tera = Tera::new("frontend/*.html").unwrap();

    let mut context = common_context();
    context.insert("page_title", "Index");
    context.insert("message", "indexページでのメッセージです。");

    let output = tera.render("index.html", &context);
    Html(output.unwrap())
}

async fn about_page() -> Html<String> {
    let tera = Tera::new("frontend/pages/*.html").unwrap();

    let mut context = common_context();
    context.insert("page_title", "About");
    context.insert("message", "aboutページでのメッセージです。");
    let output = tera.render("about.html", &context);
    Html(output.unwrap())
}

async fn not_found() -> impl IntoResponse {
    let tera = Tera::new("frontend/pages/*.html").unwrap();
    let mut context = common_context();
    context.insert("page_title", "Not Found");
    let output = tera.render("not_found.html", &context);
    (StatusCode::NOT_FOUND, Html(output.unwrap()))
}
