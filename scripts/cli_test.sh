#!/usr/bin/env bash

. ./scripts/lib.sh

set -e

base_command() {
  flag=$1

  cargo run -p cli "$flag" -- \
    --command=./scripts/command.sh \
    --cache-key-files "$CACHE_BUSTERS_DIR_PATH" \
    "$NESTED_DIR_PATH"
}

run_test_command() {
  local tmp_out
  tmp_out="$(mktemp)"

  base_command -q >"$tmp_out" 2>&1 || local exit_code=$?

  if [ -z $exit_code ]; then
    echo "===========Command Output================"
    cat "$tmp_out"
    echo "========================================="
    echo ""
  else
    # Rerun without quiet to show build errors
    base_command
  fi
}

main() {
  clear_state
  clear_test_files

  seed_test_files

  echo "Before modified cache"
  run_test_command
  print_state_files

  modify_cache_busters

  echo ""
  echo "After modified cache"
  run_test_command
  print_state_files
}

main
