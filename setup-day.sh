#!/usr/bin/env bash

lang=$1
year=$2
day=$3

if [ $# -lt 3 ]
then
	echo lang:
	read lang
	echo year:
	read year
	echo day:
	read day
fi
year="20$year"
day=$(printf %02d $day)

./download-puzzle.sh $year $day

./setup-templates.sh $lang $year $day

cd "$lang/$year/$day"
