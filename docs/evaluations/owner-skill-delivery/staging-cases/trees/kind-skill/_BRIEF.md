# word-tools — brief

## Goal

A disposable grove for one staging trial. Its one leaf is small, so that
the session measures the loading route rather than the work.

The product is two small Bash tools in this repository:

- `words.sh` prints the number of words in each file that it is given.
- `longest.sh` prints the longest line of each file that it is given.

Each tool has a check script that makes a temporary file, runs the tool on
it and compares the output.

## Done when

- Both tools and their check scripts exist and pass.
