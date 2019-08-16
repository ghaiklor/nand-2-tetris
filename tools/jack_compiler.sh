#!/usr/bin/env sh

root=$PWD
cd "$(dirname "$0")" || exit

if [ "$#" -gt 1 ] || [ "$1" = "-h" ] || [ "$1" = "--help" ]; then
	echo "Usage:"
	echo "    $(basename "$0")              Compiles all .jack files in the current"
	echo "                                  working directory."
	echo "    $(basename "$0") DIRECTORY    Compiles all .jack files in DIRECTORY."
	echo "    $(basename "$0") FILE.jack    Compiles FILE.jack to FILE.vm."
else
	java -classpath "${CLASSPATH}:bin/classes:bin/lib/Hack.jar:bin/lib/Compilers.jar" Hack.Compiler.JackCompiler "$root/$1"
fi
