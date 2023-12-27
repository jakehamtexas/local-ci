#!/usr/bin/env bash

filename=$1

file_contents="$(cat "$filename")"

echo "$filename: $file_contents"
