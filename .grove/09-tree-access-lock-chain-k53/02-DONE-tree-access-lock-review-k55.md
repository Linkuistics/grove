# tree-access-lock-review-k55

**Kind:** review-impl
**Reviews:** tree-access-lock-k52
**Producer launch:** {"producer":"tree-access-lock-k52","session":"tree-access-lock-k52","generation":"k52","harness":"codex","model":"sol-xhigh"}

## Goal

Adversarially review `tree-access-lock-k52` and record concrete findings for its integration step.

## Context

- This review is inspection-only. Inspect the producer's committed diff,
  source, specifications, and recorded verification evidence. Do not run test,
  build, lint, or format commands, edit production or test code, or redo the
  implementation.
- Record findings only. `tree-access-lock-integrate-k56` owns every fix and all
  post-fix verification.

## Done when

- Findings are recorded here with severity and concrete source or diff evidence,
  or an explicit no-finding result.
- The review relies on the producer's recorded verification evidence; no test,
  build, lint, or format command is run.
- No production or test code is changed.

## Findings

Inspected: commit `ozpkkwuwwlum` / `5b81eeb8536f` (`tree-access-lock-k52`) diff
and post-change source for `src/tree_access.rs`, `src/tree_lifecycle.rs`,
`src/tree_read.rs`, `src/tree_grow.rs`, `src/tree_promotion.rs`,
`src/tree_format.rs`, `src/llm_cli.rs`, `src/loop_driver.rs`,
`tests/tree_access.rs`, `tests/leaf_promote_chain.rs`, and `tests/pick.rs`;
`docs/specs/config-driven-sessions.md` sections "Process ownership and session
epochs" (340-512), "Fresh tree" (514-529), and "Test seams" (795-855);
`docs/specs/doubt-grove-review-mechanics.md` "Fail-closed transaction"
(130-150); `docs/adr/promotion-transactions-fail-closed.md`; `CONTEXT.md`
*Tree access lock* (632-652); `docs/ARCHITECTURE.md` (157-174). **The producer
recorded no verification evidence** (F4), so no conclusion below rests on a test
result. No code was run and no production or test file was changed.

### F1 — medium — the reference-taking grow verbs resolve their operand under a *separate* acquisition, so one command takes the tree lock twice and mutates a target resolved from a tree state it no longer holds

`src/llm_cli.rs:551-552`, `:581-582`, `:615-616`, `:624-625`. Each grow command
resolves its `<parent>` / `<target>` argument and *then* mutates, through two
independently guarded exported operations:

```rust
let parent_dir = resolve_parent(&grove_root, &args.parent)?;      // llm_cli.rs:551
let path = tree_grow::leaf_add(&grove_root, &parent_dir, &args.slug, kind)?;  // :552
```

`resolve_parent` (`src/llm_cli.rs:560-566`) falls through to
`resolve_ref_or_path` (`:731-749`), whose reference branch calls
`tree_read::resolve` — the *guarded* form (`src/tree_read.rs:320-323`:
`let guard = tree_access::read(grove_root)?;`). That guard is dropped when
`resolve` returns. `tree_grow::leaf_add` then acquires exclusive from scratch
(`src/tree_grow.rs:48`).

Trigger is precise: only a **key/slug/handle** operand takes the extra lock. A
path operand short-circuits at `existing_path` (`src/llm_cli.rs:755-765`, pure
filesystem, no lock), and `leaf-add .` short-circuits at
`src/llm_cli.rs:561-562` — which is exactly why
`every_existing_mutator_waits_for_promotion_before_touching_the_tree`
(`tests/leaf_promote_chain.rs:588-658`) still asserts a single waiting
diagnostic: every one of its five invocations happens to use a path operand or
`.`. The defect is latent behind the test suite, not contradicted by it.

Failure scenario: session A runs `grove-llm leaf-insert 14 earlier`. `resolve`
takes the shared lock, returns `.grove/05-impl-mid-k14.md`, releases. Before A
re-acquires exclusively, B runs `leaf-retire` on k14, renaming it to
`05-DONE-impl-mid-k14.md`. A then mutates against a path that no longer exists.
It fails safe — a path-not-found error rather than a wrong-entry mutation,
because `resolve` returns slug+key, which a renumber never changes — but the
operator gets a diagnostic describing a tree state that was never
simultaneously true, and the slice's serialization property does not hold for
the command.

Secondary, user-visible symptom: under sustained contention one command can
print `waiting for active Grove tree operation` **twice**. That contradicts
`docs/specs/config-driven-sessions.md:498` ("A contended **command** prints one
waiting diagnostic and then waits"),
`docs/specs/doubt-grove-review-mechanics.md:138-140` ("Exported operations
acquire exactly once… A contended **caller** prints one waiting diagnostic"),
and this leaf's own Done-when ("contention reports once then waits").

Contract threatened: Done-when bullet 3 — "Current pick/resolve/brief-chain,
grow, lifecycle, terminal, and promotion operations **acquire once** and pass
the guard into lock-neutral helpers". It is met at the module level and missed
at the command level, and this slice is the one that owns the property.

`tree_promotion::promote` already demonstrates the correct shape and is the
model to copy: it acquires once (`src/tree_promotion.rs:47`) and resolves its
producer reference *inside* the guard via `tree_read::resolve_unlocked`
(`:565`, `:575`).

Suggested direction: give the grow verbs a guarded entry point that takes the
raw reference string and resolves it through `resolve_unlocked` under its own
guard, the way `promote` does — leaving `resolve_ref_or_path`'s
path-then-reference precedence intact but moving it inside.

### F2 — medium — `leaf-insert` re-acquires a third time to lint cross-references, so the lint reports against a tree the mutation no longer holds

`src/llm_cli.rs:625` then `:647`:

```rust
let (path, renumbers) = tree_grow::leaf_insert(&grove_root, &target, &args.slug, kind)?;
…
tree_grow::surface_cross_refs(&grove_root, &renumbers, &mut stderr)?;
```

`leaf_insert`'s exclusive guard is a local (`src/tree_grow.rs:366`) dropped on
return; `surface_cross_refs` then takes a fresh shared guard
(`src/tree_grow.rs:454`) and scans every `.md` body for the *old* names in the
renumber log. Between the two, another process can retire, insert, decompose or
promote, so the lint can miss a reference that has since been written, or name a
file that has since moved — and it is the operator-facing output whose whole
purpose is accuracy about stale references (`src/tree_grow.rs:441-448`).

Combined with F1, a single `grove-llm leaf-insert <key> <slug>` can take the
tree lock **three** times and emit up to three waiting diagnostics.

Same root cause and same fix as F1: acquire once per command and pass the guard
into `surface_cross_refs_unlocked` (`src/tree_grow.rs:458`), which already
exists and already takes a plain `&Path`.

### F3 — medium — the producer recorded no verification evidence, so Done-when bullet 5 is unverified and this review cannot rest on it

The leaf `.grove/09-tree-access-lock-chain-k53/01-DONE-tree-access-lock-k52.md`
ends at an empty `## Notes` (line 39) with no `## Verification evidence`
section, and the commit message is a single line with no body:

```
tree-access-lock-k52: serialize task-tree access
```

Done-when bullet 5 claims "`cargo fmt --check` and `cargo test --locked` pass";
nothing records that either ran. The precedent exists in this same grove —
`session-kind-tree-k23` carries a `## Verification evidence` section recording
both exit codes and the test count, and `session-kind-tree-review-k24` cites it.
This review's own Done-when says it "relies on the producer's recorded
verification evidence", so the gap is load-bearing rather than cosmetic: for a
concurrency change, "the suite passes" is the only evidence that
Done-when bullet 5's "unchanged Git/jj behavior" holds, since the producer added
no new Git/jj test and rests that clause entirely on the inherited suite.

Action for `tree-access-lock-integrate-k56`: run both commands, record the
result, and treat bullet 5 as open until then.

### F4 — low — `brief-chain` picks and then reads the chain under two separate acquisitions

`src/llm_cli.rs:456` calls `tree_read::pick` (guarded, `src/tree_read.rs:48`)
and `:467` calls `tree_read::brief_chain` (guarded, `src/tree_read.rs:105`).
Between them the picked leaf can be retired, renumbered, decomposed or promoted,
so `brief-chain` can fail on a path it just printed as live, or emit the
ancestor chain of a leaf that is no longer the pick.

Lower severity than F1 only because both acquisitions are *shared*, so the
common contended case is silent and the failure is a clean error rather than a
wrong result. But this is the **bootstrap path every grove session runs**
(SKILL.md's Bootstrap step), and Done-when bullet 3 names `brief-chain`
explicitly. Same fix shape: one guard, `pick_unlocked` +
`brief_chain_unlocked` (`src/tree_read.rs:52`, `:109`), both already present.

### F5 — low — the driver still observes `.grove/` outside the lock to choose the launcher prompt

`src/loop_driver.rs:791`:

```rust
fn launch_verb(worktree: &Path) -> &'static str {
    if worktree.join(".grove").is_dir() { "continue" } else { "start" }
}
```

This is the one remaining unguarded task-tree observation in the binary — a
`grep -rn '"\.grove"' src/` outside `src/tree_*` returns only this and
`src/llm_cli.rs:721` (path construction, not an observation). The leaf's Goal
says "Move **every** current task-tree observation and mutation onto one
advisory lock"; this one was not moved.

Failure scenario: `root-init` creates `.grove/` and writes `FORMAT` **last**, by
design (`src/tree_lifecycle.rs:65-77`, `src/tree_format.rs:32-42`). A second
bare `grove` in the same working tree can therefore see `is_dir() == true`
mid-scaffold, route `continue`, and have its first `pick` fail with "Grove tree
format witness is missing … this is a legacy tree and must be migrated"
(`src/tree_format.rs:12-16`) — telling the operator to migrate a tree that is
seconds old.

Benign under a single driver, which is why this is low: the second-driver
refusal is `driver-lease-k31`'s (`.grove/11-driver-lease-chain-k30`), and the
`start`/`continue` routing itself is superseded by the spec's "Fresh tree" flow
(`docs/specs/config-driven-sessions.md:514-529`) in `lifecycle-cutover-k39`.
Recording it so neither slice assumes the other closed it.

### F6 — low — `TreeWriteGuard` now carries two different guarantees under one type, and `write_for_root_init`'s `root` field is dead and points at a directory that does not exist

`src/tree_access.rs:68-74`. `write_for_root_init` returns the same
`TreeWriteGuard` as `write` (`:42`) and `write_for_promotion` (`:55`), but
deliberately skips `require_grove_root`, `refuse_pending`, **and**
`tree_format::require_current` — correctly, and the doc comment says so. The
consequence is that a `TreeWriteGuard` no longer implies "exclusive lock on a
current-format tree with no pending transaction", which is what its two other
constructors establish and what every `*_unlocked` helper downstream assumes.
Nothing enforces the distinction; a future helper taking `&TreeWriteGuard` would
silently accept the weaker one.

Its `root` field is also `worktree.join(".grove")` — a path that by construction
does not exist when the guard is built. `root_init` never calls `guard.root()`;
it recomputes the identical path one line later (`src/tree_lifecycle.rs:59` then
`:61`), leaving the guard's own accessor dead and misleading.

Suggested direction (cheap, no behaviour change): make root-init's guard a
distinct type — or at minimum document on `TreeWriteGuard` which invariants a
guard carries and which constructor establishes them.

### F7 — low — Done-when bullet 4 holds only incidentally; no code or test pins it, and the widened lock raises the cost of losing it

Bullet 4 requires the driver to release its guard before configuration reload or
launch. It holds because the driver never takes an in-process guard at all: it
peeks by spawning `grove-llm kind --with-harness --json`
(`src/loop_driver.rs:1156-1165`), so the guard lives and dies inside that
subprocess. `grep -n 'tree_access' src/loop_driver.rs` returns nothing, and no
test asserts the property — `tests/tree_access.rs` covers root-init races,
reader/mutator contention, symlink aliasing and close-on-exec, but not
guard-release-before-launch or session-side mutation.

The spec lists it as an acceptance item
(`docs/specs/config-driven-sessions.md:821-823`: "Tree access guard release
before launch; close-on-exec descriptors; and successful session-side mutation
without deadlock"). Because the lock identity is now the *whole working-tree
root*, a later in-process peek would deadlock **every** session-side mutation
rather than only `.grove/` ones — the blast radius grew while the guard against
it did not.

Cheap regression guard for `tree-access-lock-integrate-k56` (or, if it belongs
with the loop, `lifecycle-cutover-k39`): a test that launches a fake configured
command which runs `grove-llm leaf-add`, and asserts it completes rather than
hanging.

### F8 — informational — `docs/ARCHITECTURE.md` still describes the lock as being on `.grove/`

`docs/ARCHITECTURE.md:159-161`: "Every steady-state task-tree reader holds a
shared **Tree access lock** on the open `.grove/` directory". That is the
identity this slice replaced. `CONTEXT.md:632-652`,
`docs/adr/promotion-transactions-fail-closed.md:3-8`, and
`docs/specs/doubt-grove-review-mechanics.md:135-141` already say working-tree
root; ARCHITECTURE is the last stale statement.

**Not work for `tree-access-lock-integrate-k56`** —
`architecture-records-reconciliation-k88`
(`.grove/19-durable-docs-reconciliation-chain-k70/…`) explicitly owns
`docs/ARCHITECTURE.md`. Recorded so that slice has the exact line.

### Verified without finding

- **Root-init serialization is correct and complete.** `src/tree_lifecycle.rs:59`
  binds `let _guard` — a *named* binding, not `let _ =`, which would drop the
  guard immediately and silently defeat the whole slice. The guard therefore
  spans the absence check (`:62`), `create_dir_all` (`:65`), the root brief
  (`:69`), the first leaf (`:76`), and `FORMAT`-last (`:77`).
  `concurrent_root_initializers_wait_before_observing_or_creating_the_grove`
  (`tests/tree_access.rs:107-141`) proves two concurrent `root-init` processes
  yield exactly one success, one waiting diagnostic each, and no `.grove/`
  observable before the lock releases.
- **No self-deadlock in any exported operation.** flock is held per *open file
  description*, so a nested acquisition inside one process blocks forever. Every
  guarded entry point delegates to a `*_unlocked` helper instead
  (`src/tree_read.rs:52,109,236,325`; `src/tree_grow.rs:458`;
  `src/tree_lifecycle.rs:103,185,264`; `src/tree_promotion.rs:63,227`), and the
  one place it would have bitten — `root_init` growing its first leaf — calls
  `leaf_add_unlocked`, not `leaf_add` (`src/tree_lifecycle.rs:76`). F1/F2/F4 are
  *sequential* re-acquisition, not nesting; they do not deadlock.
- **Alias identity holds for free.** `acquire_worktree` locks the descriptor
  from `File::open(worktree)` (`src/tree_access.rs:96-103`), so every path
  spelling of the same directory collapses to one inode and one lock, while
  distinct worktrees stay independent. `reader_through_a_symlink_alias_waits_…`
  (`tests/tree_access.rs:143-168`) covers it, and the deliberate refusal to
  canonicalise returned paths (`:84-88`) keeps `pick` output byte-identical on
  macOS's `/var` ↔ `/private/var`.
- **Close-on-exec is real and load-bearing.** Rust's `File::open` sets
  `O_CLOEXEC`; `worktree_lock_descriptor_is_close_on_exec`
  (`src/tree_access.rs:180-188`) asserts `FD_CLOEXEC`. This matters more than it
  looks: grow and lifecycle spawn `git`/`jj` *while holding the exclusive
  guard*, and a leaked descriptor in any descendant would keep the whole working
  tree locked after `grove-llm` exits.
- **Shared readers never mutate.** `tree_format::require_current`
  (`src/tree_format.rs:7-30`) is pure read — the only write path is
  `write_current_last` (`:35`), reachable only from `root_init` under the
  exclusive guard. Reordering `require_grove_root` to *after* acquisition
  (`src/tree_access.rs:33,44,57,121-126`) introduces no write under `LOCK_SH`.
- **Error-path compatibility survives the reordering.** "grove root not found"
  is still the message for a missing `.grove/`, now emitted post-lock by
  `require_grove_root`; `errors_when_grove_root_absent` (`tests/pick.rs:156-167`)
  and the skill's rootless-resume check ("`grove-llm pick` errors with 'grove
  root not found'") both still match. The new "working tree root not found"
  (`src/tree_access.rs:94`) is reachable only when the worktree itself is gone.
- **Fail-closed promotion is preserved.** `write_for_promotion` still skips
  `refuse_pending` so it can recover its own witness
  (`src/tree_access.rs:53-63`, `src/tree_promotion.rs:47-62`), the
  `PROMOTING-` prefix scan is unchanged (`:140-170`), and
  `docs/adr/promotion-transactions-fail-closed.md`'s process-interruption
  contract is untouched — matching the spec's "live-process serialization, not
  crash atomicity" (`docs/specs/config-driven-sessions.md:508-512`).

## Notes

The reviewer produces findings only; `tree-access-lock-integrate-k56` owns every
fix and all post-fix verification.

F1, F2 and F4 are one defect at three call sites — a command composing several
separately-guarded exported operations — and one fix shape closes all three:
acquire once at the command boundary and pass the guard into the `*_unlocked`
helpers that already exist. `tree_promotion::promote` is the in-repo model.
Fixing them also removes the only way a single command can print the waiting
diagnostic more than once, which is what the spec and this leaf both promise.

F3 gates the rest: bullet 5 is unverified, so integration should run
`cargo fmt --check` and `cargo test --locked` and record the outcome before
judging any other bullet closed.

F5, F7 and F8 need no change under this leaf's own scope — they are sequencing
notes for `driver-lease-k31`, `lifecycle-cutover-k39`, and
`architecture-records-reconciliation-k88` respectively.

Note for F1's fix: `resolve_ref_or_path`'s path-first-then-reference precedence
(`src/llm_cli.rs:725-749`) is deliberate and documented; moving resolution
inside the guard must preserve it, and `existing_path` is already lock-free so
only the `tree_read::resolve` branch needs to become `resolve_unlocked`.
