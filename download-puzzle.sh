#!/bin/bash
set -u
lang=$1
year=$2
day=$3
folder="$lang/$year/$day"
curl "https://adventofcode.com/$year/day/$((10#$day))/input" -o "$folder/input" -b cookies.txt
curl "https://adventofcode.com/$year/day/$((10#$day))/" -o "$folder/html" -b cookies.txt

