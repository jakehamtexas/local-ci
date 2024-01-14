mod util;
use std::str::FromStr;

use crate::util::FakeFileIdBuilder;
use common::{prelude::FsFacade, RelativePath};
use std::sync::Arc;

use assert_fs::{prelude::*, TempDir};
use common::ReadonlyList;
use rstest::*;
use util::ConfigBuilder;

#[fixture]
fn tmp_dir() -> TempDir {
    TempDir::new().unwrap()
}

#[rstest]
fn expect_state_dir_to_exist(tmp_dir: TempDir) {
    let state_dir = tmp_dir.join("cache");
    let path = tmp_dir.child("file");
    path.touch().unwrap();

    let args = ConfigBuilder::new(&tmp_dir)
        .with_state_dir(&state_dir)
        .with_path(&path)
        .build();
    let res = work_skipping::run(args).map(|_| {
        tmp_dir.child("cache").assert(predicates::path::is_dir());
    });

    assert_eq!(Ok(()), res);
}

#[rstest]
fn expect_state_dir_to_have_command_cache(tmp_dir: TempDir) {
    let state_dir = tmp_dir.join("cache");
    let path = tmp_dir.child("file");
    path.touch().unwrap();

    let command = "echo";
    let args = ConfigBuilder::new(&tmp_dir)
        .with_state_dir(&state_dir)
        .with_path(&path)
        .with_command(command)
        .build();

    let file_id = FakeFileIdBuilder::default().with_command(command).build();
    let res = work_skipping::run(args).map(|_| {
        tmp_dir
            .child("cache")
            .child(file_id.command_id)
            .assert(predicates::path::is_dir());
    });

    assert_eq!(Ok(()), res);
}

#[rstest]
fn expect_state_dir_to_have_state_cache(tmp_dir: TempDir) {
    let state_dir = tmp_dir.join("cache");
    let path = tmp_dir.child("file");
    path.touch().unwrap();

    let command = "echo";
    let args = ConfigBuilder::new(&tmp_dir)
        .with_state_dir(&state_dir)
        .with_path(&path)
        .with_command(command)
        .build();
    let file_id = FakeFileIdBuilder::default()
        .with_command(command)
        .with_path(&path.path().as_os_str())
        .build();
    let res = work_skipping::run(args).map(|_| {
        tmp_dir
            .child("cache")
            .child(file_id.command_id)
            .child(file_id.cache_key_file_contents_id)
            .child(file_id.target_file_path_id)
            .assert(predicates::path::is_file());
    });

    assert_eq!(Ok(()), res);
}

#[rstest]
fn expect_state_cache_to_use_cache_keys(tmp_dir: TempDir) {
    let state_dir = tmp_dir.join("cache");
    let path = tmp_dir.child("file");
    path.touch().unwrap();

    let mut cache_key_files: Vec<RelativePath> = vec![];
    let cache_keys_dir = tmp_dir.child("keys");
    cache_keys_dir.create_dir_all().unwrap();
    for n in 0..3 {
        let file = cache_keys_dir.child(n.to_string());

        let rel_path = RelativePath::new(&file);
        cache_key_files.push(rel_path);

        file.write_str(&n.to_string()).unwrap();
    }

    let cache_key_files = cache_key_files.into_iter().collect::<ReadonlyList<_>>();
    let cache_key_path_bufs = cache_key_files
        .iter()
        .map(|f| f.to_path_buf())
        .collect::<ReadonlyList<_>>();

    let test_state_path = tmp_dir.child("state.txt");
    test_state_path.touch().unwrap();
    let command = format!(
        "./tests/script-fixtures/should_run_only_once.sh {}",
        &test_state_path.path().display()
    );
    let args = ConfigBuilder::new(&tmp_dir)
        .with_state_dir(&state_dir)
        .with_path(&path)
        .with_command(&command)
        .with_cache_key_files(&cache_key_path_bufs)
        .build();
    let res = work_skipping::run(Arc::clone(&args))
        .and_then(|_| work_skipping::run(Arc::clone(&args)))
        .map(|_| {
            let test_state_file_content = RelativePath::new(test_state_path.path()).read().unwrap();
            let test_state_file_content = String::from_utf8(test_state_file_content).unwrap();
            let expected =
                RelativePath::from_str("./tests/script-fixtures/expected_not_to_run_twice")
                    .unwrap()
                    .read()
                    .unwrap();
            let expected = String::from_utf8(expected.into()).unwrap();
            assert_eq!(expected, test_state_file_content)
        });

    assert_eq!(Ok(()), res);
}
