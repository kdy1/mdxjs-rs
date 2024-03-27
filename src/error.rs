use std::{cell::RefCell, fmt::Display};

use markdown::unist::Point;
use scoped_tls::scoped_thread_local;
use swc_core::common::Span;

#[derive(Debug, Clone, PartialEq)]
pub struct Error {
    pub kind: ErrorKind,
    pub point: Option<Point>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    JsxSpreadNotSupported,

    // Msg(String),
    Parser(Span, swc_core::ecma::parser::error::Error),
    OnlyImportExport(Span),
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Error { kind, point: None }
    }
}

// impl From<String> for Error {
//     fn from(value: String) -> Self {
//         Error {
//             kind: ErrorKind::Msg(value),
//             point: None,
//         }
//     }
// }
// impl From<&'_ str> for Error {
//     fn from(value: &'_ str) -> Self {
//         Self::from(value.to_string())
//     }
// }

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(point) = &self.point {
            write!(f, "{}:{} ", point.line, point.column)?;
        } else {
            write!(f, "0:0 ")?;
        }

        match self.kind {
            ErrorKind::JsxSpreadNotSupported => write!(
                f,
                "Unexpected spread child, which is not supported in Babel, SWC, or React"
            )?,
            ErrorKind::Parser(_, err) => write!(f, "{}", err.kind().msg())?,
            ErrorKind::OnlyImportExport(..) => write!(f, "Only import and export are supported")?,
        }
        Ok(())
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
