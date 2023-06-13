use actix_files::Files;
use actix_web::{get, App, HttpResponse, HttpServer, Responder, web};
use serde::Serialize;
use actix_files::NamedFile;
use std::path::PathBuf;

#[derive(Serialize)]
struct MyData {
    field1: String,
    field2: i32,
}

#[get("/data")]
async fn data() -> impl Responder {
    let my_data = MyData {
        field1: "Hello".to_string(),
        field2: 42,
    };
    HttpResponse::Ok().json(my_data)
}



async fn somepage() -> impl Responder {
    NamedFile::open(PathBuf::from("./static/index.html")).unwrap()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(data)
            .service(web::resource("/dashboard").route(web::get().to(somepage)))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
