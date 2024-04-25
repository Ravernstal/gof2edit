[![Tests](https://github.com/Ravernstal/gof2edit/actions/workflows/ci.yml/badge.svg)](https://github.com/Ravernstal/gof2edit/actions/workflows/ci.yml)

# gof2edit
Galaxy on Fire 2 CLI tool to unpack and repack BIN files and save files, as well as patch binaries.

## Unpacking/Repacking

This tool is capable of unpacking and repacking the following game files:

### BIN Files

* [Agents (agents.bin)](https://github.com/Ravernstal/gof2-bin-info/blob/master/agents.md)
* [Items (items.bin)](https://github.com/Ravernstal/gof2-bin-info/blob/master/items.md)
* [Language Strings (*.lang)](https://github.com/Ravernstal/gof2-bin-info/blob/master/lang.md)
* [Most Wanted (wanted.bin)](https://github.com/Ravernstal/gof2-bin-info/blob/master/wanted.md)
* [News Items (ticker.bin)](https://github.com/Ravernstal/gof2-bin-info/blob/master/ticker.md)
* [Ships (ships.bin)](https://github.com/Ravernstal/gof2-bin-info/blob/master/ships.md)
* [Ship Weapon/Engine Positions (weapons*.bin)](https://github.com/Ravernstal/gof2-bin-info/blob/master/weapons.md)
* [Stations (stations.bin)](https://github.com/Ravernstal/gof2-bin-info/blob/master/stations.md)
* [Systems (systems.bin)](https://github.com/Ravernstal/gof2-bin-info/blob/master/systems.md)

### Save Files

* Save Game (gof2_save_game_*) (Android Only)
* [Save Game Preview (gof2_save_game_preview_*)](https://github.com/Ravernstal/gof2-bin-info/blob/master/save-preview.md)

## Patching

### New Objects

In order to add new entries into these files, binary modification is necessary.
`gof2edit` can patch the game binary for the following files:

* [Stations (stations.bin)](https://github.com/Ravernstal/gof2-bin-info/blob/master/stations.md)
* [Systems (systems.bin)](https://github.com/Ravernstal/gof2-bin-info/blob/master/systems.md)

This functionality is currently only fully available for the Android binary, and experimentally for the iOS binary.

### Binary Patches

`gof2edit` can also apply arbitrary patches to binaries.
See the [patches](patches) folder for examples.

## Example Commands
* `gof2edit unpack systems systems.bin` - Unpacks the systems in JSON format into `systems.json`
* `gof2edit repack systems systems.json` - Repacks the systems from `systems.json` into `systems.bin`
* `gof2edit patch systems systems.json libgof2hdaa.so` - Patches the game using `systems.json`
* `gof2edit apply-patch patches/enable-bloom.json libgof2hdaa.so` - Applies the "Enable Bloom" patch to the Android binary

Execute `gof2edit --help` to get a list of commands, and `gof2edit <command> --help` for more information.

## Building
Simply download the repository and execute `cargo build` in the project directory.
A binary will be created in `target/debug`.

## License
This project is licensed under the GNU AGPLv3 License - see the [LICENSE](LICENSE) file for details.
