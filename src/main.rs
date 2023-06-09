use actix::SyncArbiter;
use actix_web::{web::Data, App, HttpServer};
use config::{Config, File, FileFormat};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use env_logger::Env;
use log::LevelFilter;

use services::{create_task, create_user, fetch_tasks, fetch_users};
use utils::{get_pool, AppState, DbActor};

mod actors {
    pub mod task;
    pub mod user;
}

mod insertable;
mod messages;
mod models;
mod schema;
mod services;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .filter_level(LevelFilter::Info)
        .init();

    let config = Config::builder()
        .add_source(File::new("config.toml", FileFormat::Toml))
        .build()
        .expect("Failed to build config");

    let pool: Pool<ConnectionManager<PgConnection>> = get_pool(
        &config
            .get_string("database.url")
            .expect("Failed to get database url"),
    );
    let db_addr: actix::Addr<DbActor> = SyncArbiter::start(5, move || DbActor(pool.clone()));
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState {
                db: db_addr.clone(),
            }))
            .service(fetch_tasks)
            .service(create_task)
            .service(fetch_users)
            .service(create_user)
    })
    .bind((
        config.get_string("server.host").expect("Host not set."),
        config.get_int("server.port").expect("Port not set.") as u16,
    ))?
    .run()
    .await
}
