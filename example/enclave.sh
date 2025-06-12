#!/bin/bash

ntk-up
echo "STARTED"

ps aux

ifconfig
curl http://10.0.0.1:8000/

while true; do
    echo "Hello"
    sleep 5
done