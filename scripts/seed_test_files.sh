#!/usr/bin/env bash

extensions=(.js .ts .txt '')
base_path=./test_files

make_files() {
  local stem="$base_path${1:-}/file"

  for extension in "${extensions[@]}"; do
    path="$stem$extension"
    mkdir -p "$(dirname $path)"
    echo "$path" >$path
  done
}

make_files
make_files "/nested_dir"
