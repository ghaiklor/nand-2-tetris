#!/usr/bin/env bash
#title          :run_tests.sh
#description    :This script will run all the required tests to check if CPU is working
#author         :ghaiklor
#date           :2019-08-19
#version        :0.1
#usage          :bash run_tests.sh
#bash_version   :3.2.57(1)-release
#===================================================================================

set -euo pipefail

PROJECT_ROOT=$PWD
PROJECT_TOOLS_ROOT=$PROJECT_ROOT/tools
CPU_EXAMPLES=$PROJECT_ROOT/examples/cpu
HASM_EXAMPLES=$PROJECT_ROOT/examples/hasm
CPU_SOURCES=$PROJECT_ROOT/src/cpu
COMPUTER_SOURCES=$CPU_SOURCES/computer

SUCCESS_COLOR="\033[32m"
PENDING_COLOR="\033[33m"
FAILED_COLOR="\033[31m"
DEFAULT_COLOR="\033[39m"

function header() {
    echo -e "$PENDING_COLOR--->"
    echo -e "---> $1"
    echo -e "--->$DEFAULT_COLOR"
}

function success() {
    echo -e "$SUCCESS_COLOR---> $1$DEFAULT_COLOR"
}

function pending() {
    echo -e "$PENDING_COLOR---> $1$DEFAULT_COLOR"
}

function failed() {
    echo -e "$FAILED_COLOR---> $1$DEFAULT_COLOR"
}

# Assembling and testing examples for CPU, according to Project 4
# https://github.com/ghaiklor/nand-2-tetris#project-4-machine-language-programming
function cpu_examples() {
    echo
    header "Assembling $CPU_EXAMPLES"
    for asm_file in "$CPU_EXAMPLES"/**/*.asm; do
        "$PROJECT_TOOLS_ROOT"/assembler.sh "$asm_file"
        success "🙂 $(basename "$asm_file")"
    done

    echo
    header "Testing $CPU_EXAMPLES"
    for tst_file in "$CPU_EXAMPLES"/**/*.tst; do
        # Fill.tst can not be tested automatically
        if [[ $tst_file =~ Fill.tst ]]; then
            continue
        fi

        "$PROJECT_TOOLS_ROOT"/cpu_emulator.sh "$tst_file" > /dev/null 2>&1
        success "🙂 $(basename $tst_file)"
    done
}

# Assembling examples for the Computer, according to the Project 5
# https://github.com/ghaiklor/nand-2-tetris#project-5-computer-architecture
function hasm_examples() {
    echo
    header "Assembling $HASM_EXAMPLES"
    for asm_file in "$HASM_EXAMPLES"/*.asm; do
        "$PROJECT_TOOLS_ROOT"/assembler.sh "$asm_file"
        success "🙂 $(basename "$asm_file")"
    done

    echo
    header "Moving assembled files to $COMPUTER_SOURCES"
    for hack_file in "$HASM_EXAMPLES"/*.hack; do
        mv "$hack_file" "$COMPUTER_SOURCES/$(basename "$hack_file")"
        success "🙂 $(basename "$hack_file") -> $COMPUTER_SOURCES/$(basename "$hack_file")"
    done
}

# Running tests for the Projects 1, 2, 3, 5
function cpu_tests() {
    echo
    header "Running tests for $CPU_SOURCES"
    for tst_file in "$CPU_SOURCES"/**/*.tst; do
        # Memory.tst can not be tested automatically
        if [[ $tst_file =~ Memory.tst ]]; then
            continue
        fi

        pending "🕰 Simulating $(basename $tst_file)"
        "$PROJECT_TOOLS_ROOT"/hardware_simulator.sh "$tst_file" > /dev/null 2>&1
    done
}

cpu_examples
hasm_examples
cpu_tests

echo
success "All tests have been passed!"
