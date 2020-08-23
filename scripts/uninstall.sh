#!/bin/sh

# uninstall YDict


read -p "Please enter root password: " -s rootPassword

echo "$rootPassword" | sudo -S rm /usr/bin/YDict &&
echo "$rootPassword" | sudo -S rm /usr/share/applications/YDict.desktop &&
echo "$rootPassword" | sudo -S rm -rf /usr/share/icons/hicolor/scalable/apps/YDict