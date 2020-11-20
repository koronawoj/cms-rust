use warp::{ self, Filter, Reply, Rejection, reject };
use crate::handlers;
use serde::de::DeserializeOwned;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
type PgPool = Pool<ConnectionManager<PgConnection>>;

use crate::data_access::DBAccessManager;
use crate::errors::{AppError, ErrorType};

pub fn customer_routes(pool: PgPool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    // get_customer(pool.clone())
    create_customer(pool.clone())
        // .or(create_customer(pool.clone()))
        .or(update_customer(pool.clone()))
        .or(delete_customer(pool.clone()))
        .or(customers_list(pool.clone()))
}

fn customers_list(pool: PgPool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("customers")
        .and(warp::get())
        .and(with_db_access_manager(pool))
        .and_then(handlers::list_customers)
}

fn create_customer(pool: PgPool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("customers")
        .and(warp::post())
        .and(with_db_access_manager(pool))
        .and(with_json_body())
        .and_then(handlers::create_customer)
}

fn update_customer(pool: PgPool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("customers" / i64)
        .and(warp::put())
        .and(with_db_access_manager(pool))
        .and(with_json_body())
        .and_then(handlers::update_customer)
}

fn delete_customer(pool: PgPool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("customers" / i64)
        .and(warp::delete())
        .and(with_db_access_manager(pool))
        .and_then(handlers::delete_customer)
}

fn with_db_access_manager(pool: PgPool) -> impl Filter<Extract = (DBAccessManager,), Error = Rejection> + Clone {
    warp::any()
        .map(move || pool.clone())
        .and_then(|pool: PgPool| async move {  match pool.get() {
            Ok(conn) => Ok(DBAccessManager::new(conn)),
            Err(err) => Err(reject::custom(
                AppError::new(format!("Error getting connection from pool: {}", err.to_string()).as_str(), ErrorType::Internal))
            ),
        }})
}

fn with_json_body<T: DeserializeOwned + Send>(
) -> impl Filter<Extract = (T,), Error = Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}


// fn get_customer(pool: PgPool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
//     warp::path!("customers" / String)
//         .and(warp::get())
//         .and(with_db_access_manager(pool))
//         .and_then(handlers::get_customer)
// }