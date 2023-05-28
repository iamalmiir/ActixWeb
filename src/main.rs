use actix::SyncArbiter;
use actix_web::{web::Data, App, HttpServer};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use dotenv::dotenv;
use std::env;

mod actors {
    pub mod task;
    pub mod user;
}
mod utils;
mod insertable;
mod messages;
mod models;
mod schema;
mod services;

use utils::{get_pool, AppState, DbActor};
use services::{create_task, create_user, fetch_tasks, fetch_users};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set to continue.");
    let pool: Pool<ConnectionManager<PgConnection>> = get_pool(&db_url);
    let db_addr = SyncArbiter::start(5, move || DbActor(pool.clone()));

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
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
