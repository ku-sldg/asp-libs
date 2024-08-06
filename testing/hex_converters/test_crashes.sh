#!/bin/bash
# set -eu

if [ "$#" -ne 2 ]; then
  echo "Usage: $0 <binary> <directory>"
  exit 1
fi

for f in $(ls -1 $2); do
  echo "Processing $f"
  $1 $(cat $2/$f)
  echo "Done"
done