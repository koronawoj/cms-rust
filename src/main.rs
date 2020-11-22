#[macro_use]
extern crate diesel;

mod pool;
mod errors;
mod data_access;
mod schema;
mod models;
mod handlers;
mod routes;

use std::env;
use warp::{Filter};
use log::{info};

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Builder};
use crate::errors::{AppError};
use crate::pool::AsyncPool;

use dotenv::dotenv;

fn pg_pool(db_url: &str) -> AsyncPool {
    let manager = ConnectionManager::<PgConnection>::new(db_url);

    let pool = Builder::new()
        .max_size(10)
        .build(manager).unwrap();
    
    // let manager = ConnectionManager::<PgConnection>::new(db_url);
    // let pool = Pool::new(manager).expect("Postgres connection pool could not be created");

    AsyncPool::new(pool, 10)
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL env not set");

    let pg_pool = pg_pool(database_url.as_str());

    // let routes = api_filters(pg_pool)
    //     .recover(errors::handle_rejection);

    let customer_routes = routes::customer_routes(pg_pool)
        .recover(errors::handle_rejection);


    info!("Starting server on port 3030...");

    // Start up the server...
    warp::serve(customer_routes).run(([127, 0, 0, 1], 3000)).await;

    // let db = db::init_db();
    // let customer_routes = routes::customer_routes(db);

    // warp::serve(customer_routes)
    //     .run(([127, 0, 0, 1], 3000))
    //     .await;
}