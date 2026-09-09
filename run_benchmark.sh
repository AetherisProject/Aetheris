#!/bin/bash

# Ensure cargo is available
cargo --version

# Run the benchmark
cargo bench crypto_bench

# Exit with the same status
exit $?