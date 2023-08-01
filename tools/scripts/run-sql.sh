#!/bin/bash

echo "Starting..."

for f in tools/sql/*.sql
do
    command=`cat $f`
    psql postgresql://root:root@localhost:9001/root -c "$command"
done

echo "Done!"