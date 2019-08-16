#!/usr/bin/env sh

root=$PWD
cd "$(dirname "$0")" || exit

if [ "$#" -gt 1 ] || [ "$1" = "-h" ] || [ "$1" = "--help" ]; then
	echo "Usage:"
	echo "    $(basename "$0")             Starts the Hardware Simulator in"
	echo "                                      interactive mode."
	echo "    $(basename "$0") FILE.tst    Starts the Hardware Simulator and runs the"
	echo "                                      FILE.tst test script. The success/failure"
	echo "                                      message is printed to the command console."
elif [ $# -eq 1 ]; then
	java -classpath "${CLASSPATH}:bin/classes:BuiltIn:bin/lib/Hack.jar:bin/lib/HackGUI.jar:bin/lib/Simulators.jar:bin/lib/SimulatorsGUI.jar:bin/lib/Compilers.jar" HardwareSimulatorMain "$root/$1"
else
	java -classpath "${CLASSPATH}:bin/classes:BuiltIn:bin/lib/Hack.jar:bin/lib/HackGUI.jar:bin/lib/Simulators.jar:bin/lib/SimulatorsGUI.jar:bin/lib/Compilers.jar" HardwareSimulatorMain
fi
