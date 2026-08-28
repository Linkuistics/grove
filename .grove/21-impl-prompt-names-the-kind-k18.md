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

`minimalism-k1`'s `## Deletion list`, *Reconciled* row 4: `src/prompt.rs`'s content
dependency (~270 of 324 lines) — `reference_file`'s nineteen-to-ten `match` and
`ending_file`'s nineteen-to-two `match` both go, and with them `prompt.rs`'s call
to `methodology::embed()`, which becomes grove's own signalling sentence. Row 2,
`src/methodology.rs` itself, is **not** this leaf's: two of its three call sites
are provisioning's and the build-pairing report's, and both live until
`delete-provisioning-k19`.

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
- **`src/methodology.rs` stays, and so do `--content-hash` and the build-pairing
  report.** They belong to provisioning's last live build and go at
  `delete-provisioning-k19`; see the note below. What goes here is `src/prompt.rs`'s
  *content dependency* — `reference_file`, `ending_file` and `${locations}` — which
  is `minimalism-k1`'s row 4 and needs nothing from `methodology.rs`.
- **The binaries are installed in this session and the session ends without
  signalling** — the root brief's `### The cutover sequence`, run in full. The
  behavioural probe for step 3 is the composed prompt itself: the new build's
  prompt names one `grove-<kind>` skill and carries the release version, where the
  old build's names a reference file and a `${locations}` list.
- `cargo test` and `cargo clippy --all-targets` clean; `tests/prompt.rs`
  reconciled; `CHANGELOG.md` updated.

## Notes

**Lands green**, and it **depends on `plugin-kind-skills-k17`**: the prompt may
not name a skill that does not exist. k16 decided the exact token to name —
plugin-namespaced or bare — and this leaf uses that decision rather than
re-deriving it.

**Why the delivery deletions moved to `delete-provisioning-k19`.** Provisioning is
still live here, and its per-iteration path calls `methodology::identity()`
(`src/provision.rs:53-77`), which the driver invokes before every transition
(`src/loop_driver.rs:116-128`). Deleting `src/methodology.rs` at this leaf would
leave a source dependency only the *later* k19 removes, so the workspace would not
compile — and the two retirements that go with it would be false while a build
still writes skill directories. The spec assigns both to *the leaf that deletes
provisioning* (`docs/specs/module-decomposition.md:776-777`), and the root brief's
rule that no ADR is rewritten ahead of the code that makes it true says the same
thing from the other direction. The cost-recording bullet that accompanied those
retirements moved with them.

**There was never a build-pairing guard to retire.** `report_build_pairing` prints
and returns `()` (`src/loop_driver.rs:550-576`); `docs/USAGE.md:164-177` states it
reports without refusing. Neither this leaf nor any other changes what a reinstall
does to a running loop, because a reinstall never did anything to one. The root
brief's cutover sequence carries the handoff that does work — the session ends
without signalling and the human restarts `grove`.

**`Kind` is still closed here.** Deleting the two `match`es does not open the
type; `open-kind-k20` does that, and it comes after this leaf because these two
are the last places the closed set is *interpreted*.
