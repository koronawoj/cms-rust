mod db;
mod models;
mod handlers;
mod routes;

#[tokio::main]
async fn main() {
    let db = db::init_db();
    let customer_routes = routes::customer_routes(db);

    warp::serve(customer_routes)
        .run(([127, 0, 0, 1], 3000))
        .await;
}
