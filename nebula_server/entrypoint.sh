#!/bin/sh
# entrypoint.sh

# Set default values
PORT=${PORT:-"1234"}
ADDRESS=${ADDRESS:-"127.0.0.1"}

# Exec neb server
exec nebula_server -a "$ADDRESS" -p "$PORT"
