# PowerTools
<!-- TODO Update badges for new git repo location -->
[![Decky store](https://img.shields.io/badge/dynamic/json?color=blue&label=release&query=%24%5B%3F%28%40.name%3D%3D%27PowerTools%27%29%5D.versions%5B0%5D.name&url=https%3A%2F%2Fplugins.deckbrew.xyz%2Fplugins&style=flat-square)](https://plugins.deckbrew.xyz/)
[![Custom store](https://img.shields.io/badge/dynamic/json?color=blue&label=preview&query=%24%5B%3F%28%40.name%3D%3D%27PowerTools%27%29%5D.versions%5B0%5D.name&url=https%3A%2F%2Fnot-decky-alpha.ngni.us%2Fplugins&style=flat-square)](https://github.com/NGnius/PowerTools/wiki)
[![GitHub package.json version](https://img.shields.io/badge/dynamic/json?url=https%3A%2F%2Fgit.ngni.us%2FNG-SD-Plugins%2FPowerTools%2Fraw%2Fbranch%2Fmain%2Fpackage.json&query=%24.version&style=flat-square&label=local&cacheSeconds=600)](https://git.ngni.us/NG-SD-Plugins/PowerTools/src/branch/main/package.json)

[![Liberapay](https://img.shields.io/liberapay/patrons/NGnius?style=flat-square)](https://liberapay.com/NGnius)
[![GitHub](https://img.shields.io/badge/GPL--3.0-orange?style=flat-square&label=license&cacheSeconds=600)](https://github.com/NGnius/PowerTools/blob/main/LICENSE)
[![GitHub package.json dependency version (local)](https://img.shields.io/badge/dynamic/json?url=https%3A%2F%2Fgit.ngni.us%2FNG-SD-Plugins%2FPowerTools%2Fraw%2Fbranch%2Fmain%2Fpackage.json&query=%24..%5B'decky-frontend-lib'%5D&style=flat-square&label=decky-frontend-lib&cacheSeconds=600)](https://github.com/NGnius/PowerTools/blob/main/pnpm-lock.yaml)

![plugin_demo](./assets/ui.png)

Steam Deck power tweaks for power users.

This is generated from the template plugin for the [Decky Plugin Loader](https://github.com/SteamDeckHomebrew/decky-loader).
You will need that installed for this plugin to work.

## What does it do?

- Enable & disable CPU threads & SMT
- Set CPU frequencies
- Set GPU frequencies and power (fastPPT & slowPPT)
- Cap battery charge rate (when awake)
- Display supplementary battery info
- Keep settings between restarts (stored in `~/.config/powertools/<gameId>.json`)

This plugin is tested on Steam Deck, but is designed to work on other Linux devices as well. Unfortunately I am currently unable to test on other devices.

## Install

Please use Decky's [built-in store](https://plugins.deckbrew.xyz/) to install official releases.
If you want to test unstable versions, use [my custom store](https://not-decky-alpha.ngni.us/plugins). If you would like to use an in-development version, feel free to build PowerTools yourself.

## Build

0. Requirements: a functioning Rust toolchain for x86_64-unknown-linux-gnu (or -musl), pnpm, and some tech literacy
1. In a terminal, navigate to the backend directory of this project and run `./build.sh`
2. In the root of this project, run `pnpm run build`
3. Transfer the project (especially dist/ and bin/) to a folder in your Steam Deck's `~/homebrew/plugins` directory

## License

This is licensed under GNU GPLv3.
