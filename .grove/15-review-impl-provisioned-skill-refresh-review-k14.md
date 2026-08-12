# provisioned-skill-refresh-review-k14

**Reviews:** provisioned-skill-refresh-k9

## Goal

Try to **disprove** the contract `provisioned-skill-refresh-k9` committed: that
the skill/CLI boundary is a *build* boundary, that zero skew is what the design
delivers, and that `tests/provision.rs`'s new check is the enforceable remainder.

## Why this was cut

Not because the finding was doubtful — the diagnosis is settled and reproducible
(installed skill is byte-identical to `content/SKILL.md` at tag `v17.0.0`; the
installed `grove-llm` is `17.0.0` and still exposes both removed verbs; the
`v17.0.0` tag is this grove's branch base). It was cut because the *argument
built on top of it* went wrong three times in one session, each time caught only
on re-reading:

1. The first draft argued skew was safe in one direction ("an older skill names
   only verbs its binary still has"). False — removals make an older skill name
   verbs a newer binary has dropped, which is this repo's own `leaf-add-chain`.
2. The correction then overclaimed the other way ("row 2 is this repository's
   own state"). Also false: skill v17 sits beside binary v17, so there is no
   skew today; the ingredients exist, the pairing does not.
3. The new test's first scanner was line-based and silently dropped the wrapped
   invocation at `content/SKILL.md:434`.

Three errors of the same species — a confident claim about a mechanism, checkable
in under a minute, wrong. That is the signal for a fresh context.

## Specific things to attack

- **The two-property zero-skew argument** (`docs/ARCHITECTURE.md` §The boundary
  is a build, not a commit): "one embed per build, and Grove as the only writer
  of these directories". Is *only writer* true? Consider a harness that ships its
  own bundled grove skill, a marketplace/plugin install, the `linkuistics`
  plugin's install script, and a user's own edits. `provision_target` refuses an
  unstamped foreign dir — but it *replaces* a stamped one unconditionally. Does
  any non-Grove writer produce a stamped directory?
- **"A driver never re-execs, so per-iteration provisioning would extract
  identical bytes."** Verify against `loop_driver.rs` and `launch.rs`. Is there
  any path that re-enters `bare_grove`, or any way the embed could differ between
  iterations of one process?
- **The claim that provisioning runs exactly once per invocation.**
  `launch.rs:14` is the only call site found by grep. Confirm nothing in the
  migration/recovery transition (`tree_lifecycle::transition_driver_to_current`)
  or the finish path provisions again.
- **The scanner's reading rule** (`scan_instructed_verbs`). Look for false
  negatives beyond the wrapped case: a verb inside a markdown table cell, split
  by an inline code-span boundary (`` `grove-llm` `leaf-add` ``), hyphenated
  across a wrap, or written as `grove-llm  leaf-add` with two spaces. And false
  positives: prose that legitimately writes `grove-llm ` followed by a lowercase
  word that is not a verb.
- **`distinct.len() >= 8`** — is a floor of 8 meaningful against a corpus of 11
  distinct verbs, or is it loose enough to pass a badly broken scanner?
- **Glossary grain.** `CONTEXT.md`'s new **Embedded methodology** entry may be
  explaining mechanism rather than defining a term; the file's own preamble says
  "Definitions only. How a seam works belongs to `docs/ARCHITECTURE.md`". Judge
  whether it should shrink to a definition plus the two `_Avoid_` lines.
- **The root brief's correction note.** `.grove/BRIEF.md` now carries a
  "Corrected by `provisioned-skill-refresh-k9`" paragraph. Is recording the
  reversal in the brief right, or should the brief simply state the true reason
  and let the commit history hold the correction?

## Out of scope

The shared-skill-dir clobber is filed as `shared-skill-dir-clobber-k13`; do not
decide it here. Rebuilding or reinstalling the running binary is explicitly out
of scope, as it was for `k9`.

## Verification already done (re-check, don't redo)

`cargo test` (548 + integration suites, all green), `cargo clippy --all-targets
--all-features -- -D warnings` clean, `cargo fmt --check` clean. The new test was
mutation-checked twice: an inline removed verb and a *wrapped* removed verb each
made it fail with file, line and verb, and it passed again on restore.
