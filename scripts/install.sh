#!/bin/sh

# install YDict

read -p "Please enter root password: " -s rootPassword

echo "$rootPassword" | sudo -S install -Dm755 ../YDict /usr/bin/YDict &&
echo "$rootPassword" | sudo -S install -Dm644 ../desktop/YDict.desktop /usr/share/applications/YDict.desktop &&
echo "$rootPassword" | sudo -S mkdir -p /usr/share/icons/hicolor/scalable/apps/YDict &&
echo "$rootPassword" | sudo -S install -Dm644 ../media/logo.png /usr/share/icons/hicolor/scalable/apps/YDict/desktop.png &&
echo "$rootPassword" | sudo -S install -Dm644 ../media/tray_icon.png /usr/share/icons/hicolor/scalable/apps/YDict/tray.png &&
echo "$rootPassword" | sudo -S install -Dm644 ../media/born_icon.png /usr/share/icons/hicolor/scalable/apps/YDict/born.png &&
echo "$rootPassword" | sudo -S install -Dm644 ../media/clip_close.png /usr/share/icons/hicolor/scalable/apps/YDict/close.png
