use axum::{response::Html, routing, Router, Server};
use tera::Tera;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", routing::get(root));
    Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> Html<String> {
    let tera = Tera::new("templates/*").unwrap();

    let mut context = tera::Context::new();
    context.insert("title", "Indexページです。");
    context.insert("message", "メッセージです。");

    let output = tera.render("index.html", &context);
    Html(output.unwrap())
}
