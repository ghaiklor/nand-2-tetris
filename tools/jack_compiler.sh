#!/usr/bin/env bash

root=$PWD
cd "$(dirname "$0")" || exit

if [ "$#" -gt 1 ] || [ "$1" = "-h" ] || [ "$1" = "--help" ]; then
	echo "Usage:"
	echo "    $(basename "$0")              Compiles all .jack files in the current"
	echo "                                  working directory."
	echo "    $(basename "$0") DIRECTORY    Compiles all .jack files in DIRECTORY."
	echo "    $(basename "$0") FILE.jack    Compiles FILE.jack to FILE.vm."
else
	arg=$([[ $1 = /* ]] && echo "$1" || echo "$root/${1#./}")
	java -classpath "${CLASSPATH}:bin/classes:bin/lib/Hack.jar:bin/lib/Compilers.jar" Hack.Compiler.JackCompiler "$arg"
fi
