#!/bin/bash
# Starts daemon with auto-restart on failure

DIRNAME=$(dirname $0)
source $DIRNAME/common.sh

# bounce the log file
rm $LOG_FILE 2>/dev/null
touch $LOG_FILE

# start daemon
while true; do
    ntk-enclave >$LOG_FILE 2>&1

    echo "Daemon failed. Restarting..." >> $LOG_FILE
    sleep .5
done;