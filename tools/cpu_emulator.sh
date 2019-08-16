#!/usr/bin/env bash

root=$PWD
cd "$(dirname "$0")" || exit

if [ "$#" -gt 1 ] || [ "$1" = "-h" ] || [ "$1" = "--help" ]; then
	echo "Usage:"
	echo "    $(basename "$0")             Starts the CPU Emulator in interactive mode."
	echo "    $(basename "$0") FILE.tst    Starts the CPU Emulator and runs the File.tst"
	echo "                                test script. The success/failure message"
	echo "                                is printed to the command console."
elif [ $# -eq 1 ]; then
	arg=$([[ $1 = /* ]] && echo "$1" || echo "$root/${1#./}")
	java -classpath "${CLASSPATH}:bin/classes:bin/lib/Hack.jar:bin/lib/HackGUI.jar:bin/lib/Simulators.jar:bin/lib/SimulatorsGUI.jar:bin/lib/Compilers.jar" CPUEmulatorMain "$arg"
else
	java -classpath "${CLASSPATH}:bin/classes:bin/lib/Hack.jar:bin/lib/HackGUI.jar:bin/lib/Simulators.jar:bin/lib/SimulatorsGUI.jar:bin/lib/Compilers.jar" CPUEmulatorMain
fi
