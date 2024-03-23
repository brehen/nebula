#!/bin/sh
docker run -it -p 1883:1883 eclipse-mosquitto:2.0.18 mosquitto -c /mosquitto-no-auth.conf
