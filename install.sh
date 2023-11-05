#!/usr/bin/bash

[ "$UID" -eq 0 ] || exec sudo "$0" "$@"
USER_DIR="$(getent passwd $SUDO_USER | cut -d: -f6)"
WORKING_FOLDER="${USER_DIR}/homebrew/plugins/PowerTools"

# Clean directory
mkdir -p /tmp/powertools_binary || true
pushd /tmp/powertools_binary
rm -rf *

systemctl stop plugin_loader || true
curl -L https://github.com/hicder/PowerTools/releases/latest/download/backend -o backend
curl -L https://github.com/hicder/PowerTools/releases/latest/download/index.js -o index.js
curl -L https://github.com/hicder/PowerTools/releases/latest/download/set-clock -o set-clock
curl -L https://github.com/hicder/PowerTools/releases/latest/download/set-clock-mode -o set-clock-mode

chmod +x backend set-clock set-clock-mode

cp index.js $WORKING_FOLDER/dist/index.js
cp backend $WORKING_FOLDER/bin/backend
cp set-clock /usr/local/bin/set-clock
cp set-clock-mode /usr/local/bin/set-clock-mode


systemctl start plugin_loader || true

echo "Successfully installed patched PowerTools for ROG Ally!"
