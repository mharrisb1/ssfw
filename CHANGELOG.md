# Changelog

- [0.5.1](#051-2024-04-23)
- [0.5.0](#050-2024-03-24)
- [0.4.0](#040-2024-03-14)
- [0.3.0](#030-2024-03-14)
- [0.2.1](#021-2024-03-08)
- [0.2.0](#020-2024-03-07)

## [0.6.0] - 2024-06-13

Uses [globset](https://docs.rs/globset/latest/globset) as pattern matcher since it supports `{a,b}` patterns.

## [0.5.1] - 2024-04-23

Uses generic `new_debouncer` to avoid explicit usage of FSEvent since that is local to MacOS.

## [0.5.0] - 2024-03-24

Adds option to use `notify::PollWatcher` instead of debounce watcher.

### Related issues

- [#9: Avoid infinite loop when command changes file](https://github.com/mharrisb1/ssfw/issues/9)

## [0.4.0] - 2024-03-14

Move to FSEvent watcher with debounce for Mac. Helps avoid the issue that [0.3.0](#030-2024-03-14) tried to address in a better way. The debouncer is used
to avoid duplicate events sent from FSEvent.

## [0.3.0] - 2024-03-14

Ran into an issue on a large project where the watcher was taking so long to diff that it became unusable. This release adds a new `--root` option
so you can specify an alternative root for the watcher other than `cwd`.

### Related issues

- [#6: Add optional root](https://github.com/mharrisb1/ssfw/issues/6)

## [0.2.1] - 2024-03-08

Fixes issue where some commands will exit with status failed but will still pipe to stdout so the file watcher was not displaying the error message.

## [0.2.0] - 2024-03-07

Breaking change for `--path` command which is is now called `--pattern` and breaking change for `{fname}` variable which is now `{path}`.

### Related issues

- [#1: feat: add newly created files to watch paths](https://github.com/mharrisb1/ssfw/issues/1)
- [#2: bug: handle file delete](https://github.com/mharrisb1/ssfw/issues/2)
