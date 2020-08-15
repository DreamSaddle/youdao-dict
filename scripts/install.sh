#!/bin/sh

# install youdao-dict

read -p "Please enter root password: " -s rootPassword

echo "$rootPassword" | sudo -S install -Dm755 ../youdao-dict /usr/bin/youdao-dict &&
echo "$rootPassword" | sudo -S install -Dm644 ../desktop/youdao-dict.desktop /usr/share/applications/youdao-dict.desktop &&
echo "$rootPassword" | sudo -S install -Dm644 ../media/logo.png /usr/share/icons/hicolor/scalable/apps/youdao-dict-desktop.png &&
echo "$rootPassword" | sudo -S install -Dm644 ../media/tray_icon.png /usr/share/icons/hicolor/scalable/apps/youdao-dict-tray.png
