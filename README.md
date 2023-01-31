# Roulette!

*Fancy a game of Roulette?*

Rules:

- spin the wheel with "./roulette.sh"
- whatever TODO, FIXME, or type error you get, you must fix!
- total number of TODOs, FIXMEs, or type error must go down

Inspired by (i.e. copied from) Andreas Kling's [FIXME Roulette](https://www.youtube.com/watch?v=fk0EMHevbPs&list=PLMOpZvQB55bdRLT1IY-QD_U4DVp8NDeHo&index=1)

## Supported Commands

- `git grep -niE "(TODO|FIXME)"`

## Soon-to-be Supported Commands

- `mypy`
- `ruff`

What else?

## TODO

- how to find commands to use?
- Is UTF-8 OK for character interpretation?
- stdout is interpreted as `Vec<u8>` - is this the 'natural' way to manage terminal output?
