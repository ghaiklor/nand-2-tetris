#!/usr/bin/env bash

function cpu_tests() {
    echo "Running test benches for src/cpu..."
    for tst_file in "$PWD"/src/cpu/**/*.tst; do
        echo "Running $tst_file in hardware simulator..."
        "$PWD"/tools/hardware_simulator.sh "$tst_file" > /dev/null 2>&1
    done
}

cpu_tests
