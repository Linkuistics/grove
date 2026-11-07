# notes-tool-k1

## Goal

Add `notes.py` at the root of this repository. It is a command-line tool that
other agents run to keep short notes. Write it in Python 3 with the standard
library only. It keeps the notes in the file `notes.json` in its working
directory.

It has three subcommands:

- `add <text>` adds one note.
- `list` shows each note.
- `remove <id>` removes one note.

## Done when

- `notes.py` exists at the repository root and is executable.
- A run of `add`, then `list`, then `remove`, then `list` shows each change.
  Remove `notes.json` after the check, so that only the tool is committed.
