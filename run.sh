#!/bin/bash
#use script dir
orignal_path="$PWD"
script_path="$0"
script_dir=$(dirname "$script_path")
cd "$script_dir"

nano envvars.sh
source envvars.sh

cargo run
cd "$orignal_path"