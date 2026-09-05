#!/bin/bash
# Aetheris CLI wrapper for Unix shells
command -v aeth > /dev/null 2>&1 || { echo "aeth not found in PATH. Please install Aetheris first."; exit 1; }
exec aeth "$@"
