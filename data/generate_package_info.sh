#! /bin/bash

if [[ $1 == "version" ]]
then
	if [[ $2 == "pika-rocm-meta" ]]
	then
		apt-cache show rocm-core | grep Version: | cut -d":" -f2 | head -n1
	else
		apt-cache show $2 | grep Version: | cut -d":" -f2 | head -n1
	fi
elif [[ $1 == "description" ]]
then
	if echo "$2" | grep mesa-git &> /dev/null
	then
		apt-cache show mesa-git | grep 'Description*' | cut -d":" -f2 | head -n1
	else
		apt-cache show $2 | grep 'Description*' | cut -d":" -f2 | head -n1
	fi
elif [[ $1 == "icon" ]]
then
                if echo "$2" | grep "pika-rocm-meta"&> /dev/null; then
                    echo "amd"
                elif echo "$2" | grep "vulkan-amdgpu-pro"&> /dev/null; then
                    echo "amd"
                elif echo "$2" | grep "amf-amdgpu-pro"&> /dev/null; then
                    echo "amd"
                elif echo "$2" | grep "amdvlk"&> /dev/null; then
                    echo "amd"
                elif echo "$2" | grep "opencl-legacy-amdgpu-pro-icd"&> /dev/null; then
                    echo "amd"
                elif echo "$2" | grep "amdgpu-pro-oglp"&> /dev/null; then
                    echo "amd"
                elif echo "$2" | grep "xone-dkms"&> /dev/null; then
                    echo "input-gaming"
                elif echo "$2" | grep "nvidia"&> /dev/null; then
                    echo "nvidia"
                elif echo "$2" | grep "intel"&> /dev/null; then
                    echo "intel"
                else
                    echo "pika-drivers"
                fi
elif [[ $1 == "safe" ]]
then
                if [[ "$2" == "mesa-git" ]]; then
                    echo "true"
                else
                    echo "false"
                fi
elif [[ $1 == "device" ]]
then
	if echo "$2" | grep -i -E 'pika-rocm-meta|vulkan-amdgpu-pro|amf-amdgpu-pro|amdvlk|opencl-legacy-amdgpu-pro-icd|amdgpu-pro-oglp' &> /dev/null
	then
		DEVICE="$(lspci | grep -i -E 'vga|display|3d' | grep -i AMD)"
	elif echo "$2" | grep -i -E 'xone' &> /dev/null
	then
		DEVICE="$(lsusb | grep -i xbox)"
	elif echo "$2" | grep -i -E 'intel' &> /dev/null
	then
		DEVICE="$(lspci | grep -i -E 'vga|display|3d' | grep -i intel)"
	elif echo "$2" | grep -i -E 'nvidia' &> /dev/null
	then
		DEVICE="$(lspci | grep -i -E 'vga|display|3d' | grep -i nvidia)"
	elif echo "$2" | grep -i -E 'mesa' &> /dev/null
	then
		DEVICE="$(lspci | grep -i -E 'vga|display|3d' | grep -vi nvidia)"
	fi
	if [[ ! -z $DEVICE ]]
	then
		echo "$DEVICE"
	else
		echo "UNKNOWN!"
	fi
fi