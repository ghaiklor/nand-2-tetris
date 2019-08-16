#!/usr/bin/env bash

root=$PWD
cd "$(dirname "$0")" || exit

if [ "$#" -ne 2 ] || [ "$1" = "-h" ] || [ "$1" = "--help" ]; then
	echo "Usage:"
	echo "    $(basename "$0") FILE1 FILE2    Compares FILE1 and FILE2. The success"
	echo "                                    message or the first miscompared line"
	echo "                                    is printed to the command console."
else
	arg1=$([[ $1 = /* ]] && echo "$1" || echo "$root/${1#./}")
	arg2=$([[ $2 = /* ]] && echo "$2" || echo "$root/${2#./}")
	java -classpath "${CLASSPATH}:bin/classes" TextComparer "$arg1" "$arg2"
fi
