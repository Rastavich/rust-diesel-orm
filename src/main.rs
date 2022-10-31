use actix::SyncArbiter;
use actix_web::{web::Data, App, HttpServer};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use dotenv::dotenv;
use std::env;

mod actors;
mod db_models;
mod db_utils;
mod insertables;
mod messages;
mod schema;
mod services;

use db_utils::{get_pool, AppState, DbActor};
use services::{fetch_meetings, fetch_team_member, fetch_team_members, fetch_talking_points};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool: Pool<ConnectionManager<PgConnection>> = get_pool(&db_url);
    let db_addr = SyncArbiter::start(5, move || DbActor(pool.clone()));

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState {
                db: db_addr.clone(),
            }))
            .service(fetch_team_members)
            .service(fetch_team_member)
            .service(fetch_meetings)
            .service(fetch_talking_points)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
