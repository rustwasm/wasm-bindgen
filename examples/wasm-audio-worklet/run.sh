#!/bin/sh

set -ex

./build.sh

python3 server.py
