use actix_files as fs;
use actix_web::{web, App, HttpServer, Responder};
use homedir::my_home;
use std::fs::read_dir;
use std::path::PathBuf;

async fn list_files(path: web::Path<String>) -> impl Responder {
    let mut base_path = PathBuf::from(".");
    let home = my_home().unwrap().unwrap();
    let home_path = home.as_os_str();
    println!("home {:?}", home_path);

    base_path.push(home_path);

    let mut entries = vec![];
    if let Ok(read_dir) = read_dir(base_path) {
        for entry in read_dir.flatten() {
            let item = entry.file_name().as_os_str().to_str().unwrap().to_string();

            if !item.starts_with('.') {
                entries.push(item);
            }
        }
    }
    web::Json(entries)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/list/{path:.*}", web::get().to(list_files))
            .service(fs::Files::new("/", "./static").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
