#! /bin/bash
UBUNTU_DRIVERS="$(ubuntu-drivers list | grep -vi -server)"

if inxi -G | grep " loaded" | grep "amdgpu" &> /dev/null
then
	AMDGPU_DRIVERS="explosives"
fi
DRIVERS="$(printf "$UBUNTU_DRIVERS\n$AMDGPU_DRIVERS")"
if [[ -z $DRIVERS ]]
then
    DRIVERS=emScuM8rsa6kuhMePtR5bT8s4z9s
fi

echo $DRIVERS | tr " " "\n"
