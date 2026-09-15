# finish-k16

## Goal

Propose the complete finish cycle and wait for explicit human confirmation.

## Done when

- Promote durable material from the grove briefs.
- Run `grove-llm finish-commit finish-k16`.
- Run `grove-llm complete --done` as the last action.

## Decisions (running log)

The human confirmed the finish cycle and added advancing main, pushing to the
Git remote, a minor release and local installation. Complete those operations
before the final signal. The viewer design is already preserved in usage and
architecture documentation; correct the glossary's obsolete no-viewer statement.
All six final book validations passed for that correction. The release is
21.1.0, following 21.0.0, using the default colocated workspace for the cut.
