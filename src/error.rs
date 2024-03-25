use std::cell::RefCell;

use scoped_tls::scoped_thread_local;

#[derive(Debug, Clone)]
pub enum Error {
    Msg(String),
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Error::Msg(value)
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
