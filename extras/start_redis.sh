#!/bin/bash
echo "Starting the redis server!"
redis-server &
disown
