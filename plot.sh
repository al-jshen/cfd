#!/bin/bash

cargo run --release > data.txt
python3 anim.py data.txt $1
