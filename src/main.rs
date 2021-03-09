mod routers;
use std::io;
use std::env;
use actix_web::{HttpServer, App, web, HttpResponse};


#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().ok();

    let address = format!("{}:{}",
                            env::var("APP_HOST").expect("variabel host perlu didefinisikan"),
                            env::var("APP_PORT").expect("variabel port perlu didefinisikan"));
    let server = HttpServer::new( || {
        App::new()
            .route("/", web::get().to(|| HttpResponse::Ok().body("Halaman Ini dikosongkan")))
            .configure(routers::config)
    }).bind(address).unwrap();

    println!("Server Running!");
    server.run().await
}