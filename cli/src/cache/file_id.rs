pub use crate::cache::error::{Error, Result};
use crate::canonicalized_path::CanonicalizedPath;
use std::fs::{self};
use std::rc::Rc;
use xxhash_rust::xxh3::xxh3_64;

pub struct FileId {
    pub command_id: String,
    pub target_file_path_id: String,
    pub cache_key_file_contents_id: String,
}

impl FileId {
    pub fn new(
        command_hash: u64,
        target_file_path_hash: u64,
        cache_key_file_contents_hash: u64,
    ) -> Self {
        fn pad_left(bytes: u64) -> String {
            format!("{:0>10}", bytes.to_string())
        }

        FileId {
            command_id: pad_left(command_hash),
            target_file_path_id: pad_left(target_file_path_hash),
            cache_key_file_contents_id: pad_left(cache_key_file_contents_hash),
        }
    }
}

fn get_cache_key_file_content_bytes(
    cache_key_file_paths: &Rc<[CanonicalizedPath]>,
) -> Result<Vec<Vec<u8>>> {
    let bytes: Result<Vec<Vec<u8>>> = cache_key_file_paths
        .iter()
        .map(|path| fs::read(&path.value).map_err(Error::from))
        .collect();

    bytes
}

pub fn get_file_id(
    command: &str,
    target_file_path: &CanonicalizedPath,
    cache_key_file_paths: Option<&Rc<[CanonicalizedPath]>>,
) -> Result<FileId> {
    let command_hash = xxh3_64(command.as_bytes());
    let target_file_path_bytes = target_file_path
        .value
        .to_str()
        .ok_or(Error::other(
            "Canonical paths are supposed to be coercible to str",
        ))?
        .as_bytes();
    let target_file_path_hash = xxh3_64(target_file_path_bytes);

    let cache_key_file_contents_hash = cache_key_file_paths
        .map(|paths| match get_cache_key_file_content_bytes(paths) {
            Ok(bytes) => Ok(xxh3_64(
                bytes.into_iter().flatten().collect::<Vec<_>>().as_slice(),
            )),
            Err(Error::IoNotFound) => Ok(u64::MIN),
            Err(e) => Err(e),
        })
        .unwrap_or(Ok(u64::MIN))?;

    Ok(FileId::new(
        command_hash,
        target_file_path_hash,
        cache_key_file_contents_hash.clone(),
    ))
}
