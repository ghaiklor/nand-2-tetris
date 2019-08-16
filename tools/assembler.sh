#!/usr/bin/env bash

root=$PWD
cd "$(dirname "$0")" || exit

if [ "$#" -gt 1 ] || [ "$1" = "-h" ] || [ "$1" = "--help" ]; then
	echo "Usage:"
	echo "    $(basename "$0")               Starts the assembler in interactive mode."
	echo "    $(basename "$0") FILE[.asm]    Assembles FILE.asm to FILE.hack."
elif [ $# -eq 1 ]; then
	arg=$([[ $1 = /* ]] && echo "$1" || echo "$root/${1#./}")
	java -classpath "${CLASSPATH}:bin/classes:bin/lib/Hack.jar:bin/lib/HackGUI.jar:bin/lib/Compilers.jar:bin/lib/AssemblerGUI.jar:bin/lib/TranslatorsGUI.jar" HackAssemblerMain "$arg"
else
	java -classpath "${CLASSPATH}:bin/classes:bin/lib/Hack.jar:bin/lib/HackGUI.jar:bin/lib/Compilers.jar:bin/lib/AssemblerGUI.jar:bin/lib/TranslatorsGUI.jar" HackAssemblerMain
fi
