# 🪬 ssfw - Super simple file watcher

A minimal file watcher that watches for modifications to files and executes a command in the foreground.

## Usage

The program will watch for file system events and will compare those events to a given pattern. If the pattern
matches the file from the event, a command will be ran.

```bash
ssfw --pattern '*.rs' --command 'cargo test'
```

> [!IMPORTANT]
> Please note the use of single quotes around the glob. This is required.

## Help

```bash
🪬 Super simple file watcher

Usage: ssfw [OPTIONS] --pattern <PATTERN>

Options:
  -p, --pattern <PATTERN>          Filter pattern
  -c, --command <COMMAND>          Run command [default: :]
  -w, --working-dir <WORKING_DIR>  Optional working directory
      --sh <SH>                    Shell to execute command in [default: zsh] [possible values: zsh, bash]
      --debounce <DEBOUNCE>        Optional debounce window (mulliseconds) [default: 500]
  -v, --verbose...                 Increase logging verbosity
  -q, --quiet...                   Decrease logging verbosity
  -h, --help                       Print help
  -V, --version                    Print version
```

## Variables

Sometimes you will just want to run a given command on the exact file that changed. To do this, you can make use of variables.
Variables are names surrounded with curly braces and will be rendered on command execute.

### Supported Variables

| Name   | Description                     |
| ------ | ------------------------------- |
| `path` | Event detected file or dir path |

```bash
ssfw --pattern '*.ts' --command 'eslint --fix {path}'
```

## Limitations

This program has a number of limitations compared to more mature and robust file watching services.
Some of these limitations are by design since this aims to be a "super simple" file watcher,
but some are from the program's immaturity, namely:

1. Foreground execution only (this was a design choice and will likely not change)
2. Currently, no support for terminating a long-lived program executed by the command. Need to implement some sort of process group management and right now that is out of scope.
3. No support for globbing multiple files types (see [this SO post](https://stackoverflow.com/a/60371634))

## Changelog

- [0.4.0](#040-2024-03-14)
- [0.3.0](#030-2024-03-14)
- [0.2.1](#021-2024-03-08)
- [0.2.0](#020-2024-03-07)

### [0.4.0] - 2024-03-14

Move to FSEvent watcher with debounce for Mac. Helps avoid the issue that [0.3.0](#030-2024-03-14) tried to address in a better way. The debouncer is used
to avoid duplicate events sent from FSEvent.

### [0.3.0] - 2024-03-14

Ran into an issue on a large project where the watcher was taking so long to diff that it became unusable. This release adds a new `--root` option
so you can specify an alternative root for the watcher other than `cwd`.

#### Related issues

- [#6](https://github.com/mharrisb1/ssfw/issues/6)

### [0.2.1] - 2024-03-08

Fixes issue where some commands will exit with status failed but will still pipe to stdout so the file watcher was not displaying the error message.

### [0.2.0] - 2024-03-07

Breaking change for `--path` command which is is now called `--pattern` and breaking change for `{fname}` variable which is now `{path}`.

#### Related issues

- [#1](https://github.com/mharrisb1/ssfw/issues/1)
- [#2](https://github.com/mharrisb1/ssfw/issues/2)
