use actix::SyncArbiter;
use actix_session::{storage::RedisSessionStore, Session, SessionMiddleware};
use actix_web::cookie::Key;
use actix_web::{get, web::Data, App, Error, HttpServer, Result};
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
mod db_models;
mod db_utils;
mod insertable;
mod messages;
mod schema;
mod services;

use db_utils::{get_pool, AppState, DbActor};
use services::{create_task, create_user, fetch_tasks, fetch_users};

#[get("/")]
async fn index(session: Session) -> Result<&'static str, Error> {
    if let Some(count) = session.get::<i32>("counter")? {
        println!("SESSION value: {}", count);
        session.insert("counter", count + 1)?;
    } else {
        session.insert("counter", 1)?;
    }

    Ok("Welcome!")
}

fn get_secret_key() -> Key {
    Key::generate()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let secret_key = get_secret_key();
    let redis_connection_string =
        env::var("REDIS_CONNECTION_STRING").expect("REDIS_CONNECTION_STRING must be set");
    let store = RedisSessionStore::new(redis_connection_string)
        .await
        .unwrap();
    let db_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set to continue.");
    let pool: Pool<ConnectionManager<PgConnection>> = get_pool(&db_url);
    let db_addr = SyncArbiter::start(5, move || DbActor(pool.clone()));

    HttpServer::new(move || {
        App::new()
            .wrap(SessionMiddleware::new(store.clone(), secret_key.clone()))
            .app_data(Data::new(AppState {
                db: db_addr.clone(),
            }))
            .service(fetch_tasks)
            .service(create_task)
            .service(create_user)
            .service(fetch_users)
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
