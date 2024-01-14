use thiserror::Error;

#[derive(Clone, Error)]
pub struct OpaqueError<E = Box<dyn std::error::Error>> {
    debug: String,
    display: String,
    original: Option<E>,
}

impl<E: std::error::Error> OpaqueError<E> {
    pub fn from_error(e: E) -> Self {
        OpaqueError {
            debug: format!("{:?}", e),
            display: format!("{}", e),
            original: Some(e),
        }
    }
}

impl<E> OpaqueError<E> {
    fn new<Debug, Display>(debug: Debug, display: Display, original: Option<E>) -> Self
    where
        Debug: std::fmt::Debug,
        Display: std::fmt::Display,
    {
        OpaqueError {
            debug: format!("{:?}", debug),
            display: format!("{}", display),
            original,
        }
    }

    pub fn from_bespoke<Debug, Display>(debug: Debug, display: Display) -> Self
    where
        Debug: std::fmt::Debug,
        Display: std::fmt::Display,
    {
        Self::new(debug, display, None)
    }

    pub fn into_inner(self) -> Option<E> {
        self.original
    }
}

impl<E> std::fmt::Debug for OpaqueError<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.debug)
    }
}

impl<E> std::fmt::Display for OpaqueError<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display)
    }
}

impl From<std::io::Error> for OpaqueError<std::io::Error> {
    fn from(value: std::io::Error) -> Self {
        OpaqueError::from_error(value)
    }
}
