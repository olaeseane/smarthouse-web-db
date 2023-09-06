use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;

use cserver::{
    api::{add_device, add_room, del_device, del_room, list_devices, list_rooms},
    db::initialize_db_pool,
};

pub const ADDR: &str = "127.0.0.1";
const PORT: u16 = 8080;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let pool = initialize_db_pool();

    log::info!("starting HTTP server at {ADDR}");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(list_rooms)
            .service(add_room)
            .service(del_room)
            .service(list_devices)
            .service(add_device)
            .service(del_device)
    })
    .bind((ADDR, PORT))?
    .run()
    .await
}
