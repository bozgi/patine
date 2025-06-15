#!/bin/bash

if [[ $EUID -ne 0 ]]; then
   echo "This script must be run as root" 
   exit 1
fi

read -p "Username: " username
read -p "Are you sure? [y/n]" -n 1 -r
echo    

if [[ ! $REPLY =~ ^[Yy]$ ]]
then
    echo "Nothing was changed"
    exit 0
fi

rm -rf "/var/mail/$username"
userdel -f $username

echo "User $username was removed"
