use actix_cors::Cors;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use reqwest;

#[allow(unused_must_use)]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        println!("Server is running on port {}", get_port());
        let cors = Cors::default().allow_any_origin();
        App::new()
            .wrap(cors)
            .route("/", web::get().to(|| HttpResponse::Ok().body("Usage: GET /URL")))
            .route("/{path:.+}", web::get().to(cors_proxy_get))
    })
    .bind(("0.0.0.0", get_port()))?
    .run()
    .await;

    Ok(())
}

async fn cors_proxy_get(request: HttpRequest) -> impl Responder {
    // Removing first slash from relative path
    let request_path = &request.path()[1..request.path().len()];
    let url = format!("{}?{}", request_path, request.query_string());

    // TODO: make it better
    println!("Requesting url: {}", &url);
    reqwest::get(url)
        .await
        .expect("Some wrong url or server or client")
        .text()
        .await
        .expect("response.text is wrong")
}

fn get_port() -> u16 {
    std::env::var("PORT")
        .unwrap_or("8000".to_string())
        .parse()
        .unwrap()
}
