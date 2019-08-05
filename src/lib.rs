use std::io::{Error, ErrorKind};
use failure::Fail;
use actix::{Actor, Addr};


// Translate errors into useful kind
pub fn other<E>(err: E) -> Error
where
    E: Into<Box<std::error::Error + Send + Sync>>,
{
    Error::new(ErrorKind::Other, err)
}
