use tokio::sync::{Semaphore, OwnedSemaphorePermit};
use std::sync::Arc;
use diesel::pg::PgConnection;

use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPoolConnection = PooledConnection<ConnectionManager<PgConnection>>;


#[derive(Clone)]
pub struct AsyncPool {
    pool: PgPool,
    semaphore: Arc<Semaphore>,
}

impl AsyncPool {
    pub fn new(pool: PgPool, max_size: usize) -> AsyncPool {
        AsyncPool {
            pool,
            semaphore: Arc::new(Semaphore::new(max_size))
        }
    }

    pub async fn get_pool(&self) -> AsyncPoolConnection {
        let lock = self.semaphore.clone().acquire_owned().await;

        let pool = self.pool.clone();
    
        let conn = tokio::task::spawn_blocking(move || {
            let result = pool.get().unwrap();
            result
        }).await.unwrap();

        AsyncPoolConnection {
            _lock: lock,
            conn
        }
    }

    pub async fn get<R: Send + 'static, F: Send + 'static>(&self, exec: F) -> R where F: FnOnce(&PgConnection) -> R {
        let connection = self.get_pool().await;
        connection.deref(exec).await
    }
}

pub struct AsyncPoolConnection {
    _lock: OwnedSemaphorePermit,
    conn: PgPoolConnection,
}

impl AsyncPoolConnection {
    pub async fn deref<R: Send + 'static, F: Send + 'static>(self, exec: F) -> R where F: FnOnce(&PgConnection) -> R {
        
        let exec_boxed = Box::new(exec);

        let result = tokio::task::spawn_blocking(move || {

            let connection = &*self.conn;
            let result = exec_boxed(connection);

            result
        }).await.unwrap();

        result
    }
}