use actix_web::{get, post, web::{self, get, resource}, App, HttpResponse, HttpServer, Responder};
use std::env;
use dotenv::dotenv;
use reqwest::Client;
use serde::Deserialize;
use tera::{Tera, Context};
// 静的ファイルの取得
use actix_files::Files;

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
async fn get_articles(tera: web::Data<Tera>) -> impl Responder {
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

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // PORT環境変数を取得 (デフォルトは8080)
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let port: u16 = port.parse().expect("PORT must be a valid u16 number");

    // Teraの初期化
    let tera = Tera::new("templates/**/*").expect("Failed to initialize Tera");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tera.clone()))
            .service(get_articles)
            .service(
                // 静的ファイル読み込み
                web::scope("/static").service(Files::new("/", "./static"))
            )
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
