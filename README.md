# Roulette!

*Fancy a game of Roulette?*

Rules:

- spin the wheel with `roulette`
- whatever TODO, FIXME, or type error you get, you must fix!
- total number of TODOs, FIXMEs, or type error must go down

Inspired by (i.e. copied from) Andreas Kling's [FIXME Roulette](https://www.youtube.com/watch?v=fk0EMHevbPs&list=PLMOpZvQB55bdRLT1IY-QD_U4DVp8NDeHo&index=1)

## Installation and Usage

```console
cargo install fixme-roulette
```

Once that is done, navigate to the root of your directory and run

```console
roulette
```

and you'll get something like this!

```console
Number of 'pockets': 13
README.md:8:- whatever TODO, FIXME, or type error you get, you must fix!
```

For more options,

```console
$ roulette --help

Spin the wheel, get a task!

Usage: roulette [OPTIONS]

Options:
  -p, --path <PATH>
          Path to directory (defaults to `.`). Note that `clippy` and `git grep` don't take a path, and require you to run `roulette` where you want the commands to be executed
  -c, --commands <COMMANDS>...
          Commands to run (any of git_grep, rip_grep, grep, mypy, ruff, flake8) - defaults git grep and mypy
  -g, --grep-keywords <GREP_KEYWORDS>...
          Optional keywords for grep: defaults to "TODO" and "FIXME"
      --supported
          Print supported commands
      --debug
          Print out failed commands (instead of silently ignoring, good for debugging)
      --all
          Print out every result, useful for debugging
  -h, --help
          Print help
  -V, --version
          Print version
```

## Configuration files

Run `roulette` in the same directory as the configuration files, and the tools (e.g. `mypy` or `ruff`) should be able to find it.

## Supported Commands

- `git_grep`
- `rip_grep`
- `grep`
- `mypy`
- `ruff`
- `flake8`
- `clippy`

## Soon-to-be Supported Commands

What else? [Let me know.](https://github.com/Axel-Jacobsen/roulette/issues)
