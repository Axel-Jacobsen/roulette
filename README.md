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

## Supported Commands

- `git grep -niE "(TODO|FIXME)"`
- `mypy`

## Soon-to-be Supported Commands

- `ruff`

What else?

## TODO

- how to find commands to use?
- Is UTF-8 OK for character interpretation?
- stdout is interpreted as `Vec<u8>` - is this the 'natural' way to manage terminal output?
- how to deal with config files for the commands? e.g. mypy.ini
