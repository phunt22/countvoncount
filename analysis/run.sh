#!/bin/bash
set -e
[ ! -d venv ] && python3 -m venv venv
source venv/bin/activate
pip install -q matplotlib numpy
python visualize.py "$1" "${2:-output}"
