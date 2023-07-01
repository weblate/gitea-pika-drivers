#! /bin/bash

if zenity --question --text ""$1" has been processed successfully. Would you like to reboot for changes to take effect?"
then
	systemctl reboot
fi
