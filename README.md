# 🪬 ssfw - Super simple file watcher

## Usage

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
