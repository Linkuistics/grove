# draft — stage record

The draft is **not a scored stage**. There is no book without it and no
alternative to it, so no marginality is computable for it; this record exists to
fix the baseline the five editorial stages are measured against, and to account
for everything the draft's commits touched.

It is also the one stage the
[preregistration](../preregistration.md) permits a **commit range**: `jj-workspace-book-k25`
decomposed into one child per slice of the book's own sequence, so the draft is
every commit from the first drafting commit to the last inclusive, and its
baseline is the commit before the first of them. This record is appended to by
each child; `## Provenance` lists every commit in the range.

<a id="provenance"></a>
## Provenance

**Stage:** draft (stage 0).
**Baseline commit** — the parent of the first drafting commit, and the state the
whole range is diffed against: change id `pmyytxvyxmzzsmrxmovzuyxqqnxxurzr`,
`pilot-preregistration-k54: integrate all seven preregistration review findings`.

| # | Child | Change id | Book-directory digest after | Scope proved |
|---:|---|---|---|---|
| 1 | `orientation-k55` | `wnnkykqxxnotnpltmvnvnmkqtsuxzkqs` | `523b02550a3aef5fcb0d1161ae5ad5adeaaed352450d0f72e36e84f4b4b33d4d` | `--through no-dependencies` |
| 2 | `the-gate-k56` | `kvyvvxzyyvxtrynswrnqzqyloznzlovx` | `2f6288f285cabb89ab4cfbc0ced592e8cdd2efe813b2177b385fc1904a84dd9c` | `--through one-lane` |
| 3 | `subprocess-seam-k57` | `qyxstotksvmrnzqtwouvtsxxxtzskqxn` | `3a333b9028ab5d3971a6daa068cf199e89d65a6bf1f7a0c2376f8766ddad9975` | `--through nothing-ambient` |
| 4 | `namespace-k58` | `vuottxtuuktvwmqlzozukwoztnmrvvmo` | `d8beca1b843585d75b6a134b08e4411585eec292cbe1af03d6c0d424af9b49b5` | `--through no-consumer-vocabulary` |

The book-directory digest **before** child 1 has no value: the directory did not
exist. Digests follow the preregistration's recipe —
`find <dir> -type f \( -name '*.md' -o -name '*.toml' \) | LC_ALL=C sort | xargs
shasum -a 256 | shasum -a 256`.

**Preregistration read by this session:** SHA-256
`b9ee9f8e48b69170c7ecd9787b9373dfc42242895e91d37d8a181af6f09a694e`.

<a id="frozen-inputs"></a>
## Frozen inputs, re-checked

Every input the preregistration digested was re-computed at the start of child 1.
All matched except one, and the exception was mandated rather than accidental.

| Input | Verdict |
|---|---|
| `crates/jj-workspace/Cargo.toml`, `src/lib.rs`, `src/jj.rs`, `src/refusal.rs` | all four unmoved; the corpus was not touched |
| `docs/specs/walkthrough-books.md` | unmoved |
| `docs/specs/jj-workspace-book-structure.md` | unmoved |
| `docs/USAGE.md` | unmoved |
| `docs/walkthroughs/ordinal-fs-tree/` | unmoved |
| `CONTEXT.md` | **moved**, by mandate — see below |

**`CONTEXT.md` moved from `c7dde4d628a0f4cd8556b7c5238e1abe80d482b724f27efdcca4c9071e190e52`
to `c0d4698440f2d925f6c67563487add708100cd8188be025267039be09b733822`, and the
move was required by the task this stage executes.** The preregistration's
*Frozen inputs* lists `CONTEXT.md` as a standard the stages are judged against,
while its *Validity* section's allowlist explicitly permits the draft — and only
the draft — to touch `CONTEXT.md` anchors. Those two statements cannot both hold
once a book cites the glossary, and `jj-workspace` is the first book that does.
The change is recorded here rather than argued away so the report can classify it
by inspection: four term entries — *Loop control channel*, *Task commit boundary /
sealing*, *Driver lease* and *Stated VCS* — each gained one `<a id="…"></a>` line
and had their bold lead-in promoted to a `###` heading. No definition, `_Avoid_`
line or retirement note was altered, and no other entry was touched.

**Re-checked at the start of child 4, and unchanged.** All four corpus files,
all three specification and guide inputs, the `ordinal-fs-tree` precedent
directory and `CONTEXT.md` carry exactly the digests above — `CONTEXT.md` still
at its post-child-1 value, because chapter 4 reserves
`CONTEXT.md#driver-lease` and `CONTEXT.md#loop-control-channel`, and child 1
created both. This child touches no file outside the book directory except this
record and `.grove/`. The preregistration read by this child hashes to the same
`b9ee9f8e48b69170c7ecd9787b9373dfc42242895e91d37d8a181af6f09a694e`.

**Re-checked at the start of child 3, and unchanged.** All four corpus files, all
three specification and guide inputs, the `ordinal-fs-tree` precedent directory
and `CONTEXT.md` carry exactly the digests above — `CONTEXT.md` still at its
post-child-1 value, because chapter 3 reserves no glossary anchor and cites
neither the guide nor the glossary. This child touches no file outside the book
directory except this record and `.grove/`. The preregistration read by this
child hashes to the same
`b9ee9f8e48b69170c7ecd9787b9373dfc42242895e91d37d8a181af6f09a694e`.

**Re-checked at the start of child 2, and unchanged.** All four corpus files
carry the digests child 1 recorded, and `CONTEXT.md` is unmoved from the
post-child-1 value above: chapter 2 cites `CONTEXT.md#stated-vcs`, which child 1
had already created, so this child adds no glossary anchor and touches no file
outside the book directory except this record and `.grove/`.

The promotion to a heading is not decoration. `docs/specs/walkthrough-books.md`
requires a cited anchor to exist as an explicit anchor line **immediately
preceding a heading**, and `book-validation`'s `explicit_anchors`
(`crates/book-validation/src/markdown.rs`) implements exactly that: an anchor line
whose next line is not a heading is not an anchor and is reported as `M201`
against the manifest. An anchor before a bold paragraph would have discharged
nothing.

<a id="baseline"></a>
## Baseline

The draft has no before-state to cite, so this section stands where an editorial
stage carries `## Claims`. It is completed by the last child of the range.

**Page inventory, as of child 4.** Eight of the book's ten declared files exist;
the manifest declares all ten from the start, because a plan authored
incrementally cannot be compared against a prefix.

| File | Role | Slice | State |
|---|---|---|---|
| `walkthrough.toml` | manifest | — | complete: all 7 chapters, 4 roots, 11 blocks, 5 early uses, guide and glossary groups |
| `README.md` | contents | — | present; chapters 1–4 linked, chapters 5–7 listed as plain text |
| `01-orientation.md` | chapter | `no-dependencies` | written |
| `02-the-gate.md` | chapter | `one-lane` | written |
| `03-subprocess-seam.md` | chapter | `nothing-ambient` | written |
| `04-namespace.md` | chapter | `no-consumer-vocabulary` | written |
| `concept-index.md` | lookup | — | present, curated for the prefix |
| `source-index.md` | lookup | — | present; 4 roots, 11 ownership rows, 52 fragment rows, 9 early-use rows |
| `05-scope-and-commit.md` | chapter | `no-transactions` | not written; 2 blocks deferred |
| `06-refusal.md` | chapter | `no-remedy-of-its-own` | not written; 1 block deferred |
| `07-what-jj-owns.md` | chapter | `assembly` | not written; owns no source, final-only |

**Validation, as of child 4.** Scoped, not final:

```console
$ cargo run --quiet -p book-validation --bin book-check -- \
    --repo . --book docs/walkthroughs/jj-workspace \
    --through no-consumer-vocabulary --check all
valid: 4 files, 329 resolved lines, 369 deferred lines, final=false
```

329 + 369 = 698, the corpus line count the root brief froze; child 1 stood at
98 + 600, child 2 at 190 + 508 and child 3 at 271 + 427, and chapter 4's three
ownership blocks are the 58 lines that moved. `bash scripts/check.sh` reports 1 of 8 failing, and the
failure is `book-check` alone. Its nineteen diagnostics were read rather than
summarised: six `F003` on the three defers still outstanding, eight `F009` — one on
the ownership ledger, one on the early-use ledger, and one on each of its six
`pending` rows — two `M103` on the contents and on chapter 4's navigation lacking
a `Next`, and three `M101` on the unwritten pages. `cargo test --locked --workspace`, `cargo clippy`
and `cargo fmt --all --check` are green.
`bash scripts/check.sh` is red on `book-check` alone for every child of the range
but the last, because the script runs `--final` over every book root by discovery
and a prefix deliberately leaves later blocks deferred. The final result belongs in this
section and is written by `what-jj-owns-k61`.

<a id="out-of-charter"></a>
## Out of charter

None. The draft owns no taxonomy class, so no claim it makes can be out of
charter; the section is present because the record's shape requires it.

<a id="unclaimed-changes"></a>
## Unclaimed changes

The draft makes no claims, so this section accounts for every hunk of its diff
that touches book prose or an allowlisted file outside the book. As of child 1:

- **`docs/walkthroughs/jj-workspace/` (five new files).** The whole of the book
  as it stands. This is the baseline; none of it closes a taxonomy defect,
  because there is no before-state for it to close one against.
- **`CONTEXT.md` (four anchored headings).** Mandated by the task's `Done when`
  and by the book contract's *Outbound links*. Reader-facing but deliberately not
  claimed: it is a precondition for the book validating at all, not an
  improvement to the book.
- **`docs/ARCHITECTURE.md` (one ownership row).** Mandated: every book root under
  `docs/walkthroughs/` owes a row in the *Documentation ownership* table, and a
  repository test in `crates/grove/tests/reference_navigation.rs` fails the book
  without one.
- **`.grove/` bookkeeping.** The leaf's decomposition into a node with seven
  children, the children's task files, and two leaves cut for defects this
  session found. Allowlisted, and it changes no book prose.

As of child 2:

- **`02-the-gate.md` (new).** Chapter 2 entire: the one-lane thesis, the
  pointer-file premise, the three-ending worked resolution, and the two ownership
  blocks resolved into eight literal fragments. Baseline prose, unedited.
- **`source-index.md`, `README.md`, `01-orientation.md`, `concept-index.md`.**
  The ledger and navigation this slice is required to move: two defers replaced
  by inserts, two ownership rows turned `resolved`, ten fragment rows added, the
  `Workspace` early-use row turned `explained` and two rows added, chapter 2
  linked from the contents, chapter 1's two navigation lines given their `Next`,
  and eight concept-index entries. Mechanical and validator-driven; no prose on a
  page an earlier slice owns was touched.
- **`.grove/` bookkeeping.** This child's `DONE` rename and one leaf cut for a
  defect found while drafting. It changes no book prose.

As of child 3:

- **`03-subprocess-seam.md` (new).** Chapter 3 entire: the nothing-ambient
  thesis, the premise about how jj selects a repository and why there is no
  `JJ_*` counterpart to strip, the three-ending worked invocation, and the single
  81-line ownership block resolved into ten literal fragments. Baseline prose,
  unedited.
- **`source-index.md`, `README.md`, `02-the-gate.md`, `concept-index.md`.** The
  ledger and navigation this slice is required to move: one defer replaced by an
  insert, one ownership row turned `resolved`, eleven fragment rows added, the
  `jj::output` / `jj::produced_output` early-use row turned `explained` and one
  row added for the seam's three refusal constructors, chapter 3 linked from the
  contents, chapter 2's two navigation lines given their `Next`, and ten
  concept-index entries. Mechanical and validator-driven; no prose on a page an
  earlier slice owns was touched.
- **`.grove/` bookkeeping.** This child's running decision log and its `DONE`
  rename. It changes no book prose.

As of child 4:

- **`04-namespace.md` (new).** Chapter 4 entire: the no-consumer-vocabulary
  thesis and the argument against an enum of consumers, the premise that `.jj/`
  is the repository in jj's own vocabulary and that jj snapshots the working
  copy, the worked reservation of `grove` with its second call and its four
  refusals, and the three ownership blocks resolved into twelve literal fragments
  under three composites. Baseline prose, unedited.
- **`source-index.md`, `README.md`, `03-subprocess-seam.md`, `concept-index.md`.**
  The ledger and navigation this slice is required to move: three defers replaced
  by inserts, three ownership rows turned `resolved`, fifteen fragment rows added
  and the fragment index re-sorted into its required root, ascending-range, ID
  order, the `control_dir` early-use row turned `explained` and one row added for
  the namespace's two refusal constructors, chapter 4 linked from the contents,
  chapter 3's two navigation lines given their `Next`, and twelve concept-index
  entries. Mechanical and validator-driven; no prose on a page an earlier slice
  owns was touched.
- **`.grove/` bookkeeping.** This child's running decision log, one leaf cut for
  a defect found while drafting, and this child's `DONE` rename. It changes no
  book prose.

<a id="findings-not-fixed"></a>
## Findings not fixed

Two defects were found while drafting and neither was fixed here. Both are
outside the draft's charter and both would have put a non-book change inside the
draft's commit range, which the preregistration's *Validity* rules out.

1. **The book system's `[[early-use]]` group is unsatisfiable in scoped mode when
   a required row's first use is on a later chapter.** `check_early_uses`
   (`crates/book-validation/src/ledger.rs`) requires every manifest
   `[[early-use]]` row to appear in the ledger *and* requires each ledger row's
   first-use anchor to be found in a page present in the snapshot — but scoped
   mode forbids later pages from existing. `docs/specs/walkthrough-books.md` says
   the manifest is complete from the start, so the two rules disagree for any
   book whose second chapter forces an early use. Worked around here by declaring
   only the five rows whose first use is in chapter 1 and leaving the structure
   brief's other two to be added to the ledger by `the-gate-k56`, which is what
   the specification's "authors add further rows to the book's own ledger" already
   permits. Cut as `early-use-scope-k63`.
2. **`CONTEXT.md`'s glossary is not uniformly addressable.** Four of its
   sixty-one term entries now carry explicit anchors and fifty-seven do not. Cut
   as `glossary-anchors-k62`, placed after every crate book so the full reserved
   anchor set is known when the generalisation is made.

As of child 2, one more:

3. **The one external URL in production source names a documentation host that
   now redirects.** `crates/jj-workspace/src/refusal.rs:184` prints
   `https://jj-vcs.github.io/jj/latest/install-and-setup/` in the `NotRunnable`
   remedy, and that host answers `301 Moved Permanently` to `docs.jj-vcs.dev`.
   Not fixed here: the corpus is frozen, and the root brief requires a source
   change to land in one commit with every affected ledger, page and validator
   run — which is not this child's charter and would put a non-book change inside
   the draft's commit range. Chapter 2 links jj's glossary at the current host
   rather than propagating the stale one. Cut as `jj-docs-url-k64`, placed after
   every crate book so the pages that quote `refusal.rs` exist and can be
   re-proved in the same commit.

As of child 3, none beyond the three above. Chapter 3 quotes the stale
`install-and-setup` URL inside the `NotRunnable` message it reproduces, which is
finding 3 showing through to a reader rather than a new defect; `jj-docs-url-k64`
already owns it and already has to re-prove this page when it lands.

As of child 4, one more:

4. **`JJ_OWNED_NAMES` is one name short of what jj writes inside `.jj/`.** On jj
   0.44.0 a workspace created by `jj git init` and one created by
   `jj git init --colocate` both contain `.jj/.gitignore`, and the crate's
   reserved list holds only `repo` and `working_copy`. `control_dir(".gitignore")`
   therefore passes `validated_namespace` and is refused by `fs::create_dir_all`
   with `AlreadyExists`, which reaches the consumer as a `ControlDir` refusal
   telling it to check permissions rather than as the `Namespace` refusal naming
   Jujutsu. Observed against this crate rather than inferred. Not fixed here: the
   corpus is frozen, and the fix moves a line boundary inside the
   `namespace-owned-names-list` fragment, so it must land in one commit with the
   page that quotes it. Chapter 4 states the gap as an outstanding observable in
   both its worked example and its account of the reserved list — the list's own
   one-directional cost argument coming due, rather than a claim the book leaves
   unchecked. Cut as `jj-owned-names-k65`, placed beside `jj-docs-url-k64` after
   every crate book for the same reason.
