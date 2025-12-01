#!/bin/bash
set -u
lang=$1
year="20$2"
day=$(printf %02d "$3")
dir="$(pwd)/rust/template"
cd "$lang/$year/$day" || exit
if [ "$lang" = "rust" ]
then
	cargo generate -p "$dir" -n part-1
	cargo generate -p "$dir" -n part-2
fi
