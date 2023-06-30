#! /bin/bash

if [[ $1 == "version" ]]
then
	apt-cache show $2 | grep Version:
else
	if [[ $1 == "description" ]]
	then
		apt-cache show $2 | grep "Description*" | head -n1
	else
		if [[ $1 == "device" ]]
		then
			if echo "$2" | grep -i -E 'pika-rocm-meta|vulkan-amdgpu-pro|amf-amdgpu-pro|amdvlk|opencl-legacy-amdgpu-pro-icd|amdgpu-pro-oglp' &> /dev/null
			then
				echo "Device: $(inxi -G | grep -i device | grep -i AMD)"
			else
				if echo "$2" | grep -i -E 'xone' &> /dev/null
				then
					echo "Device: $(lsusb | grep -i xbox | cut -d":" -f3)"
				else
					ubuntu-drivers devices | sed ':a;N;$!ba;s/\nmodel/ /g' | grep vendor | grep -i $2 | sed 's/vendor/Device:/'
				fi
			fi
		fi
	fi
fi
