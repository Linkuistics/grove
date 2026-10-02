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

## Decisions (running log)

**One session, not two.** The book seam in the notes was not needed: the
commands, the examples, the session configuration and both books fit.

**Both configuration ADRs go here; the specification and the reference stay.**
`complete-session-configuration` and `untracked-configuration-delta` were cited
from live code only in `session_config.rs` and its test, so this leaf removes
their last citation and deletes them with it. No live code cited
`docs/specs/modular-configuration.md`, `docs/design/modular-configuration/`,
`docs/CONFIGURATION.md` or `docs/configuration-forms-audit.md` by path except
`reference_navigation.rs`'s guide list and one release script, and the grammar
they describe is still the runner's templates', so they are left for
`runner-templates-k14`. Their links into what this leaf deleted are unlinked
and nothing else in them is touched.

**`docs/examples/modular-configuration/` is deleted.** It was the bytes
`grove config examples` installed and had no other consumer.

**`Error::diagnostics` goes with the JSON report.** Its only caller was
`config_json`, and it downcast to the session configuration's own error.

**`grove`'s own argument scan is gone.** It existed to emit usage errors as JSON
for `config show --json`; `Cli::parse()` is what is left.

**The glossary loses only what this leaf removes.** The **Configuration delta**
entry and the two sentences on the `grove config` commands. The rest of the
**Grove configuration** cluster describes the runner's templates and is
`runner-templates-k14`'s, as its *Done when* says.

**The `grove-loop` book loses chapter 18 and renumbers.** The validator requires
a chapter's file prefix to equal its position, so chapters 19 to 21 became 18 to
20, and every count in the closing chapter that included the deleted chapter was
restated.
