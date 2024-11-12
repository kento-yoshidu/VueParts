use actix_web::{get, post, web::{self, get, resource}, App, HttpResponse, HttpServer, Responder};
use std::env;
use dotenv::dotenv;

use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize, serde::Serialize)]
struct Article {
    id: String,
    title: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct ResponseData {
    contents: Vec<Article>,
    totalCount: usize,
    offset: usize,
    limit: usize,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    let test = env::var("TEST").expect("Not found environment variable.");

    let res = format!("{} {}", req_body, test);
    HttpResponse::Ok().body(res)
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

          // レスポンスのステータスコードをチェック
    if response.status().is_success() {
        // JSONをパースして、ベクトルとして取得
        let response: ResponseData = response.json().await.expect("Failed to parse JSON");
        println!("{:?}", response);
        HttpResponse::Ok().json(response.contents) // JSON形式でレスポンスを返す
    } else {
        HttpResponse::InternalServerError().body("Failed to fetch articles")
    }
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(get_articles)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
