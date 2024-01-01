
#!/usr/bin/env bash

check_command() {
  echo "$1"
}

export STATE_PATH=./.local-ci
export TEST_FILES_BASE_PATH=./test_files
export CACHE_BUSTERS_DIR_PATH="$TEST_FILES_BASE_PATH"/cache_busters
export NESTED_DIR_PATH="$TEST_FILES_BASE_PATH"/nested_dir

modify_cache_busters() {
  echo "TEST: Modifying test file contents with append"
  for file in $(find "$CACHE_BUSTERS_DIR_PATH" -type f); do
    echo "mod" >> $file
  done
}

seed_test_files() {
  echo "TEST: Seeding test files"
  local extensions=(.js .ts .txt '')

  make_files() {
    for extension in "${extensions[@]}"; do
      local path="$TEST_FILES_BASE_PATH/$1/file$extension"
      mkdir -p "$(dirname $path)"
      echo "$path" >"$path"
    done
  }

  make_files "nested_dir"
  make_files "cache_busters"
}

clear_test_files() {
  rm -rf "$TEST_FILES_BASE_PATH" || true
}

clear_state() {
  rm -rf "$STATE_PATH" || true
}

print_state_files() {
  echo "===============State Files==============="
  find ./.local-ci
  echo "========================================="
  echo ""
}
