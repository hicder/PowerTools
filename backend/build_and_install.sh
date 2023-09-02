#!/bin/bash

echo "Build the backend binary..."
cargo build --release

echo "Install the backend binary..."
sudo systemctl stop plugin_loader
sudo cp --preserve=mode ./target/release/powertools $HOME/homebrew/plugins/PowerTools/bin/backend
sudo systemctl start plugin_loader
