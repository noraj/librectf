use std::sync::Arc;

use failure::Error;

use super::db::DbConn;
use core::Pool;

struct InnerState {
    pub(super) secret_key: Vec<u8>,
    pub(super) pool: Pool,
}

#[derive(Clone)]
pub struct State {
    inner: Arc<InnerState>,
}

impl State {
    pub fn new(secret_key: Vec<u8>, pool: Pool) -> State {
        let inner = InnerState { secret_key, pool };
        let inner = Arc::new(inner);
        State { inner }
    }

    pub fn get_secret_key(&self) -> Vec<u8> {
        self.inner.secret_key.clone()
    }

    pub fn get_connection(&self) -> Result<DbConn, Error> {
        match self.inner.pool.get() {
            Ok(conn) => Ok(DbConn::new(conn)),
            Err(err) => Err(format_err!("Database connection error: {}", err)),
        }
    }
}
