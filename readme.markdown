# Cube Code Maker

This tool takes a list of author names and creates a Cutter-code-like table or "map" to encode any author name into a series of numbers. 

As an example, the family name "Smith" might be encoded as `S6187`.

The tool performs some basic letter frequency analysis to create this "map". 

This map is a Rust HashMap with 26 keys, one for each letter of the English alphabet. Each value of this HashMap is a Vector of 676 values representing all possible second and third letter combinations in a given name. These 676 values range from `01` to `99`, so that each combination will be represented in two characters. To accommodate for this, any number may be repeated any number of times across less frequently occurring letter combinations like "cgn".

## Installation

### Using Rust and cargo
1. [Install Rust](https://www.rust-lang.org/tools/install) if you haven't already
2. Run: `cargo install --git https://github.com/sts10/cube-code-maker --locked --branch main` (Run this same command to upgrade Cube Code Maker.)

You should then be able to run `cube_code --help` for help text.

Uninstall Cube Code Maker by running `cargo uninstall cube_code`.

### Releases
Check the [GitHub Releases page](https://github.com/sts10/cube-code-maker/releases) for binaries suitable for Mac, Windows, and Linux users.

To install the executable on a Linux/macOS machine, download the appropriate executable and move it to somewhere in your `$PATH`, like `$HOME/.local/bin` (you can do this on the command line with something like `mv ~/Downloads/cube_code ~/.local/bin/`).
