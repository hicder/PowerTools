# PowerTools For ROG Ally

## What does it do?

This is a heavily modified version of [PowerTools](https://git.ngni.us/NG-SD-Plugins/PowerTools.git) in order to work
on the ROG Ally. Note that these instructions are all run inside the ROG Ally desktop mode.

## Build

0. Requirements: a functioning Rust toolchain for x86_64-unknown-linux-gnu (or -musl), pnpm
1. In the root of the directory:
```shell
./build_all.sh
```

Alternatively, you can download `backend` and `index.js` from the Releases page.

## Install
Note that all these commands are run inside the Ally.

1. Please use Decky's [built-in store](https://plugins.deckbrew.xyz/) to install official releases of PowerTools.
2. Copy these files to the decky loader's directory:
```shell
# Assume you've downloaded `backend`, `index.js`, `set-clock`, `set-clock-mode` to /tmp

sudo systemctl stop plugin_loader
rm $HOME/.config/powertools/limits_cache.json
sudo cp /tmp/backend $HOME/homebrew/plugins/PowerTools/bin/backend
sudo cp /tmp/index.js $HOME/homebrew/plugins/PowerTools/dist/index.js
sudo cp /tmp/set-clock /usr/local/bin/set-clock
sudo cp /tmp/set-clock-mode /usr/local/bin/set-clock-mode
sudo systemctl start plugin_loader
```
