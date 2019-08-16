#!/usr/bin/env sh

root=$PWD
cd "$(dirname "$0")" || exit

if [ "$#" -ne 2 ] || [ "$1" = "-h" ] || [ "$1" = "--help" ]; then
	echo "Usage:"
	echo "    $(basename "$0") FILE1 FILE2    Compares FILE1 and FILE2. The success"
	echo "                                    message or the first miscompared line"
	echo "                                    is printed to the command console."
else
	java -classpath "${CLASSPATH}:bin/classes" TextComparer "$root/$1" "$root/$2"
fi
