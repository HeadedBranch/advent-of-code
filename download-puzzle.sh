#!/bin/bash
set -u
year=$1
day=$2
folder="$year/$day"
curl "https://adventofcode.com/$year/day/$((10#$day))/input" -o "master/$folder/input" -b cookies.txt
for i in $(find . -maxdepth 1 -type d -not -name ".*" -and -not -name "master")
do
	ln -fs "$(pwd)/master/${folder}/input" "$(pwd)/${i}/${folder}/input"
done
