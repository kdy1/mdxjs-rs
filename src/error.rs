use std::cell::RefCell;

use markdown::unist::Point;
use scoped_tls::scoped_thread_local;

#[derive(Debug, Clone, PartialEq)]
pub struct Error {
    pub kind: ErrorKind,
    pub point: Option<Point>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    Msg(String),
    Parser(swc_core::ecma::parser::error::Error),
    OnlyImportExport,
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Error {
            kind: ErrorKind::Msg(value),
            point: None,
        }
    }
}
impl From<&'_ str> for Error {
    fn from(value: &'_ str) -> Self {
        Self::from(value.to_string())
    }
}

impl From<swc_core::ecma::parser::error::Error> for Error {
    fn from(value: swc_core::ecma::parser::error::Error) -> Self {
        Error {
            kind: ErrorKind::Parser(value),
            point: None,
        }
    }
}

scoped_thread_local!(static ERROR: RefCell<Option<Error>>);

pub(crate) fn capture<F, T, E>(op: F) -> Result<T, Error>
where
    F: FnOnce() -> Result<T, E>,
    Error: From<E>,
{
    let error = RefCell::default();

    let result = ERROR.set(&error, || match op() {
        Ok(value) => Ok(value),
        Err(err) => Err(err),
    });

    match RefCell::into_inner(error) {
        Some(err) => return Err(err),
        None => Ok(result?),
    }
}

pub(crate) fn set_error(error: Error) {
    ERROR.with(|e| {
        *e.borrow_mut() = Some(error);
    });
}
