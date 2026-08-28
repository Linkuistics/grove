# decomposition-k5

**Integrates:** decomposition-k3

## Goal

Amend `docs/specs/module-decomposition.md` so the decomposition is implementable
at the public seams it promises. Resolve every finding from the adversarial
review of producer commit `4d4f2dd5`; this leaf integrates the design review, it
does not implement the decomposition.

## Context

**Reviewed artifact:** `decomposition-k2` at `4d4f2dd5`.

## Findings

### 1. The preserved research-pair verb contradicts kind opacity

The spec preserves `leaf_add_pair` in the thirteen-verb loop surface
(`docs/specs/module-decomposition.md:395`) while requiring that the machinery
contain no kind enumeration and name only `requirements` and `finish` literally
(`docs/specs/module-decomposition.md:494`). The current contract has no kind
inputs and its only distinguishing state is the fixed
`research-a`/`research-b`/`combine-research` array
(`src/task_grow.rs:80`, `src/task_grow.rs:105`, `src/task_grow.rs:137`,
`src/llm_cli.rs:382`). The ordinary add and insert CLI also default to the
literal `impl` (`src/llm_cli.rs:378`, `src/llm_cli.rs:400`). The spec neither
removes these literals nor gives the pair an opaque replacement interface, so
the requirement and preserved verb set cannot both be implemented. This is the
sixth compiled kind site the producer's five-site account missed.

### 2. `Vacancy::initialize` cannot create the distinguished root brief

The design assigns all task-tree filesystem writes to the store, then gives
`initialize` only `Vec<NewEntry<N::Parts>>`
(`docs/specs/module-decomposition.md:99`). A `NewEntry` can create only a
positioned leaf or node (`crates/ordinal-fs-tree/src/ops.rs:36`), while Grove's
`TaskName::Parts` deliberately excludes `BRIEF.md`; the brief is the
distinguished, parts-free `TaskName` variant (`src/task_name.rs:190`,
`src/task_name.rs:247`). This is exactly why current root initialization writes
the brief outside the store (`src/tree_lifecycle.rs:347`,
`src/tree_lifecycle.rs:384`). With no distinguished-content input or equivalent
operation, the loop must retain a direct `.grove/BRIEF.md` write and violate the
proposed ownership boundary.

### 3. `delete -> Report<N>` cannot report what the spec promises

`WriteGuard::delete` promises to report every removed name through the existing
`Report<N>` (`docs/specs/module-decomposition.md:105`). That report has only
created and renamed buckets (`crates/ordinal-fs-tree/src/report.rs:19`,
`crates/ordinal-fs-tree/src/report.rs:27`,
`crates/ordinal-fs-tree/src/report.rs:38`) and no removed representation.
Moreover, deletion removes foreign filesystem names that the store deliberately
cannot parse as `N`, so `Report<N>` cannot name every removed entry even if a
third bucket is added. The postcondition needs a deletion result whose domain
matches recursive root removal, or a narrower stated postcondition.

### 4. The configuration amendment cannot preserve eager template validation

The spec says every template rule remains document-eager while only key
presence becomes just-in-time (`docs/specs/module-decomposition.md:248`), but
`Templates::load(primary, overlay)` receives no slot vocabulary; requiredness
and valid slot names arrive only at `expand(key, slots)`
(`docs/specs/module-decomposition.md:281`). Current eager validation depends on
knowing that vocabulary at load: it rejects unknown/embedded substitutions and
enforces `${prompt}` exactly once plus optional-slot cardinality
(`src/session_config.rs:436`, `src/session_config.rs:488`,
`src/session_config.rs:548`). The proposed interface therefore cannot make all
of those failures eager.

The same change also alters, rather than leaves untouched,
`untracked-configuration-delta`: that record's safety argument requires the
personal file to remain complete regardless of the delta
(`docs/adr/untracked-configuration-delta.md:3`), while the proposed resolver
allows a key to exist in either source and defines no way to require its
presence in the primary file. The design must state the revised safety
property and include this ADR in reconciliation.

### 5. The public runner and loop surfaces are not complete crate interfaces

The runner signatures use an undeclared `Argv`, and expose `Token(String)` with
neither a public field nor an accessor even though the caller must interpret its
content (`docs/specs/module-decomposition.md:286`,
`docs/specs/module-decomposition.md:303`). The loop's separately built
`grove-llm` binary is given only a comment naming thirteen verbs, with no public
signatures or result/error types for any of them
(`docs/specs/module-decomposition.md:395`). These are not compilable consumer
seams, so the producer's mechanical done-when (types and signatures sufficient
to implement each crate without inventing another boundary) is not met.

### 6. `jj-workspace::Workspace::control_dir` still contains Grove policy

The crate is claimed to be fully domain-free, but its public method is specified
as the place "where a lease file may live"
(`docs/specs/module-decomposition.md:343`). The implementation being moved
currently obtains that fact by hard-coding `.jj/grove`
(`src/repo.rs:118`), under which the loop writes `driver.lease`,
`session.epoch`, and `signal-*` (`src/driver_lease.rs:13`). Returning the raw
`.jj` directory would instead make those generic filenames collide in jj's
administrative namespace. The seam needs to expose a genuinely VCS-shaped
administrative root or accept an opaque consumer namespace; as written, the
crate cannot both preserve the control-path guarantee and remain Grove-free.

### 7. The ADR reconciliation set omits records the target design makes false

The deferred reconciliation list in `minimalism-k1` does not include
`one-live-driver-per-working-tree`, although that record specifies embedded
provisioning, Git and jj lanes, the same-device gate, `.jj/grove`, and the
Git-or-jj lost-result recovery path
(`docs/adr/one-live-driver-per-working-tree.md:1`,
`docs/adr/one-live-driver-per-working-tree.md:94`). It also omits
`grove-binds-without-the-plugin`, whose opening current-state claim is that the
binary sweeps its own `content/` into harness skill directories
(`docs/adr/grove-binds-without-the-plugin.md:1`). Both become false under this
design. Finding 4 also makes `untracked-configuration-delta` move despite the
spec calling it untouched. Finally, dropping plain Git is a hard-to-reverse VCS
decision, but the records being retired leave no current-state ADR explaining
why jj-only is the chosen boundary after the transient spec is deleted. The
target minimum coherent set and the implementation ownership of each rewrite
must be stated before planning.

## Done when

- Every finding above is resolved in `docs/specs/module-decomposition.md`, or
  rejected there with evidence that satisfies the cited contract.
- The corrected public types and signatures are sufficient for the four library
  crates, the two thin binaries, and the plugin to be planned without inventing
  another architectural boundary.
- The ADR reconciliation section names every record that will be amended,
  reworked, retired, or added when implementation lands.

## Notes

The codebase-memory index could not be created because its fixed cache directory
failed `chmod 0700`; review evidence therefore came from exact source reads and
targeted `rg` fallback, not graph coverage.

## Decisions (running log)

**All seven findings verified against the cited sources before triage, and all
seven are real.** None was noise, none was a contract stated unclearly, and none
was accepted as a visible trade-off — every one named a contract the spec itself
imposes and could not satisfy. Verification was exact source reads
(`src/task_grow.rs`, `src/llm_cli.rs`, `src/task_name.rs`, `src/tree_lifecycle.rs`,
`src/session_config.rs`, `src/repo.rs`, `src/driver_lease.rs`,
`crates/ordinal-fs-tree/src/{ops,report,name}.rs`) plus a read of all seventeen
records in `docs/adr/`, since the graph index remains unavailable.

**Finding 1 — the pair verb is generalised, not deleted.** Its atomicity is a
real guarantee (one snapshot, contiguous ordinals, consecutive keys, all-or-none)
and the review attacked the *kinds*, not the atomicity. So `leaf-add` takes an
ordered list of kinds and appends them as one unit; the pair becomes three tokens
the methodology spells. Rejected: deleting the verb and letting the skill call
`leaf-add` three times, which reintroduces the live-prefix hazard the verb's own
contract exists to exclude. The seventh site — `--kind` defaulting to `impl` on
add and insert — is removed outright, because a default is a literal that
produces a *wrong* leaf rather than an error.

**Finding 2 — `initialize` gains a distinguished-content input.** Bytes, not
parts, and named by `EntryName::distinguished()`, which the trait already carries
and which promotion already writes through. So the ownership boundary holds with
no new trait method and `entry-name-is-the-only-seam` is untouched. Rejected:
letting the loop keep its own `BRIEF.md` write, which is the boundary violation
the finding names, and would put it at the first operation of every fresh grove.

**Finding 3 — `delete` returns `Removed { root, entries }`, paths not names.**
A third `Report<N>` bucket cannot name what deletion removes, because deletion
reaches the foreign entries the domain declines to parse as `N`. Paths are the
honest postcondition and the whole of what the operation knows.

**Finding 4 — the slot vocabulary moves to `load`, and the delta's safety
argument is restated per-kind.** Every current template rule is a rule about slot
*names*, so a loader without the vocabulary cannot check any of them eagerly and
decision 6's amendment would silently widen from *presence* to *everything*.
Separately: the all-nineteen rule was what made a partial second source safe, so
its per-kind replacement has to carry that weight explicitly — **a key resolves
only if the primary declares it; the overlay overrides and never supplies**.
`untracked-configuration-delta` therefore moves too, stating that as its own
property rather than borrowing one that no longer exists.

**Finding 5 — `Argv`, `Token`'s accessors, both error types, the conformance
outcome and all twelve verb signatures are written out.** Three shapes made
explicit while doing it: a reading verb takes `Tree` and a writing verb takes
`TreeWrite`, so the lock is in the signature; a search that matched nothing
answers `Sought` rather than an option each verb re-interprets; and every verb
returns the paths it wrote, because a session names them in a commit message by
hand. The loop's `open`/`Opened` becomes `read`/`write` mirroring the store's,
which is what lets `root_init` take the vacancy and be unable to run over a live
grove.

**Finding 6 — `control_dir` takes a namespace.** The postcondition *where a lease
file may live* cannot be stated without naming the consumer, which is the tell.
Rejected: returning the administrative directory raw, which puts the consumer's
generic filenames into a namespace the version control system owns and extends.

**Finding 7 — a full `## ADR reconciliation` section, accounting for every one of
the seventeen records.** The review named three omissions; enumerating the whole
set rather than checking the named ones found four more —
`corpus-rules-have-one-owner` and `restatement-declares-its-class` (their
register and reachability edge are the embedded corpus, which ceases to exist),
`grove-does-not-stage-its-own-renames` (Git lane and migration), and
`bulk-marks-are-not-atomic` (an implementation pointer only). Two records are
**added**, both clearing `ADR-FORMAT.md`'s three-part test where the spec's own
rewrite would otherwise drop the argument: `jj-is-the-only-lane` and
`a-kind-is-an-open-token`. The spec itself is stated **not** to be a retirement —
its subject is a spec's own grain, and the leaf that lands the last decision
rewrites it to current state.

**The problem statement's "matched in five places" is corrected at the summary
layer, not only in decision 5.** That count has now undercounted twice, so it is
replaced by a pointer to the enumeration rather than by a third count.

**No new producer review chain is cut.** Every finding was repairable at the
interface the spec already fixes; none required the design to be rethought. The
in-session reviewer allowance was not spent — the session is under a standing
instruction not to dispatch subagents, so the two claims that would have earned
it (the restated delta safety property, and that a load-time vocabulary makes
every current template rule eager) were checked directly against
`docs/adr/untracked-configuration-delta.md` and `src/session_config.rs` instead.
