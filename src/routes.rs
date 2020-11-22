use warp::{ self, Filter, Reply, Rejection };
use crate::handlers;
use serde::de::DeserializeOwned;
use std::convert::Infallible;
use crate::pool::AsyncPool;

pub fn customer_routes(pool: AsyncPool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    get_customer(pool.clone())
    // create_customer(pool.clone())
        .or(create_customer(pool.clone()))
        .or(update_customer(pool.clone()))
        .or(delete_customer(pool.clone()))
        .or(customers_list(pool.clone()))
}

fn customers_list(pool: AsyncPool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("customers")
        .and(warp::get())
        .and(with_db_access_manager(pool))
        .and_then(handlers::list_customers)
}

fn create_customer(pool: AsyncPool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("customers")
        .and(warp::post())
        .and(with_db_access_manager(pool))
        .and(with_json_body())
        .and_then(handlers::create_customer)
}

fn update_customer(pool: AsyncPool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("customers" / i64)
        .and(warp::put())
        .and(with_db_access_manager(pool))
        .and(with_json_body())
        .and_then(handlers::update_customer)
}

fn delete_customer(pool: AsyncPool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("customers" / i64)
        .and(warp::delete())
        .and(with_db_access_manager(pool))
        .and_then(handlers::delete_customer)
}

fn get_customer(pool: AsyncPool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("customers" / i64)
        .and(warp::get())
        .and(with_db_access_manager(pool))
        .and_then(handlers::get_customer)
}

fn with_db_access_manager(pool: AsyncPool) -> impl Filter<Extract = (AsyncPool,), Error = Infallible> + Clone {
    warp::any()
        .map(move || pool.clone())
}

// pub fn injectState<T: Clone + Sized + Send>(state: T) -> impl Filter<Extract = (T,), Error = Infallible> + Clone {
//     warp::any().map(move || state.clone())
// }

fn with_json_body<T: DeserializeOwned + Send>(
) -> impl Filter<Extract = (T,), Error = Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}