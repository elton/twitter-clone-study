use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use std::{env, io};

#[actix_web::main]
async fn main() -> io::Result<()> {
    // 初始化日志
    env::set_var("RUST_LOG", "actix_web=debug, actix_server=info");
    env_logger::init();
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(|| HttpResponse::Ok().body("Hey!")))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
