#!/usr/bin/env bash

set -eu

if [ $# -ne 2 ]; then
  echo "Usage: $0 YEAR DAY"
  exit 1
fi

YEAR=$1
DAY=$2
PADDED_DAY=$(printf "%02d" $DAY)

mkdir -p "inputs/$YEAR"
mkdir -p "src/y$YEAR"

printf "downloading input..."

if ! curl -f --cookie "session=$AOC_SESSION" "https://adventofcode.com/$YEAR/day/$DAY/input" > "inputs/$YEAR/$PADDED_DAY.txt" 2>/dev/null; then
  echo "failed"
  exit 1
fi

echo "downloaded"

SRC_RS="src/y$YEAR/day$PADDED_DAY.rs"

printf "copy template..."
if [ -e $SRC_RS ]; then
    echo "$SRC_RS already exists"
    exit 1
fi

# create day file
cat dayNN.rs.tpl | sed "s/\$YEAR/$YEAR/g" | sed "s/\$DAY/$DAY/g" > src/y$YEAR/day$PADDED_DAY.rs

# add mod declaration
echo "pub mod day${PADDED_DAY};" >> src/y$YEAR/mod.rs

# update main.rs
sed -i "s/y.*::day.*::solve/y$YEAR::day$PADDED_DAY::solve/g" src/main.rs

echo "done"
