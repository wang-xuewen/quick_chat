#!/bin/bash
echo ""
cd ./quick_chat_c
# cargo run --quiet -p quick_chat_c
# cargo run -p quick_chat_c -- --auth-key xxx
cargo run -p quick_chat_c -- -a password
cd ..
echo ""
