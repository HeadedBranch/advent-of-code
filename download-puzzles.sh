#!/bin/bash
set -u
lang=$1
year=$2
day=$3
file="$lang/$year/$(printf %02d "$day")/input"
curl "https://adventofcode.com/$year/day/$day/input" -o "$file" -b cookies.txt
