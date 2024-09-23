#!/bin/bash

echo "Building the project..."
cargo build -p workspace-binding --release

echo "Copying the binary to the lib directory..."
cp target/release/libworkspace_binding.dylib lua/workspace_binding.so
