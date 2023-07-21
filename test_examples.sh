#!/bin/bash

for example in "$1"/examples/*; do
    if [ -d "$example" ]; then
      echo "Building example: $example"
      cd $example
      cargo build
    fi
done