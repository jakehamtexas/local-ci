use std::sync::Arc;
use thiserror::Error;

struct MaybeInner<'a, T> {
    t: Option<&'a T>,
    serde: Option<Arc<serde_json::Error>>,
}

#[derive(Debug, Error, Clone)]
pub enum CacheReadError<T: PartialEq> {
    #[error(transparent)]
    Other(Box<T>),
    #[error("Deserialization error: {:?}", 0)]
    Deserialization(#[source] Arc<serde_json::Error>),
}

impl<T: PartialEq> From<serde_json::Error> for CacheReadError<T> {
    fn from(value: serde_json::Error) -> Self {
        CacheReadError::Deserialization(Arc::new(value))
    }
}

fn get_comparable<T: PartialEq>(pair: MaybeInner<T>) -> Result<&T, serde_json::error::Category> {
    match pair {
        MaybeInner {
            t: Some(t),
            serde: None,
        } => Ok(t),
        MaybeInner {
            t: None,
            serde: Some(e),
        } => Err(e.classify()),
        _ => unreachable!(),
    }
}

fn eq_inner<'a, PE, T>(first: &'a T, second: &'a T) -> bool
where
    PE: PartialEq + 'a,
    &'a T: Into<MaybeInner<'a, PE>>,
{
    let first = get_comparable(first.into());
    let second = get_comparable(second.into());

    first.is_ok_and(|f| second.is_ok_and(|s| f == s))
        || first.is_err_and(|f| second.is_err_and(|s| f == s))
}

impl<T: PartialEq> PartialEq for CacheReadError<T> {
    fn eq(&self, other: &Self) -> bool {
        eq_inner(self, other)
    }
}

#[derive(Debug, Error, Clone)]
pub enum CacheWriteError<T: PartialEq> {
    #[error(transparent)]
    Other(Box<T>),
    #[error("Serialization error: {:?}", 0)]
    Serialization(#[source] Arc<serde_json::Error>),
}

impl<T: PartialEq> From<serde_json::Error> for CacheWriteError<T> {
    fn from(value: serde_json::Error) -> Self {
        CacheWriteError::Serialization(Arc::new(value))
    }
}

impl<T: PartialEq> PartialEq for CacheWriteError<T> {
    fn eq(&self, other: &Self) -> bool {
        eq_inner(self, other)
    }
}

impl<'a, T: PartialEq> From<&'a CacheWriteError<T>> for MaybeInner<'a, T> {
    fn from(value: &'a CacheWriteError<T>) -> Self {
        match value {
            CacheWriteError::Other(t) => Self {
                t: Some(t),
                serde: None,
            },
            CacheWriteError::Serialization(e) => Self {
                t: None,
                serde: Some(Arc::clone(e)),
            },
        }
    }
}

impl<'a, T: PartialEq> From<&'a CacheReadError<T>> for MaybeInner<'a, T> {
    fn from(value: &'a CacheReadError<T>) -> Self {
        match value {
            CacheReadError::Other(t) => Self {
                t: Some(t),
                serde: None,
            },
            CacheReadError::Deserialization(e) => Self {
                t: None,
                serde: Some(Arc::clone(e)),
            },
        }
    }
}
