# owner-switch-dry-run-k271

## Goal

Build the dry-run instrument and its read-only comparison command. The dry
run executes a plan once in a disposable copy of HOME, including full
restore, and identifies each input by content.

## Context

- The section "The rehearsed plan" of `docs/specs/owner-skill-delivery.md`
  owns the contract. "Test seams" lists its controls. D1 requires the
  content identity of each file that a plan runs.
- owner-switch-contraction-k270 leaves the program without rehearsal
  admission. The switch consumes prepared values, not a passing record.
- `scripts/owner_switch_rehearsal.py` builds disposable layouts for the
  fixture suite. Reuse its mechanics where they fit.
- owner-switch-rehearsal-k255 executes this instrument on the real plan.

## Done when

- The dry run executes the ordinary writes, hook commands, probe cleanup,
  final comparison and full restore. It also tests `native-off` without a
  switch backup. It changes no real configuration, launchd job or login
  item.
- Its result holds a machine-readable input table. Each row names a logical
  input, its executed path, its content digest and its role. The rows cover
  the plan, program, invoked scripts, imported local helpers, command inputs,
  interpreters and external tool versions.
- The dry run digests each subject before and after execution. A changed
  subject invalidates the run.
- The comparison command reads the real prepared inputs through the same
  instrument and reports differences row by row. It permits only the
  declared prefix substitutions, with equal content and versions.
- Fixture controls cover each case of "Test seams". A changed helper at the
  same path fails. Equal content at a declared prefix passes. An unknown
  substitution fails.
- The result also holds per-probe outcomes, the restore comparison and its
  limits. A fixture result claims no real-harness acceptance.

## Notes

The comparison command and its result stay readable outside `.grove/`. The
owner runs it before each real attempt, including after finish.

## Decisions (running log)

- The instrument is a new program, `scripts/owner_switch_dry_run.py`, with
  three entries: `run`, `inputs` and `compare`. It uses only the standard
  library, like the switch program. It reads the prepared plan and program
  from `<home>/<state>/prepared/`, as the switch and `mac-check` do.
- `run` copies the relevant layout from the source HOME into OUT/home. The
  layout is the plan's write set, restore paths, inventory items and roots,
  the manual, `<state>/prepared`, each plan path under HOME, each discovered
  script or helper under HOME, and each `--include`. The copy keeps bytes,
  mode, ACL, attributes and link text. No file content is rewritten.
- The executed plan differs from the source plan only in its declared path
  fields: each `inputs` path, each command argument, `accepted.staging` and
  each `retained` path. In those fields, the source HOME prefix becomes the
  disposable HOME. The comparison normalises both plans the same way.
- Three substitution names exist: `home` (implicit), `grove-keg` and
  `ravel`. `run --prefix NAME=DIR` declares a dry-run prefix. `compare
  --prefix NAME=DIR` supplies the real prefix. Another name is a usage
  error, so an unknown substitution cannot be declared.
- Logical names come from the plan's structure, not from paths: `plan`,
  `program`, `interpreter:$python`, `input:<role>`,
  `command:<field>/<name>:<index>`, `helper:<parent> <token>`,
  `interpreter:<parent>`, `tool:<name>` and `source:<id>`. A path that
  differs without a declared prefix is therefore an unknown substitution,
  not a missing row.
- Helpers are discovered statically. A Python script contributes each
  imported module that exists beside it. A shell script contributes each
  absolute or `$HOME` path token that names an executable or a script.
  Python string literals are not scanned, because they name configuration
  that the run changes.
- Tools are claude, brew and mise from PATH, and ravel and grove from
  `$(brew --prefix)`, as the program finds them. `--tool` adds more from
  PATH. Each tool row holds its canonical path, content digest and
  `--version` line. Sources are the realpaths of
  `$(brew --prefix)/share/ravel/skills/<id>` with a tree digest.
- The fixture task keeps the name `owner:switch:rehearse`. The dry run gets
  its own tasks, `owner:switch:dry-run` and `owner:switch:compare`.
