#!/usr/bin/bash

[ "$UID" -eq 0 ] || exec sudo "$0" "$@"
USER_DIR="$(getent passwd $SUDO_USER | cut -d: -f6)"
WORKING_FOLDER="${USER_DIR}/homebrew/plugins/PowerTools"
CONFIG_FOLDER="${USER_DIR}/.config/powertools"

# Clean directory
mkdir -p /tmp/powertools_binary || true
pushd /tmp/powertools_binary
rm -rf *

systemctl stop plugin_loader || true
curl -L https://github.com/hicder/PowerTools/releases/latest/download/backend -o backend
curl -L https://github.com/hicder/PowerTools/releases/latest/download/index.js -o index.js

echo "Removing cached limit files..."
rm $CONFIG_FOLDER/limits_cache.json || true

echo "Copying files..."
cp index.js $WORKING_FOLDER/dist/index.js
cp backend $WORKING_FOLDER/bin/backend

systemctl start plugin_loader || true
echo "Successfully installed patched PowerTools for ROG Ally!"
