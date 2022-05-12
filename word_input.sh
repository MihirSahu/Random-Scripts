#!/usr/bin/bash


if [ $(id -u) != 1000 ]
then
	echo "You are not allowed to run this script."
	exit
fi

echo "Input as many words separated by a newline as you want, and then enter q to quit."
words=()
temp=""

while true
do
	temp=""
	read temp

	if [ $temp == "q" ]
	then
		break
	fi

	words+=($temp)
done

echo
echo "Your words were:"
for word in ${words[@]}
do
	echo $word
done
