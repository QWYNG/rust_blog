use actix_web::{get, App, HttpResponse, HttpServer, ResponseError};
use askama::Template;
use anyhow::Result;
use chrono::{{DateTime, Utc}};

struct Article {
    id: u32,
    title: String,
    created_at: DateTime<Utc> 
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    articles: Vec<Article>,
}

#[get("/")]
async fn index() -> Result<HttpResponse, actix_web::Error> {
    let mut articles = Vec::new();

    articles.push(Article { id: 1,title: "first".to_string(), created_at: Utc::now()});

    let html = IndexTemplate { articles };
    let response_body = html.render().unwrap();

    Ok(HttpResponse::Ok().content_type("text/html").body(response_body))
}

#[actix_rt::main]
async fn main() -> Result<()> {
    HttpServer::new(move || App::new().service(index))
    .bind("0.0.0.0:8080")?
    .run()
    .await?;
    Ok(())
}

