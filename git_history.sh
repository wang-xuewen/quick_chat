#!/bin/bash

echo ""
git log -n 5 --pretty=format:"%h - %an, %ad : %s" origin/main
echo ""
