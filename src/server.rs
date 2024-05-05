use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

pub async fn server() -> std::io::Result<()> {
    let port = std::env::var("PORT").unwrap_or("4000".to_string());
    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run().await?;
    Ok(())
}