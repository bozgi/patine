#!/bin/bash

if [[ $EUID -ne 0 ]]; then
    echo "This script must be run as root"
    exit 1
fi

read -p "Username: " username
read -s -p "Password: " password
echo

useradd -m -s /bin/bash -G mailwriters "$username"
if [[ $? -ne 0 ]]; then
    echo "Failed to add user"
    exit 1
fi

echo "$username:$password" | chpasswd

maildir="/var/mail/$username/Maildir"
mkdir -p "$maildir"/{cur,new,tmp}

chown -R "$username":"mailwriters" "/var/mail/$username"
chmod -R 770 "/var/mail/$username"

echo "User $username created with Maildir at $maildir"
