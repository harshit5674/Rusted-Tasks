#!/bin/bash


function rusted(){
bold=$(tput bold)
normal=$(tput sgr0)

if [ "$*" = "" ]
then
	echo "${bold}rusted"
elif [ "$1" = "date" ]
then
	echo "${bold}date"
elif [ "$1" = "clear" ]
then
	echo "${bold}clear"
elif [ "$1" = "add" ]
then
	echo "${bold}add"
elif [ "$1" = "done" ]
then
	echo "${bold}done"
elif [ "$1" = "all" ]
then
	echo "${bold}all"
else
	echo "No command found"
fi
}
