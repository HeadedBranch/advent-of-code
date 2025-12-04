#!/usr/bin/env bash
set -u
lang=$1
year=$2
day=$3
dir="$(pwd)/rust/template"
cd "$lang/$year/$day" || exit
if [ "$lang" = "rust" ]
then
	cargo generate -p "$dir" -n part-1
	cargo generate -p "$dir" -n part-2
fi
