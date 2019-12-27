use actix_web::{web, App, HttpServer, Responder, HttpResponse};

async fn index(info: web::Path<(String, u32)>) -> impl Responder {
    format!("Hello {}! id:{}", info.0, info.1)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||
        // App::new().service(
        //     web::resource("/{name}/{id}/index.html").to(index)
        // )
        // .default_service(
        //     web::to(|| HttpResponse::NotFound())
        // )
        App::new()
            .route("/test", web::get().to(index))
            .route("/test", web::post().to(index))
            .route("/hello", web::get().to(index))
    )
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
