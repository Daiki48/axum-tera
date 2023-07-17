use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing, Router, Server,
};
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

fn tera_include() -> Tera {
    let tera = Tera::new("frontend/**/*").unwrap();
    tera
}

async fn root() -> Html<String> {
    let tera = tera_include();
    let mut context = common_context();
    context.insert("page_title", "Index");
    context.insert("message", "This is Index page.");

    let output = tera.render("index.html", &context);
    Html(output.unwrap())
}

async fn about_page() -> Html<String> {
    let tera = tera_include();
    let mut context = common_context();
    context.insert("page_title", "About");
    context.insert("message", "This is About page.");
    let output = tera.render("pages/about.html", &context);
    Html(output.unwrap())
}

async fn not_found() -> impl IntoResponse {
    let tera = tera_include();
    let mut context = common_context();
    context.insert("page_title", "Not Found");
    let output = tera.render("pages/not_found.html", &context);
    (StatusCode::NOT_FOUND, Html(output.unwrap()))
}
