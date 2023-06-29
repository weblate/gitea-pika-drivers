#! /bin/bash
### Get traditional ubuntu-driver list
UBUNTU_DRIVERS="$(ubuntu-drivers list | grep -vi -server)"

### Check if the amdgpu module is loaded
### advanced micro devices graphics cards with "radeon" module only do not support the additional drivers
if inxi -G | grep " loaded" | grep "amdgpu" &> /dev/null
then
	AMDGPU_DRIVERS="$(echo -e "pika-rocm-meta\nvulkan-amdgpu-pro\namf-amdgpu-pro\namdvlk\nopencl-legacy-amdgpu-pro-icd\namdgpu-pro-oglp")"
fi

### Check for xbox equipment
if lsusb | grep -i xbox  &> /dev/null
then
	XONE_DRIVERS="$(echo -e "xone-dkms")"
fi

### Merge all drivers together
DRIVERS="$(printf "$UBUNTU_DRIVERS\n$AMDGPU_DRIVERS\n$XONE_DRIVERS")"

### If no drivers were found set them to random hash to trigger no drivers needed dialog in gui app
if [[ -z $DRIVERS ]]
then
    DRIVERS=emScuM8rsa6kuhMePtR5bT8s4z9s
fi

### Print result
echo $DRIVERS | tr " " "\n"
