#!/usr/bin/env bash

./scripts/seed_test_files.sh

cargo run -p cli ./cli/test_files --command ./scripts/command.sh 
