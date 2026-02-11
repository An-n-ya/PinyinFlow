#!/usr/bin/bash

verbose=false

tauri_log="logs/tauri_log.log"
tauri_log_json="scripts/tauri-log.json"
phonix_log="logs/phonix.log"
phonix_log_json="scripts/phonix-log.json"

while getopts "h:lic" opt; do
    case $opt in
    h) # 帮助选项
        echo "usage：$0 [-h] [-i]"
        echo "  -i    install lnav format"
        echo "  -l    open log"
        echo "  -c    clear log"
        exit 0
        ;;
    l)
        lnav $tauri_log $phonix_log
        ;;
    c)
        rm -rf $tauri_log $phonix_log
        ;;
    i)
        lnav -i $tauri_log_json
        lnav -i $phonix_log_json
        ;;
    \?)
        echo "ERROR: invalid option -$OPTARG" >&2
        exit 1
        ;;
    :) # missing arguments
        echo "ERROR: option -$OPTARG need argument" >&2
        exit 1
        ;;
    esac
done
