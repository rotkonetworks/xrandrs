# Xrandrs
Xrandrs is a command-line tool for automatically adjusting the display resolution and zoom level based on the aspect ratio of the connected display. It uses the xrandr and regex libraries to parse the output of the xrandr command and calculate an appropriate zoom level based on a set of configurable rules.

## Installation
To install Xrandrs, you'll need to have Rust installed on your system. You can install Rust by following the instructions on the official Rust website.

Once you have Rust installed, you can clone the Xrandrs repository and build the project using the following commands:

```bash
git clone https://github.com/rotkonetworks/xrandrs.git
cd xrandrs
cargo build --release
```
This will create a binary file called xrandrs in the target/release directory. You can run the xrandrs command by navigating to this directory and typing ./xrandrs.

## Usage
Xrandrs accepts one optional command-line argument: the path to a configuration file. If no configuration file is provided, Xrandrs will use a default configuration file that is included in the binary.

execute on off with command
```
xrandrs
```
