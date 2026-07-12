#!/usr/bin/env bash

set -e

echo "Starting..."
cd apps/server && cargo watch '-x run --bin server' -w src -w config
