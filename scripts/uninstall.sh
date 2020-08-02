#!/bin/sh

# uninstall youdao-dict


read -p "Please enter root password: " -s rootPassword

echo "$rootPassword" | sudo -S rm /usr/bin/youdao-dict &&
echo "$rootPassword" | sudo -S rm /usr/share/applications/youdao-dict.desktop &&
echo "$rootPassword" | sudo -S rm /usr/share/icons/hicolor/scalable/apps/youdao-dict-logo.png

# echo "$rootPassword" | sudo -S rm /usr/share/icons/hicolor/scalable/apps/tray_icon.png &&