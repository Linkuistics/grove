# grove-configuration-k13

## Goal

Delete Grove's own configuration code: the `grove config` commands, the shipped
configuration examples, and the session configuration with its per-checkout
delta and trackedness check.

## Context

- Root brief, requirements 2 and 3.
- `direct-dispatch-k4`'s notes *Where the design lands in the code* (the
  *Grove* entry) and *Records this design left for the leaf that deletes the
  machinery*.

## Done when

- `grove config show` and `grove config examples` do not exist, and `grove
  --help` lists neither.
- The session configuration module, the examples and every test of them are
  deleted. Grove reads no personal file and no checkout file.
- A record of configuration whose last citation from live code is gone is
  deleted, and its citations in the context map, the glossary, the architecture
  document and the books go with it. One the runner's templates still cite is
  left for `runner-templates-k14`.
- The overview and `grove-loop` books and their structure specifications
  describe what remains.
- `bash scripts/check.sh` passes.

## Notes

- Delete consumers first so each step compiles: the commands and examples use
  the session configuration, which uses the runner's templates.
- If this proves bigger than one session, the seam is the book: the commands
  and examples with the overview book, then the session configuration with the
  `grove-loop` book.
- There is no converter, no refusal while an old file is present, and no
  replacement command.
