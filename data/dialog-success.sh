#! /bin/bash

WHO=$(whoami)
if [[ $WHO == "pikaos" ]]; then
zenity --info --text ""$1" has been processed successfully."
else
if echo $1 | grep -i mesa
then
    zenity --info --text ""$1" has been processed successfully."
else
    if zenity --question --text ""$1" has been processed successfully. Would you like to reboot for changes to take effect?"
    then
	    systemctl reboot
    fi
fi
fi
