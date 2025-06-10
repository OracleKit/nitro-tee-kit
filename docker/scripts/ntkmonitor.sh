#!/bin/bash
# Monitors daemon for successful TUN setup

DIRNAME=$(dirname $0)
source $DIRNAME/common.sh

# Monitor log file for TUN message
while ! grep "TUN device" $LOG_FILE >/dev/null 2>&1; do
    # sleep .5 seconds before trying again
    sleep .5
done