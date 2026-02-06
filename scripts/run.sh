#!/usr/bin/bash

server_path="/home/annya/playground/nlp/phonix"
virtual_python=".venv/bin/python"

while getopts "h:stv" opt; do
    case $opt in
    h) # 帮助选项
        echo "usage：$0 [-h] [-s] [-t]"
        echo "  -s    run fastapi server"
        echo "  -t    run tauri app"
        exit 0
        ;;
    t)
        WEBKIT_DISABLE_DMABUF_RENDERER=1 pnpm tauri dev
        ;;
    s)
        SERVER_SRC="${server_path}/main.py"
        EXEC_PATH="${server_path}/${virtual_python}"
        $EXEC_PATH -m fastapi dev $SERVER_SRC
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


