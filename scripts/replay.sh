#!/usr/bin/env bash
# Replays the json output in the given file generated by 'wrapper.py'

set -euo pipefail

usage() {
    echo "Usage: $0 input.json [<executable override>]"
}

if [ $# -lt 1 ]; then
    usage
    exit 1
fi

in="$1"
mapfile -t cmd < <(jq -r '.cmd[]' < "$in")

if [ $# -ge 2 ]; then
    cmd[0]="$2"
fi

jq -r .input.smtlib < "$in" | "${cmd[@]}"
