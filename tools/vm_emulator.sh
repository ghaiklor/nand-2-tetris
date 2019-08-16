#!/usr/bin/env sh

root=$PWD
cd "$(dirname "$0")" || exit

if [ "$#" -gt 1 ] || [ "$1" = "-h" ] || [ "$1" = "--help" ]; then
	echo "Usage:"
	echo "    $(basename "$0")             Starts the VM Emulator in interactive mode."
	echo "    $(basename "$0") FILE.tst    Starts the VM Emulator and runs the FILE.tst test"
	echo "                               script. The success/failure message is"
	echo "                               printed to the command console."
elif [ $# -eq 1 ]; then
	java -classpath "${CLASSPATH}:bin/classes:bin/lib/Hack.jar:bin/lib/HackGUI.jar:bin/lib/Simulators.jar:bin/lib/SimulatorsGUI.jar:bin/lib/Compilers.jar" VMEmulatorMain "$root/$1"
else
	java -classpath "${CLASSPATH}:bin/classes:bin/lib/Hack.jar:bin/lib/HackGUI.jar:bin/lib/Simulators.jar:bin/lib/SimulatorsGUI.jar:bin/lib/Compilers.jar" VMEmulatorMain
fi
