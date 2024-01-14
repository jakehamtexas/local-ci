use std::ffi::OsStr;

use common::prelude::*;
use common::FileHandleResult;
use common::ReadonlyList;
use common::RelativePath;
use xxhash_rust::xxh3::xxh3_64;

pub struct FileId {
    pub command_id: String,
    pub target_file_path_id: String,
    pub cache_key_file_contents_id: String,
}

fn to_resized_bytes(input: &[u64]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(8 * input.len());

    for value in input {
        bytes.extend(&value.to_be_bytes());
    }

    bytes
}

impl FileId {
    // TODO: (for both `target_file_path` and `cache_key_file_paths`)
    //       Make sure to include the paths themselves in each file's
    //       content bytes so that two files with different paths but the
    //       same content are not addressable at the same key.
    pub fn new(
        command: &str,
        target_file_path: &OsStr,
        cache_key_file_paths: Option<&ReadonlyList<RelativePath>>,
    ) -> FileHandleResult<Self> {
        let command_hash = xxh3_64(command.as_bytes());
        let target_file_path_bytes = target_file_path.as_encoded_bytes();
        let target_file_path_hash = xxh3_64(target_file_path_bytes);

        let cache_key_file_contents_hash = cache_key_file_paths
            .map(|paths| {
                paths
                    .iter()
                    .map(|path| match path.read() {
                        Ok(bytes) => Ok(xxh3_64(&bytes)),
                        Err(e) => match e.inner().source().kind() {
                            std::io::ErrorKind::NotFound => Ok(u64::MIN),
                            _ => Err(e),
                        },
                    })
                    .collect::<FileHandleResult<Vec<u64>>>()
                    .map(|bytes| to_resized_bytes(&bytes))
                    .map(|bytes| xxh3_64(&bytes))
            })
            .transpose()?
            .unwrap_or(u64::MIN);
        fn pad_left(bytes: u64) -> String {
            format!("{:0>10}", bytes.to_string())
        }

        Ok(FileId {
            command_id: pad_left(command_hash),
            target_file_path_id: pad_left(target_file_path_hash),
            cache_key_file_contents_id: pad_left(cache_key_file_contents_hash),
        })
    }
}
