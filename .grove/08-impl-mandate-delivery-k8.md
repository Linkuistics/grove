# mandate-delivery-k8

## Goal

Make the composed mandate the thing a session actually receives, and reduce the
launcher to framing in the same slice. The driver composes by the kind it
already resolved; `content/prompts/continue.md` becomes `content/MANDATE.md`,
carrying framing and no instructions.

## Context

These are one change of what a session receives, not two. The launcher's five
instruction sentences are a duplicate of units composition delivers into the
same mandate, and that duplicate only becomes removable the moment the driver
composes — so reducing the launcher first would ship a framing sentence that
promises slices nothing produces, and reducing it later would ship one mandate
generation carrying both copies.

- **The driver.** `src/loop_driver.rs:196` `mandate_prompt` calls
  `crate::provision::continue_prompt()` and emits the launcher file whole. The
  kind is in hand at the call site as `selection.kind` and is unused. Compose by
  that kind, then the runtime facts — the selected handle and the stated VCS —
  **last**, where they are not buried under the generic bulk, joined by a blank
  line and nothing else.
- **The rename.** `content/prompts/continue.md` → `content/MANDATE.md`;
  `content/prompts/` goes with it (it held three launchers; two went with their
  lifecycle verbs and this is the third). This is a jj-enabled tree, so a plain
  rename.
- **The reduction.** The unit stays `kinds=* class=triggering` and keeps
  position 1; only its file, its id and its body move. It states what the
  mandate *is* — methodology sliced for this session's kind, complete with
  respect to triggering conditions, with `grove-llm methodology <id>` serving
  any deferred body — and carries no instructions.
- **The duplicate inventory is already confirmed complete**
  (`specialised-ending-k6`, confirmed claim 4): the launcher's instructions map
  to six `kinds=*` units — bootstrap, decompose, retire, commit, signal,
  finish — at `content/SKILL.md` marker lines 125, 204, 425, 525, 556, 574.
  Exactly one clause has no composed replacement, "use the grove skill", and it
  is the one framing replaces.
- **Callers to reconcile.** `src/provision.rs:27` `continue_prompt()` loses its
  only caller. `tests/provision.rs:256`
  `exactly_one_launcher_is_embedded_and_provisioned` is a claim about a launcher
  this slice renames.
- **Provisioning stays live.** `content/` is still swept whole, so the renamed
  file still reaches the skill directories and `content/SKILL.md`'s YAML
  preamble still has to keep working.

## Done when

- Every session's `${prompt}` is the composed mandate for its kind followed by
  the driver's runtime facts, with a blank line as the only join and no
  driver-authored framing sentence anywhere in it.
- `content/MANDATE.md` exists, carries one `kinds=* class=triggering` framing
  unit and no instructions; `content/prompts/` is gone;
  `provision::continue_prompt()` and its test are reconciled rather than left
  dangling.
- The golden per-kind snapshots and
  `tests/methodology.rs::the_embedded_unit_set_is_pinned_complete` are updated
  for the renamed unit — the snapshots moving is the drift signal working.
- Provisioning is otherwise untouched.
- `cargo test` is green.

## Notes

**One branch on session kind deliberately survives this slice.** `skill-signal`
is still `kinds=*` and still states both endings, so a composed mandate still
hands every session an `if` on its own kind — one slice later than the launcher
did. `session-ending-k9` removes it. Do not pre-empt that here, and do not split
any unit in this slice: splitting an ending while the launcher's conditional
sentence is still shipping would deliver both endings twice over.

**The behavioural check is deferred, as the brief says.** This is a meta-grove
across the build boundary — the driver change reaches no session in this loop
until the binary is rebuilt and installed, and provisioning is still live
besides. Verify by the composed mandates and the loop-driver seam, never by
watching a session behave differently.
