# drop-step-suffix-k18

## Goal

Execute the decision `step-suffix-redundancy-k10` reached: **remove the step
suffix from a composed shape's slug, in both shapes.** Guidance prose, one
generator (`leaf_add_pair_unlocked`), the glossary, and the tests pinned to
either.

There is **no tree migration and no format change**. Both spellings remain legal
filenames, the five-field grammar is untouched, and every existing leaf keeps the
slug it was created with — including the ones in this very tree.

## The decision, verbatim

**The kind field is the canonical statement of a leaf's role. The slug names the
artifact and does not restate the kind.**

So the review chain is cut as `<stem>` / `<stem>` / `<stem>` — three leaves that
share one slug and differ by kind and key:

```
02-design-mandate-delivery-k2.md
03-review-design-mandate-delivery-k3.md
04-integrate-review-design-mandate-delivery-k5.md
```

and `leaf-add-pair <parent> <stem>` emits:

```
NN-research-a-<stem>-k<n>.md
NN+1-research-b-<stem>-k<n+1>.md
NN+2-combine-research-<stem>-k<n+2>.md
```

All five markers — `-review`, `-integrate`, `-a`, `-b`, `-combine` — are a 1:1
restatement of the kind they sit beside. Each was a second, **unvalidated** source
of truth for a fact grove already parses and routes on; nothing stops
`leaf-add <parent> foo-review --kind impl`, and when the two disagree the slug
lies while the filename tells the truth.

## The reasoning (this is the durable record — no ADR)

`step-suffix-redundancy-k10` applied the three-part when-to-write test
(`linkuistics:decision-records`) and the decision **fails test 1**: it is cheap to
reverse. No migration, no format change, both spellings legal, reversal is prose
plus one function. Miss one of the three and there is no ADR to write.

**That makes the prose you write the whole durable record**, so it must say *why*
and not just *what*. Write the reason down where the naming lives; do not merely
delete the old paragraph and leave a silence someone refills in six months. The
predictable re-invention is concrete: the first person to hit ambiguous
`resolve <stem>` on a chain will propose putting the suffix back.

### Claim A — "bare slugs stay unique" — real, small, and never an invariant

- Grove enforces **no** slug uniqueness anywhere. Verified by search: nothing in
  `src/` rejects duplicate slugs, at any level or tree-wide.
- `resolve` was *built* for collisions. `Resolution::Ambiguous`
  (`src/tree_read.rs:346-360`) lists every match as `[key] <path> (retired)`.
- **The listed path carries the kind.** So after the change,
  `grove-llm resolve mandate-delivery` prints the whole chain with each step's
  role spelled out. For "show me this chain" that is now one command where it used
  to be a `find`; only "give me the producer specifically" costs a second lookup.
- **No machine path uses a bare slug.** The driver's mandate, `**Reviews:**` /
  `**Integrates:**` lines, commit messages and grow-verb targets all use
  `<slug>-k<key>` or a bare key, and both resolve through the terminal key —
  unambiguous by construction, since keys are unique tree-wide.
- Residual cost, accepted: `path=$(grove-llm resolve <stem>)` in a script now
  yields empty stdout and a non-zero exit for a chained stem. Scripts should use
  keys, which the guidance already says.

### Claim B — "surviving commit handles" — answered structurally

The claim: after `.grove/` dies, a chain leaves three commits whose handles differ
only by an opaque key, with nothing left to say which was the review. Two answers,
and the second is the load-bearing one.

1. **The sentence defends the stem, not the suffix.** `content/SKILL.md` says the
   stem is kept *"so bare slugs stay unique and surviving commit handles still
   name their artifact once `.grove/` is gone."* Dropping the suffix **keeps the
   stem**, so the handle still names the artifact. What it loses is the *role*,
   which that sentence never claims. `content/TASK-FORMAT.md:292-298` states the
   alternative it was actually written against: shortening to bare `review` /
   `integrate`, *"where `review-k14` names a role and no artifact."* That is not
   the alternative on the table.

2. **The role survives in the commit itself, by construction.** Grove mandates
   Retire-then-Commit: every task commit contains the leaf's `DONE` rename, and
   the leaf filename carries the kind. `.grove/` teardown removes the tree from
   the **tip**, not from **history**. Verified against an already-torn-down grove:

   ```
   $ jj show --stat 64f6a3f90db6
   review: disprove build pairing implementation (one-build-owns-a-session-review-k20)
   ...iew-impl-one-build-owns-a-session-review-k20.md} | 0
   ```

   So "which of these three was the review" is one `git show --stat` away, in any
   repository, forever. Rename detection is not required — without it you get a
   delete/add pair, both kind-bearing.

   **This deliberately does not rest on the commit-subject prefix.** This
   repository does prefix its subjects (`review:`, `impl:`, `design:`), which
   would answer the claim here — but it is undocumented in the methodology,
   applied inconsistently (`review:` vs `review-design:`, `integrate:` vs
   `integrate-review:`), local to one repo, and absent from older commits. Answer
   2 needs no convention at all, so **do not** add a commit-subject rule to the
   methodology as compensation. That would re-introduce the same unvalidated
   restatement one layer along.

### The barred argument, and why it is deleted rather than relocated

`content/SKILL.md:315-317` argues a terminal suffix *"keeps stem-mates together
in a directory listing; `review-<stem>` would sort every review beside every
other review."* That is an argument for **suffix over prefix**, against a
different alternative, and the task file barred it from standing in for either
claim above.

It is also **false on its own terms**, which is why it should go rather than move:
a leaf name begins with `NN`, so a directory listing sorts by position and never
by slug. A prefix would scatter nothing in `ls`, and both spellings glob
identically under `*<stem>*`. Delete it; do not rewrite it.

### The pair — same answer, and uniformity is what saves prose

The Notes on `step-suffix-redundancy-k10` asked whether the answer differs for the
pair, given that `leaf-add-pair` creates all three eagerly. It does not, and the
reason is this grove's own north star:

- `-a` / `-b` / `-combine` are exactly `research-a` / `research-b` /
  `combine-research`. Same 1:1 restatement. The letters carry **no** semantic
  content beyond the kind, which is precisely what makes them redundant rather
  than descriptive.
- A "chain drops it, pair keeps it" answer would have to **add** a special-case
  paragraph explaining the asymmetry. Removing from both **deletes** four sites
  outright. In a grove whose thesis is deleting methodology prose, the special
  case costs more than it buys.

**One real consequence, and it belongs to the pair alone.**
`content/TASK-FORMAT.md:107` says a research producer writes
`docs/research/<slug>.md`. With identical slugs, `research-a` and `research-b`
would write the **same file** and the second would clobber the first — destroying
the two independent corpora the pair is paid for. Settle it by deriving the
discriminator from the **kind**, which is the decision's own principle:

| kind | writes |
|---|---|
| `research-a` | `docs/research/<slug>-a.md` |
| `research-b` | `docs/research/<slug>-b.md` |
| `combine-research` | `docs/research/<slug>.md` — the union |

A solo `research-a` with no pair writes `-a.md` and that is the whole record: no
special case, and adding a `-b` survey later renames nothing. Today's prose does
not say what the combine step writes at all, so this is a net clarification at
roughly equal length, not added prose.

## Surface — the exact edit list

Verified sites. **Verify exhaustively before you finish** — this list was compiled
by a design session, not by an editing pass.

### Danger: do not damage the kind names

`review-<producer>`, `integrate-review-<producer>`, `research-a`, `research-b` and
`combine-research` are **kind labels** and must not change. A naive
`-review` → `` replacement destroys them. The discriminating pattern for a *slug
suffix* is the marker immediately before the key or at the end of a slug —
`-(review|integrate|combine|a|b)-k[0-9]` — and in prose, `<stem>-review` and
friends. Grep both ways and read every hit.

### Production code — the one generator

- `src/tree_grow.rs` — `leaf_add_pair_unlocked` (~line 246): the three
  `format!("{stem}-a")` / `-b` / `-combine` become the bare `stem`. Its doc
  comment (~lines 231-236) states the suffix rule and the peers rationale; rewrite
  it to the new rule. `validate_slug(stem)` stays — it is validated in its own
  right, and that reason survives.
- The **name-length** bound moves. `tests/tree_grow.rs`-adjacent inline tests near
  `src/tree_grow.rs:1330` compute `NN-combine-research-<stem>-combine-k<key>.md`
  as `stem+34`; the longest generated name shrinks by 8, so the pinned 233-char
  stem case changes. Recompute rather than nudging the constant.
- **The review chain has no generator.** Each step is a hand-written
  `leaf-add --kind`, so nothing in `src/` changes for it.

### Methodology (`content/`) — the durable record

- `content/SKILL.md:312-317` — *"The stem gets a step suffix, not a prefix"*.
  Replace with the short rule plus its reason (see *The reasoning*): the kind
  states the role, the slug names the artifact, and the slug does not restate the
  kind. Delete the stem-mates/sorting sentence outright (it is barred **and**
  false). Update the two `leaf-add` command lines at `content/SKILL.md:257-258`
  and the `leaf-insert` line at `:370`.
- `content/TASK-FORMAT.md` — the shapes section (~`:280-298`), especially the
  *"The suffix goes on the end"* bullet: this is the naming home, so this is where
  the **why** belongs in full. Also `:107` (the research doc path table above),
  and the worked filename examples throughout.
- `content/driving.md:139-142` — the `leaf-add-pair` worked example, and
  `:154-158` *"The stem gets suffixes"*, which deletes.
  **Pre-existing bug, fix while you are here:** `:139` says the pair produces
  *"a `sync-protocols-pair/` node"*. Both shapes are flat siblings and neither
  gets a node directory; that prose is stale independent of this change.

### Glossary

- `CONTEXT.md:379-382` (**Review chain** / **vendor pair**) — the clause
  *"named off a shared stem with a terminal step suffix … which keeps every step's
  [[Work-item handle]] unique and still naming its artifact after `.grove/`
  dies."*
  **This is factually wrong today and must not be carried forward in any form:**
  a [[Work-item handle]] is `<slug>-k<key>` and keys are unique tree-wide, so
  handles were **always** unique with or without the suffix. Only *bare slugs*
  could ever collide. Reduce to "named off a shared stem", and let the kind carry
  the role.

### Docs — sites the task file's surface list missed

- `docs/USAGE.md` — `:100` (tree illustration), `:131-133` (*"`auth`,
  `auth-review`, `auth-integrate` above"*), `:200`, `:204`, `:263`.
- `docs/specs/doubt-grove-review-mechanics.md` — `:57`, `:75`, `:81`, `:107`
  (a `**Integrates:** sync-design-review-k21` example), `:155`.
- `docs/ARCHITECTURE.md` — `~:354` says both shapes are *"flat siblings named off
  a shared stem"*, which **stays true**. Check `#task-tree-scheme` (`:233`) and
  `#task-kind-taxonomy` for any restatement of the suffix; a light touch or none
  is the expected outcome.

### Tests

- `tests/composition_guidance.rs:122,516` — pin the exact string
  `grove-llm leaf-add <parent> <stem>-review --kind review-<producer>`. Move with
  the prose.
- `tests/composition_verbs.rs` — ~50 sites. Two different kinds of hit, and they
  are **not** treated alike:
  - **Generated pair filenames** (`03-combine-research-survey-combine-k3.md`,
    `:572`, `:1219-1266`, `:1381-1386`) — these assert the generator's output and
    **must** change.
  - **Hand-written chain fixtures** (`sync-review`, `sync-integrate`) — these are
    arbitrary slugs and remain legal. Update them anyway, because the module
    header (`:2-9`, `:78`) says the fixtures *illustrate the convention*; a
    fixture teaching a convention grove no longer teaches is a slow leak.
- `tests/session_kind_guidance.rs:441,828,851,863`; `tests/leaf.rs`;
  `tests/session_kind_tree.rs`; `tests/reviewed_producer_lifecycle.rs` —
  check each; most are kind labels, not slug suffixes.

### Explicitly out of scope

- **No tree migration.** `src/tree_migrate.rs` handles legacy *formats*; this is
  not one. Its fixtures are historical strings and stay.
- **No `.grove/` renames**, including in this tree. Existing leaves keep their
  slugs; that is the point of the convention/grammar distinction.
- **No commit-subject convention** added to the methodology (see Claim B).
- **No ADR** (see *The reasoning*).

## Done when

- `leaf_add_pair_unlocked` emits three bare-stem slugs, and its doc comment states
  the new rule.
- `content/SKILL.md`, `content/TASK-FORMAT.md`, `content/driving.md` and
  `CONTEXT.md` no longer teach the step suffix, and **`content/TASK-FORMAT.md`
  states the reason it went** — the kind is canonical, the slug names the
  artifact, a second unvalidated statement of the same fact can disagree with it —
  not merely the new spelling.
- The methodology says explicitly that **both spellings remain legal filenames and
  no existing tree is invalidated**, so a reader meeting an old suffixed slug
  knows it is fine.
- The research doc path table above is recorded in `content/TASK-FORMAT.md`,
  replacing the bare `docs/research/<slug>.md`.
- `driving.md`'s stale `sync-protocols-pair/` node claim is corrected.
- `CONTEXT.md`'s false handle-uniqueness clause is gone, not reworded.
- `cargo test` passes; `cargo build`'s embed gate passes (this edits `content/`,
  which the gate parses).
- No `.grove/` leaf is renamed and no migration code is touched.

## Notes

- **Consider cutting `review-impl` as your last act.** The decision itself is
  settled and `step-suffix-redundancy-k10` deliberately spent no reviewer on it —
  the human stated a position, the counter-arguments were answered on their own
  terms, and the change is cheap to reverse. The execution risk is a different
  thing and is real: this is a wide prose sweep across ~13 files where a careless
  pattern match silently damages **kind labels**, which *are* grammar. Judge it on
  what the sweep actually did.
- This leaf runs **before** `classification-k9` on purpose. Its surface is
  `content/SKILL.md` and `content/TASK-FORMAT.md` — 76,418 of the 139,136 bytes
  that node classifies. Finish it, or the classification marks units over prose
  that then moves.
- `content/` is the embedded methodology, so `cargo build` parses what you write.
  A malformed unit marker or a broken `defers=` reference fails the build —
  treat a build failure here as your own edit, not as flaky infrastructure.
