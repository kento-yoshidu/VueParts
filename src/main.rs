use actix_web::{get, web::{self, Data}, App, HttpResponse, HttpServer, Responder};
use std::env;
use dotenv::dotenv;
use reqwest::{Client, Error as ReqwestError};
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

#[derive(Debug, serde::Serialize)]
struct ArticlesResponse {
    articles: Vec<Article>,
    total_count: usize,
}

#[derive(Clone)]
struct AppState {
    client: Client,
    api_key: String,
    endpoint: String,
}

async fn fetch_from_microcms<T: for<'de> Deserialize<'de>>(
    state: &AppState,
    path: Option<&str>,
) -> Result<T, ReqwestError> {
    let url = match path {
        Some(p) => format!("{}/{}", state.endpoint, p),
        None => state.endpoint.clone(),
    };

    let response = state
        .client
        .get(&url)
        .header("X-MICROCMS-API-KEY", &state.api_key)
        .send()
        .await?;

    response.json::<T>().await
}

#[get("/articles")]
async fn get_articles(state: Data<AppState>) -> impl Responder {
    match fetch_from_microcms::<ResponseData>(&state, None).await {
        Ok(data) => {
            let articles_data = ArticlesResponse {
                articles: data.contents,
                total_count: data.totalCount,
            };

            HttpResponse::Ok().json(articles_data)
        },
        Err(_) => HttpResponse::InternalServerError().body("Failed to fetch articles"),
    }
}

#[get("/articles/{id}")]
async fn get_article(state: Data<AppState>, path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();

    match fetch_from_microcms::<Article>(&state, Some(&id)).await {
        Ok(article) => HttpResponse::Ok().json(article),
        Err(err) => {
            eprintln!("Error fetching article with id {}: {:?}", id, err);
            HttpResponse::InternalServerError().body("Failed to fetch article")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let client = Client::new();
    let api_key = env::var("MICROCMS_API_KEY").expect("MICROCMS_API_KEY must be set");
    let endpoint = env::var("MICROCMS_ENDPOINT").expect("MICROCMS_ENDPOINT must be set");

    let state = AppState {
        client,
        api_key,
        endpoint,
    };

    // PORT環境変数を取得 (デフォルトは8080)
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let port: u16 = port.parse().expect("PORT must be a valid u16 number");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(state.clone()))
            .service(get_articles)
            .service(get_article)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
