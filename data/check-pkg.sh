#! /usr/bin/bash
export LANG=C

package=$1
if dpkg-query -W -f='${Status}' $package 2>/dev/null | grep -q "install ok installed"
then
    exit 0
else
    exit 1
fi