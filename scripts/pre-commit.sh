#!/bin/bash
./scripts/test.sh || { echo "Tests failed"; exit 1; }