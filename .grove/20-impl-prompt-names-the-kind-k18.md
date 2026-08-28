# prompt-names-the-kind-k18

## Goal

Make the prompt name `grove-<kind>` and publish grove's version in it, so the
driver interprets a kind **nowhere**, and delete the content dependency that
made it interpret one.

## Context

`docs/specs/module-decomposition.md`, decisions 9 (the prompt's three parts) and
10 (the published version).

**The prompt is three driver-authored parts and carries no methodology**: an
imperative naming `grove-<kind>`; the runtime facts — the selected handle, the
stated version control, and grove's published version; and grove's own signalling
contract. Its first part must reproduce the element measured as load-bearing in
`docs/research/wording-micro-test.md` — **one imperative naming one target**, so
the session performs no selection and has nothing to defer. Read that document
before touching prompt composition; the 0/10-vs-10/10 result is the reason, and
any redesign has to preserve the property or restate the evidence against it.

`minimalism-k1`'s `## Deletion list`, *Reconciled* rows 2 and 4:
`src/methodology.rs` (160 lines, 3 call sites) and `src/prompt.rs`'s content
dependency (~270 of 324 lines) — `reference_file`'s nineteen-to-ten `match` and
`ending_file`'s nineteen-to-two `match` both go.

## Done when

- `compose(&Mandate)` produces the three parts and nothing else. `reference_file`
  and `ending_file` are gone. The `${locations}` slot and the provisioned-skill
  list go with them.
- The published value is **the workspace's single release version**, riding in
  the runtime facts beside the handle and the stated version control. A release
  version orders and means something to a human, which the content hash never
  did. `grove --version` remains as a fallback, **not** as the mechanism — a verb
  would need the CLI on `PATH` and would fire only if the session thought to run
  it, which is the deferred read the micro-test measured.
- `src/methodology.rs`, `--content-hash` and the build-pairing report are gone.
- `docs/adr/one-build-owns-a-session.md` **retired** — no build writes a skill
  directory, so there is no pairing to report.
- `docs/adr/skill-delivers-the-methodology.md` **retired** — its delivery path
  ceases to exist.
- **The cost that record existed to prevent is recorded, not argued away.** Grove
  no longer guarantees the methodology is present, so a session can be launched
  pointing at a skill that is not installed. That is a message, not machinery:
  grove states the version it is, names the install command, and stops. The
  provisioned-skill list is dropped with provisioning and the gap is recorded
  where a reader meets it — a harness with a skill-loading affordance is
  unaffected, one without loses its fallback, and the reopen condition is **a
  session that cannot reach the methodology by the affordance alone**.
- `cargo test` and `cargo clippy --all-targets` clean; `tests/prompt.rs`
  reconciled; `CHANGELOG.md` updated.

## Notes

**Lands green**, and it **depends on `plugin-kind-skills-k17`**: the prompt may
not name a skill that does not exist. k16 decided the exact token to name —
plugin-namespaced or bare — and this leaf uses that decision rather than
re-deriving it.

**Retiring the build-pairing guard changes the loop's behaviour under a
reinstall.** Until now, installing a new build under a live loop stopped the loop
before the next iteration (`release.toml` records this, and
`grammar-separator-k15` relies on it). After this leaf it does not: a running
driver simply keeps running its own code. Say so in `CHANGELOG.md` and in the
retirement, because `open-kind-k20` reinstalls and its session needs to know the
loop will *not* stop for it.

**`Kind` is still closed here.** Deleting the two `match`es does not open the
type; `open-kind-k20` does that, and it comes after this leaf because these two
are the last places the closed set is *interpreted*.
