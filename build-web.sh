#!/usr/bin/env bash

echo "Starting development server with auto-reload..."

# Kill any existing python server on port 8000
pkill -f "python.*http.server.*8000" 2>/dev/null || true

# Start the HTTP server in the background
python3 -m http.server 8000 &
SERVER_PID=$!

echo "HTTP server started on http://localhost:8000"
echo "Watching for file changes..."

# Watch for changes and rebuild
cargo watch -x "build --target wasm32-unknown-unknown" -s "wasm-pack build --target web --out-dir pkg"

# Cleanup: kill the server when the script exits
trap "kill $SERVER_PID 2>/dev/null || true" EXIT