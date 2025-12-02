#!/bin/bash
set -u
lang=$1
mkdir -p "$lang/{2015..2024}/{01..25}"
mkdir -p "$lang/2025/{01..12}"
