
// 必要なクレートを定義
use actix_web::{web, App, HttpServer, Responder};

// 非同期関数を定義
async fn home() -> impl Responder {
    "Hello,world!"
}

// メイン関数
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| 
        App::new()
        .route("/", web::get().to(home)))
        .bind("127.0.0.1:3000")?
        .run()
        .await
}