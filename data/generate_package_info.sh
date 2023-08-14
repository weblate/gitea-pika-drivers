#! /bin/bash

if [[ $1 == "version" ]]
then
	if [[ $2 == "pika-rocm-meta" ]]
	then
		echo "Version: $(apt-cache show rocm-core | grep Version: | cut -d":" -f2 | head -n1)"
	else
		echo "Version: $(apt-cache show $2 | grep Version: | cut -d":" -f2 | head -n1)"
	fi
else
	if [[ $1 == "description" ]]
	then
	    if echo "$2" | grep mesa-git &> /dev/null
	    then
	        printf "WARNING: THIS DRIVER IS EXPERMINTAL USE AT YOUR OWN RISK!\n$(apt-cache show mesa-git | grep 'Description*' | cut -d":" -f2 | head -n1)"
	    else
		    apt-cache show $2 | grep 'Description*' | cut -d":" -f2 | head -n1
		fi
	else
		if [[ $1 == "device" ]]
		then
			if echo "$2" | grep -i -E 'pika-rocm-meta|vulkan-amdgpu-pro|amf-amdgpu-pro|amdvlk|opencl-legacy-amdgpu-pro-icd|amdgpu-pro-oglp' &> /dev/null
			then
				DEVICE="$(lspci | grep -i -E 'vga|display|3d' | cut -d":" -f3 | grep -i AMD)"
			else
				if echo "$2" | grep -i -E 'xone' &> /dev/null
				then
					DEVICE="$(lsusb | grep -i xbox | cut -d":" -f3)"
				else
					if echo "$2" | grep -i -E 'nvidia' &> /dev/null
					then
						DEVICE="$(lspci | grep -i -E 'vga|display|3d' | cut -d":" -f3 | grep -i nvidia)"
					else
						if echo "$2" | grep -i -E 'mesa' &> /dev/null
						then
							DEVICE="$(lspci | grep -i -E 'vga|display|3d' | cut -d":" -f3 | grep -vi nvidia)"
						else
							DEVICE="$(ubuntu-drivers devices | sed ':a;N;$!ba;s/\nmodel/ /g' | grep vendor | grep -i $2 | sed 's/vendor/Device:/')"
						fi
					fi
				fi
			fi
		fi
			if [[ ! -z $DEVICE ]]
			then
				echo "Device: $DEVICE"
			else
				echo "Device: UNKNOWN!"
			fi
	fi
fi
