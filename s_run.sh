#!/bin/bash
echo ""
cd ./quick_chat_s
# cargo run --quiet -p quick_chat_s
# cargo run -p quick_chat_s -- --auth-key xxx
cargo run -p quick_chat_s 
cd ..
echo ""
