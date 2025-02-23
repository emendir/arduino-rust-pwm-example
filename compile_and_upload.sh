#!/bin/bash
""":"
# the absolute path of this script's directory
script_dir="$( cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"
cd $script_dir

export RAVEDUDE_PORT=$(ls /dev/ttyUSB*)
# export RAVEDUDE_PORT=/dev/ttyUSB0
export RAVEDUDE_BAUDRATE=31250

cargo run







exit 0
"""
import os
import sys

# Python: re-execute the script in Bash
os.execvp("bash", ["bash", __file__] + sys.argv[1:])