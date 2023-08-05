# gof2edit
Galaxy on Fire 2 tool to unpack and repack BIN files, as well as patch binaries.

## Example Commands
* `gof2edit unpack systems systems.bin` - Unpacks the systems in JSON format into `systems.json`
* `gof2edit repack systems systems.json` - Repacks the systems from `systems.json` into `systems.bin`
* `gof2edit patch systems systems.json libgof2hdaa.so` - Patches the game using `systems.json`

Execute `gof2edit --help` to get a list of commands, and `gof2edit <command> --help` for more information.

## Building
Simply download the repository and execute `cargo build` in the project directory.
A binary will be created in `target/debug`.

## License
This project is licensed under the GNU AGPLv3 License - see the [LICENSE](LICENSE) file for details.
