#!/bin/bash
# Build sqlx into .sqlx 
cargo sqlx prepare -- --all-targets --all-features

docker build -t model-parser .   