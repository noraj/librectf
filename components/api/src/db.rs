use std::ops::Deref;

use actix_web::{FromRequest, HttpRequest};
use core::db::{Connection, PooledConnection};
use failure::Error;

use super::State;

pub struct DbConn(Connection);

impl DbConn {
    pub fn new(conn: PooledConnection) -> Self {
        DbConn(Connection(conn))
    }
}

impl Deref for DbConn {
    type Target = <Connection as Deref>::Target;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl FromRequest<State> for DbConn {
    type Config = ();
    type Result = Result<Self, Error>;

    #[inline]
    fn from_request(req: &HttpRequest<State>, _: &Self::Config) -> Self::Result {
        req.state().get_connection()
    }
}
