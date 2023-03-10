#!/bin/bash
for arg in "$*"
do
    if test "$arg" = "start"
    then
        ./target/debug/databend-meta -c scripts/ci/deploy/config/databend-meta-node-1.toml &
        sleep 2
        ./target/debug/databend-meta -c scripts/ci/deploy/config/databend-meta-node-3.toml &
        sleep 2
        ./target/debug/databend-meta -c scripts/ci/deploy/config/databend-meta-node-2.toml &
        #./target/debug/databend-query -c scripts/ci/deploy/config/databend-query-node-1.toml &
    elif test "$arg" = "stop"
    then
        kill $(ps -ef | grep 'databend-meta' | awk '{print $2}')
        kill $(ps -ef | grep 'databend-query' | awk '{print $2}')
    fi
done