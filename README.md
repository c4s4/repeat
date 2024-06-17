# Repeat

Repeat a command given number of times.

## Installation

### Unix users (Linux, BSDs and MacOSX)

Unix users may download and install latest *repeat* release with command:

```bash
sh -c "$(curl https://sweetohm.net/dist/repeat/install)"
```

If *curl* is not installed on you system, you might run:

```bash
sh -c "$(wget -O - https://sweetohm.net/dist/repeat/install)"
```

**Note:** Some directories are protected, even as *root*, on **MacOSX** (since *El Capitan* release), thus you can't install *repeat* in */usr/bin* for instance.

### Binary package

Otherwise, you can download latest binary archive at <https://github.com/c4s4/repeat/releases>. Unzip the archive, put the binary of your platform somewhere in your *PATH* and rename it *repeat*.

## Usage

```bash
repeat [--version|-V] [--help|-h] [--black|-b] [--character|-c =] n command [args...]
```

Where:

- `n` is the number of times to repeat the command
- `command [args...]` is the command to repeat followed by its arguments
- `--version` or `-V` to print version and exit
- `--help` or `-h` to print help and exit
- `--black` or `-b` to disable color output
- `--character` or `-c` to specify the character to repeat (default is `=`)

*Enjoy!*
