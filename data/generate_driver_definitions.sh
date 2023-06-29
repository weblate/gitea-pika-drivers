#! /bin/bash

### Get traditional ubuntu-driver list
DRIVERS="$(ubuntu-drivers list | grep -vi -server)"

### Check if the amdgpu module is loaded
### advanced micro devices graphics cards with "radeon" module only do not support the additional drivers
if inxi -G | grep driver | grep amdgpu &> /dev/null
then
	DRIVERS="$DRIVERS pika-rocm-meta vulkan-amdgpu-pro amf-amdgpu-pro amdvlk opencl-legacy-amdgpu-pro-icd amdgpu-pro-oglp"
fi

### Check for xbox equipment
if lsusb | grep -i xbox  &> /dev/null
then
	DRIVERS="$DRIVERS xone-dkms xpadneo"
fi

### If no drivers were found set them to random hash to trigger no drivers needed dialog in gui app
if [[ -z $DRIVERS ]]
then
    DRIVERS=emScuM8rsa6kuhMePtR5bT8s4z9s
fi

### Print result
echo -e "$DRIVERS" | tr ' ' '\n' | grep "\S"
