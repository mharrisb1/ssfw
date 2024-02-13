# 🪬 ssfw - Super simple file watcher

## Usage

```bash
ssfw --path src/** --command 'cargo test'
```

## Help

```bash
ssfw - Super simple file watcher

Usage: ssfw [OPTIONS] --path <PATH> --command <COMMAND>

Options:
  -p, --path <PATH>        Monitoring path
  -c, --command <COMMAND>  Command
  -v, --verbose            Toggle verbosity
  -h, --help               Print help
  -V, --version            Print version
```

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
