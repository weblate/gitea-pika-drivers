#! /bin/bash

if dpkg -s "$1"
then
	pkexec env DISPLAY=$DISPLAY XAUTHORITY=$XAUTHORITY bash -c "apt remove $1 -y && sudo apt autoremove -y"
else
	pkexec env DISPLAY=$DISPLAY XAUTHORITY=$XAUTHORITY bash -c "apt update -y && apt install $1 -y"
fi
