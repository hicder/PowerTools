# PowerTools For ROG Ally

## What does it do?

This is a heavily modified version of [PowerTools](https://git.ngni.us/NG-SD-Plugins/PowerTools.git) in order to work
on the ROG Ally. Note that these instructions are all run inside the ROG Ally desktop mode.

## Install
Note that all these commands are run inside the Ally.

1. Please use Decky's [built-in store](https://plugins.deckbrew.xyz/) to install official releases of PowerTools.
2. Run this command once you've installed PowerTools to patch it:
```shell
curl -L https://github.com/hicder/PowerTools/releases/latest/download/install.sh | sh
```
## Build
If you want to develop, follow this instruction:
0. Requirements: a functioning Rust toolchain for x86_64-unknown-linux-gnu (or -musl), pnpm
1. In the root of the directory:
```shell
./build_all.sh
```
