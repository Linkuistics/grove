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
- ~~**The binaries are installed in this session and the session ends without
  signalling** — the root brief's `### The cutover sequence`, run in full.~~
  **Re-derived and answered no; see `## Why this leaf does not install anything`
  below.** The root brief requires each remaining leaf to re-derive the label
  rather than inherit it, and the matrix comes back clean.
- `cargo test` and `cargo clippy --all-targets` clean; `tests/prompt.rs`
  reconciled; `CHANGELOG.md` updated.

## Notes

**Lands green**, and it **depends on `plugin-kind-skills-k17`**: the prompt may
not name a skill that does not exist. k16 decided the exact token to name —
plugin-namespaced or bare — and this leaf uses that decision rather than
re-deriving it. It is written down in `plugins/grove/README.md`, *The token a
prompt names*: **both spellings of one target in one imperative** — ``Load the
`grove-<kind>` skill now — on Claude Code, where plugin skills are namespaced,
that is `grove:grove-<kind>`.`` One imperative, one target, so the micro-test's
load-bearing element survives; grove branches on no harness and grows no
registry to branch on. The glossary carries it as *Namespaced invocation*
(`plugins/CONTEXT.md`).

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

## Why this leaf does not install anything

The root brief's standing note is explicit that the five-leaf cutover list is
**not inherited**: *"each remaining leaf must re-derive whether it is a cutover
leaf rather than inherit the label, and the test is the matrix k6 ran: is there a
cell where the *installed* build meets the tree this leaf leaves and fails?"*
This leaf ran it, and the answer is no cell — for a simpler reason than k6's.

**This leaf leaves the tree untouched.** k6's tree-visible change was deleting
`.grove/FORMAT`; k15's was renaming every filename onto a new grammar. This one
changes what a *launch prompt* says. It adds no file to `.grove/`, removes none,
renames none, and changes no filename grammar, so the installed 19.4.0 build
reads the tree after this commit exactly as it read it before. There is no cell
to fail in.

**Nor does anything this leaf changes reach the running driver.** The old driver
composes the old prompt, and it re-provisions its **own** embedded `content/`
into `~/.claude/skills/grove` every iteration — so a session it launches is told
to load the `grove` skill and `references/<kind>.md`, and both are there,
written by the process that named them. The delivery the old build performs is
self-consistent and complete; it is simply the old one. Nothing is stranded and
nothing half-lands.

**Step 2's cost is real and buys nothing here.** A published release meets every
grove on this machine — `grammar-separator-k15` found four others — and the
brief requires checking the new build read-only against each before publishing.
That work exists to stop a release stranding a tree it can no longer read. No
tree is read differently by this build, so there is nothing for the check to
find.

So `prompt-names-the-kind-k18` comes off the cutover list the way
`delete-migration-k6` did, and **this session signals normally**. The
non-signalling exit is a deliverable of a genuine cutover leaf, not a habit.

**What this hands `delete-provisioning-k19`.** That leaf inherits an *un*-shipped
prompt change, and it must run its own matrix rather than this finding. Its cell
to check is the one this leaf does not have: it deletes `content/` and
provisioning, and a still-running 19.4.0 driver would then compose a prompt
naming a `grove` skill that the new build no longer writes — reaching whatever
that machine's `~/.claude/skills/grove` last held. Whether that is a failing cell
turns on what is on disk at the time, which is exactly the sort of thing to
measure rather than predict.

## What the in-session review changed

The leaf's one review allowance was spent on a single fresh context asked to
break the whole change across six named axes. Thirteen findings; the ones that
moved the artifact:

- **The signalling contract's grammar was the defect, not its content.** The
  first draft stated the ordinary ending as the sentence's main verb — *run
  `grove-llm complete` as your last action … unless your kind's skill states a
  different ending* — which puts back in all nineteen prompts the imperative the
  deleted `ending_file` existed to keep out of one, and rests its repair on the
  skill being read, which is the argument the too-late test rejects. The verb is
  now subordinate to the kind's own ending. The compound residue with decision
  10's un-installed-skill case is recorded in `src/prompt.rs`, in decision 9, and
  in the CHANGELOG, with a reopen condition.
- **`content/references/finish.md` was made to lie by this leaf** and nothing
  spanned the two channels to catch it. It still told live sessions the three
  outcomes were inlined in their prompt; provisioning ships that file until k19,
  so a `finish` session would have gone looking for a table that is no longer
  there. Fixed in place.
- **Two deleted guards were restored, both mutation-verified.** The unspent-
  placeholder sweep went with `src/prompt.rs`'s test module — the reviewer showed
  `{kindX}` reaching all nineteen prompts with 46 tests green — and the
  skill-name coupling was being asserted against *directory* names, where a
  harness registers the frontmatter `name:`. Both now fail on exactly those
  mutations, and the three-part test went back to whole-string equality so a
  footer cannot ride along.
- **The mirror-column reader was widened too far.** `**none — <any prose>**`
  admitted a cell contradicting its own class; the two real spellings are
  enumerated instead.

Also fixed: stale *Load predicate notation* and test-seam text in
`docs/specs/corpus-rule-ownership.md` still describing the deleted composition as
current, `rustfmt`, and the one uncovered restatement — `content/SIGNAL-FINISH.md`
against `grove-finish/SKILL.md` — recorded in `plugins/grove/README.md` rather
than answered with machinery for a thing that lives one more leaf.

The reviewer independently recomputed all nineteen re-fitted budget rows and
found them exact, and found nothing against `Workspace::resolve`.
