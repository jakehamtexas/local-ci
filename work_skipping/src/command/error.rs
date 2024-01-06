#[derive(Debug)]
pub enum Error {
    CreationFailed,
    OutputIo(std::io::Error),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
