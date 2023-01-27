#! /bin/bash

# Fancy a game of Roulette?
#
# Rules:
# - spin the wheel with "./roulette.sh"
# - whatever TODO, FIXME, or type error you get, you must fix!
# - total number of TODOs, FIXMEs, or type error must go down

# 'TODO's and thoughts
#
# - It would be sweet to be able to add / configure tools (e.g. mypy, whatever else)
# - Configure FIXME / TODO whatever else?
# - How to make it cross platform? Where is shuf supported?


our_problems=$({ mypy . --no-error-summary; git grep -niE '(FIXME|TODO)'; })

echo "number of 'FIXME's, 'TODO's, and type errors: $(echo "$our_problems" | wc -l)"
echo "$our_problems" | shuf -n 1
