#! /bin/bash

if [[ -z $pkg ]] && [[ "$1" = "xone-dkms" ]]
then
	pkg="xone-dkms xpadneo-dkms xpad-noone-dkms"
fi

if [[ -z $pkg ]] && [[ "$1" = "vulkan-amdgpu-pro" ]]
then
	pkg="vulkan-amdgpu-pro vulkan-amdgpu-pro:i386"
fi

if [[ -z $pkg ]] && [[ "$1" = "amf-amdgpu-pro" ]]
then
	pkg="amf-amdgpu-pro vulkan-amdgpu-pro vulkan-amdgpu-pro:i386"
fi

if [[ -z $pkg ]] && [[ "$1" = "amdvlk" ]]
then
	pkg="amdvlk amdvlk:i386"
fi

if [[ -z $pkg ]] && [[ "$1" = "opencl-legacy-amdgpu-pro-icd" ]]
then
	pkg="ocl-icd-libopencl1-amdgpu-pro ocl-icd-libopencl1-amdgpu-pro:i386 opencl-legacy-amdgpu-pro-icd opencl-legacy-amdgpu-pro-icd:i386"
fi

if [[ -z $pkg ]] && [[ "$1" = "amdgpu-pro-oglp" ]]
then
	pkg="amdgpu-pro-oglp amdgpu-pro-oglp:i386"
fi

if [[ -z $pkg ]]
then
	pkg="$1"
fi

if dpkg -s "$1"
then
	if echo $pkg | grep -i mesa
	then
		zenity --error --text "the following driver "$1" can not be removed only swapped"
	else
		pkexec env DISPLAY=$DISPLAY XAUTHORITY=$XAUTHORITY bash -c "apt remove $pkg -y && sudo apt autoremove -y"
	fi
else
	if echo $pkg | grep -i nvidia
 	then
		pkexec env DISPLAY=$DISPLAY XAUTHORITY=$XAUTHORITY bash -c "apt update -y && apt purge nvidia-driver-* -y && apt install $pkg -y && sudo apt autoremove -y"
	else
		if echo $pkg | grep -i mesa-hybrid
		then
			pkexec env DISPLAY=$DISPLAY XAUTHORITY=$XAUTHORITY bash -c "apt update -y && apt install mesa-stable -y && apt install mesa-hybrid -y && sudo apt autoremove -y"
		else
			pkexec env DISPLAY=$DISPLAY XAUTHORITY=$XAUTHORITY bash -c "apt update -y && apt install $pkg -y && sudo apt autoremove -y"
		fi
	fi
fi
