#!/bin/bash
#set -e
# Purge cache and installs for testing
#cargo run -- --purge cache

RUST_BACKTRACE=1 cargo run -- $@
