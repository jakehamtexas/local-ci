#!/usr/bin/env bash

state_path=$1
if [ -s "$state_path" ]; then
  debug="Previous file content: $(cat $state_path)\nFile type: $(type $state_path)"
  echo "This script ran twice!" >"$state_path"
  echo "$debug"
else
  cp "./tests/script-fixtures/expected_not_to_run_twice" "$state_path"
fi
