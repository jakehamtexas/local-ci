use super::error::{Error, Result};
use common::canonicalized_path::CanonicalizedPath;
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
        .map(|path| path.read().map_err(Error::from))
        .collect();

    bytes
}

// TODO: (for both `target_file_path` and `cache_key_file_paths`)
//       Make sure to include the paths themselves in each file's
//       content bytes so that two files with different paths but the
//       same content are not addressable at the same key.
pub fn get_file_id(
    command: &str,
    target_file_path: &CanonicalizedPath,
    cache_key_file_paths: Option<&Rc<[CanonicalizedPath]>>,
) -> Result<FileId> {
    let command_hash = xxh3_64(command.as_bytes());
    let target_file_path_bytes = target_file_path.to_str().as_bytes();
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
        cache_key_file_contents_hash,
    ))
}
