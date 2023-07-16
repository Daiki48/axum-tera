use axum::{response::Html, routing, Router, Server};
use tera::Tera;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", routing::get(root))
        .route("/about", routing::get(about_page));
    Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> Html<String> {
    let tera = Tera::new("templates/*.html").unwrap();

    let mut context = tera::Context::new();
    context.insert("title", "axum-tera");
    context.insert("page_title", "Index");
    context.insert("message", "indexページでのメッセージです。");

    let output = tera.render("index.html", &context);
    Html(output.unwrap())
}

async fn about_page() -> Html<String> {
    let tera = Tera::new("templates/pages/*.html").unwrap();

    let mut context = tera::Context::new();
    context.insert("title", "axum-tera");
    context.insert("page_title", "About");
    context.insert("message", "aboutページでのメッセージです。");
    let output = tera.render("about.html", &context);
    Html(output.unwrap())
}
