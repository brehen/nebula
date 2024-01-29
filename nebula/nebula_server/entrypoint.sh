#!/bin/sh
# entrypoint.sh

# Set default values
PORT=${PORT:-"1234"}
ADDRESS=${ADDRESS:-"127.0.0.1"}
ASSETS_PATH=${ASSETS_PATH:-"./assets"}

# Exec neb server
exec nebula_server -e "$ADDRESS" -p "$PORT" -a "$ASSETS_PATH"
