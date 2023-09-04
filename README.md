# PowerTools For ROG Ally

## What does it do?

This is a heavily modified version of [PowerTools](https://git.ngni.us/NG-SD-Plugins/PowerTools.git) in order to work
on the ROG Ally.

## Build

0. Requirements: a functioning Rust toolchain for x86_64-unknown-linux-gnu (or -musl), pnpm, and some tech literacy
1. In a terminal, navigate to the backend directory of this project and run `./build.sh`
2. In the root of this project, run `pnpm run build`

## Install
Note that all these commands are run inside the Ally.

1. Please use Decky's [built-in store](https://plugins.deckbrew.xyz/) to install official releases.
2. Build the front-end and back-end for this modified plugin
```shell
./build_all.sh
```
Then copy these files to the decky loader's directory:
```shell
sudo systemctl stop plugin_loader
sudo cp --preserve=mode /tmp/backend $HOME/homebrew/plugins/PowerTools/bin/backend
sudo cp --preserve=mode /tmp/index.js $HOME/homebrew/plugins/PowerTools/dist/index.js
sudo systemctl start plugin_loader
```
