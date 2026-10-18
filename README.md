# termrun

Command-line tool to run command-line tools on an external terminal emulator.

## Installation

```sh
cargo install --locked --git https://git.eyoun.net/ahmed/termrun termrun
```

## Usage

```
Usage: termrun [OPTIONS] <COMMAND>...

Arguments:
  <COMMAND>...  Command to run

Options:
      --terminal <TERMINAL_COMMAND>  Terminal command
      --subcommand <SUBCOMMAND>      Terminal exec subcommand
      --exec-option <EXEC_OPTION>    Terminal exec option
      --wait-option <WAIT_OPTION>    Terminal wait option
      --class-option <CLASS_OPTION>  Terminal class option
      --class <CLASS>                Terminal class
  -r, --redirect                     Redirect stdio to parent shell
  -h, --help                         Print help
  -V, --version                      Print version
```
