#!/bin/bash
# Entrypoint script - starts and waits for daemon,
# gives control to user's CMD directive

DIRNAME=$(dirname $0)
source $DIRNAME/common.sh

# start daemon
bash $DIRNAME/ntk-startup.sh &

# wait for daemon to start
bash $DIRNME/ntk-monitor.sh

exec "$@"
