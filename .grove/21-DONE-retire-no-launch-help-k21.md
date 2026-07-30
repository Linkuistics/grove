# retire-no-launch-help-k21

**Kind:** impl

## Goal

Give `grove retire --no-launch` a description, and decide whether its dry run
should run the checks `grove do --no-launch` runs.

## Context

`RetireArgs::no_launch` (`src/cli.rs`) carries **no doc comment at all**, so
`grove retire --help` renders it as a padded blank row beside two described
options. Found by `retire-help-node-path-k20` executing *every* generated help
surface and scanning for undescribed options: this is the only one in the whole
CLI — both binaries, every subcommand.

Writing the description is not purely cosmetic, because the two dry runs are not
the same thing. `grove do --no-launch` is documented as *reporting readiness* and
deliberately resolves everything a launch would fail on — pre-flight plus the
kind/model peek, both moved above the early return by `no-launch-config-check-k20`
(a leaf of an earlier grove; see `src/launch.rs`), precisely because a dry run
that skipped them printed `ready` on exactly the half-configured environments the
checks exist to expose. `launch::retire`'s dry run does neither: it resolves the
harness, prints `would exec <bin> for retire (no-launch)` and returns before
loading the prompt. So either the description says honestly that retire's dry run
only reports the harness it would exec, or the same argument that moved `do`'s
checks applies here too and the behaviour changes. Answer that first; the help
text follows from it.

## Done when

- `grove retire --no-launch` renders a description that matches what it actually
  does, executed and read — not inferred from the source.
- The `do` vs `retire` asymmetry is either **fixed** (retire's dry run runs the
  same checks) or **stated** as deliberate, with the reason recorded where the
  next reader meets it.
- No generated help surface has an undescribed option — the scan
  `retire-help-node-path-k20` ran, re-run.
- `cargo test` passes.

## Notes

**Decided: fixed, not stated.** The rule was never `do`-specific —
*model-per-task-kind* records it as a property of the flag ("`--no-launch`
resolves the launch it declines to perform … it runs the identical code path
rather than a parallel config check") and carves out no verb. What *is*
verb-specific is the content: `grove retire` peeks no leaf and passes the harness
no model, so its residual asymmetry — the report names no leaf, kind or model —
is a fact about the verb, not an unchecked claim, and that half is **stated**
(help text, `docs/grove.md`, the code comment).

**The prompt is the finding, and it is unique to this verb.** `grove retire`
never provisions — `provision_all` has one caller, `do_grove` — so `load_prompt`
reads a global skill dir some *earlier* `grove do` had to have written. That is
the one launch dependency a user cannot see, and the old dry run returned
directly on top of it. Executed against a scratch tree: unprovisioned →
`Error: reading prompt …/retire.md`, exit 1, where it used to print `would exec
claude` and exit 0.

**The bin check is against `harness.exec_bin`, and this was the trap.** The two
verbs exec through **different seams**: `loop_driver::harness_bin` honours
`GROVE_HARNESS_BIN[_<NAME>]`, but `exec_harness` has none. So reusing
`preflight_check` would have checked a *different binary* than retire runs —
the exact defect class this leaf exists to close, arrived at by reaching for the
symmetric-looking helper. `preflight_check` is wrong here for a second reason
too: it sweeps every `GROVE_<KIND>_HARNESS` override, and retire routes on none
of them, so it would fail a dry run on config the launch is indifferent to.

**Verified end-to-end against a real codex, not a fake.** In an untrusted scratch
tree `grove retire --harness codex --no-launch` now hits the read-only sandbox
refusal with the full two-way diagnostic — the pre-flight `codex-grant-refused-k35`
added, which retire's dry run had never reached. `scripts/release-publish.sh`
already documents that a scratch tree is untrusted by construction, which is what
makes this reproducible rather than lucky.

**A shell scan of `--help` text is the wrong instrument, and this session watched
it fail.** The first attempt at re-running `k20`'s scan was awk over the rendered
output; it emitted `awk: newline in string` and *two false positives* (`-h,
--help` and `-V, --version`, both described) in the same run. Fifth generation of
the grep-trap lesson (`k17`'s `rg -E`, `k20`'s `rg -r`), with a new edge: the
scraper has to reproduce clap's **two layouts** — a multi-paragraph description
switches the *whole command* into long-help form, so adding this leaf's own
description changed how every sibling row renders. The guard that shipped walks
`clap::Command` instead, where the question is a fact. Falsified by mutation: with
the new doc comment stripped it fails naming `grove retire :: argument
`no_launch`` — the original defect, exactly.

**Externalized rather than absorbed:** `retire-harness-stamp-claim-k23`.
`RetireArgs::harness`'s doc is a verbatim copy of `StartArgs::harness`'s and
claims it stamps; `maybe_stamp` has one call site and it is in `do_grove`. Left
standing deliberately — it needs a decision (is the doc wrong, or should retire
stamp?), not a reword. It is also why this leaf's `--no-launch` description omits
the "It writes no stamp" clause `do`'s carries: vacuous where nothing stamps.
