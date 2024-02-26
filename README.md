# 🪬 ssfw - Super simple file watcher

A minimal file watcher that watches for modifications to files and executes a command in the foreground.

## Usage

The program expects a set of files to watch and an optional command to run when one
of the watched files has changed. If you do not provide a command then the program will
run a no-op command in the shell.

```bash
ssfw --path 'src/**' --command 'cargo test'
```

> [!IMPORTANT]
> Please note the use of single quotes around the glob. This is required.

## Help

```bash
🪬 Super simple file watcher

Usage: ssfw [OPTIONS] --path <PATH>

Options:
  -p, --path <PATH>        Monitoring path/glob
  -c, --command <COMMAND>  Run command [default: :]
      --poll <POLL>        Poll duration (ms) [default: 500]
  -v, --verbose...         Increase logging verbosity
  -q, --quiet...           Decrease logging verbosity
  -h, --help               Print help
  -V, --version   
```

## Variables

Sometimes you will just want to run a given command on the exact file or files that have changed
and not to the entire file set. To do this, you can make use of the command variables.

### Supported Variables

|  Name   |        Description        |
|---------|---------------------------|
| `fname` | Name of file that changed |

```bash
ssfw --path 'src/**/*.ts' --command 'eslint --fix {fname}'
```

## Limitations

This program has a number of limitations compared to more mature and robust file watching services.
Some of these limitations are by design since this aims to be a "super simple" file watcher,
but some are from the program's immaturity, namely:

1. No support for files added to matching path
2. Use of file polling instead of using system events
3. Foreground execution only (this was a design choice and will likely not change)
4. Currently, no support for terminating a long-lived program executed by the command. Need to implement some sort of process group management and right now that is out of scope.

## Build

> [!WARNING]
> Only tested on Apple M1 Pro

Currently, the only option for using this tool is to build it from source.

```bash
git clone https://github.com/mharrisb1/ssfw.git
cd ssfw
cargo build --release
```

Then add to path.
