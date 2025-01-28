# 🪬 ssfw - Super simple file watcher

A minimal file watcher that executes a command in the foreground on file event.

## Usage

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
  -w, --working-dir <WORKING_DIR>  Optional working directory [default: .]
      --sh <SH>                    Shell to execute command in [default: zsh] [possible values: zsh, bash]
      --debounce <DEBOUNCE>        Optional debounce window (ms) [default: 500]
      --force-poll                 Force poll watcher
      --poll <POLL>                Polling interval (ms). Ignored unless force poll is used [default: 500]
  -v, --verbose...                 Increase logging verbosity
  -q, --quiet...                   Decrease logging verbosity
  -h, --help                       Print help
  -V, --version                    Print version
```

## Patterns

Any valid [glob](https://docs.rs/globset/latest/globset/#syntax) pattern can be used for filtering events. If a file event is detected that matches the given patter, then the command will be ran.

## Variables

> [!WARNING]
> Variables are unstable and subject to change

Variables allow you to pass values from the event context to the command.

For example:

```bash
ssfw -p 'src/*.{js,ts,vue}' -c 'pnpm eslint --fix {path}'
```

This would run `eslint --fix` _just_ for the file from the event.

### Supported Variables

| Name   | Description                     |
| ------ | ------------------------------- |
| `path` | Event detected file or dir path |

## Limitations

This program has a number of limitations compared to more mature and robust file watching services.
Some of these limitations are by design since this aims to be a "super simple" file watcher.

1. Foreground execution only (this was a design choice and will likely not change)
2. Currently, no support for terminating a long-lived program executed by the command. Need to implement some sort of process group management and right now that is out of scope.

### Alternatives

For anyone wanting a mature file wathcer, the usual suspects are recommended:

- [watchexec](https://github.com/watchexec/watchexec)
- [Watchman](https://facebook.github.io/watchman/)
