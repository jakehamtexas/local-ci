use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;

use common::ReadonlyList;

// TODO: Replace this with a lib
pub fn parse(files: &[String]) -> ReadonlyList<PathBuf> {
    files
        .iter()
        .map(Path::new)
        .filter(|path| path.is_file() || path.is_dir())
        .flat_map(get_files)
        .collect::<Rc<_>>()
}

fn get_files(path: &Path) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = Vec::new();
    match (path.is_dir(), path.is_file(), path.is_symlink()) {
        (true, _, _) => files.append(&mut get_dir_files(path)),
        (_, true, _) => files.push(path.to_owned()),
        (_, _, true) => (),
        _ => unreachable!(),
    }

    files
}

fn get_dir_files(path: &Path) -> Vec<PathBuf> {
    let paths = fs::read_dir(path)
        .map(|entries| {
            entries
                .filter_map(|result| result.map(|entry| entry.path()).ok())
                .collect::<Vec<_>>()
        })
        .unwrap_or_else(|_| vec![]);

    let mut files: Vec<PathBuf> = Vec::new();
    for path in paths {
        files.append(&mut get_files(path.as_path()));
    }

    files
}
