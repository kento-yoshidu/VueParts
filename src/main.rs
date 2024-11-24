use actix_web::{get, web::{self}, App, HttpResponse, HttpServer, Responder};
use std::env;
use dotenv::dotenv;
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize, serde::Serialize)]
struct Tag {
    name: String,
    slug: String,
}

#[derive(Debug, Deserialize, serde::Serialize)]
struct Article {
    id: String,
    title: String,
    content: String,
    tags: Vec<Tag>,
}

#[derive(Debug, Deserialize)]
struct ResponseData {
    contents: Vec<Article>,
    totalCount: usize,
    offset: usize,
    limit: usize,
}

#[get("/articles")]
async fn get_articles() -> impl Responder {
    let client = Client::new();

    let api_key = env::var("MICROCMS_API_KEY").expect("MICROCMS_API_KEY must be set");
    let endpoint = env::var("MICROCMS_ENDPOINT").expect("MICROCMS_ENDPOINT must be set");

    let response = client
        .get(&endpoint)
        .header("X-MICROCMS-API-KEY", api_key)
        .send()
        .await
        .expect("Failed to send request");

    if response.status().is_success() {
        let response_data: ResponseData = response.json().await.expect("Failed to parse JSON");

        HttpResponse::Ok().json(response_data.contents)
    } else {
        HttpResponse::InternalServerError().body("Failed to fetch articles")
    }
}

#[get("/articles/{id}")]
async fn get_article(path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();

    let client = reqwest::Client::new();
    let api_key = env::var("MICROCMS_API_KEY").expect("MICROCMS_API_KEY must be set");
    let endpoint = env::var("MICROCMS_ENDPOINT").expect("MICROCMS_ENDPOINT must be set");

    let url = format!("{}/{}", endpoint, id);

    let response = client
        .get(url)
        .header("X-API-KEY", api_key)
        .send()
        .await
        .expect("Failed to send request");

    if response.status().is_success() {
        let response_data: Article = response.json().await.expect("Failed to parse JSON");

        HttpResponse::Ok().json(response_data)
    } else {
        HttpResponse::InternalServerError().body("Failed to fetch articles")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // PORT環境変数を取得 (デフォルトは8080)
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let port: u16 = port.parse().expect("PORT must be a valid u16 number");

    HttpServer::new(move || {
        App::new()
            .service(get_articles)
            .service(get_article)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
