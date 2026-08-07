# config-driven-sessions-review-k7

**Kind:** review-design
**Reviews:** config-driven-sessions-k6
**Producer launch:** {"producer":"config-driven-sessions-k6","session":"config-driven-sessions-k6","generation":"k6","harness":"codex","model":"sol-xhigh"}

## Goal

Adversarially review `config-driven-sessions-k6` and record concrete findings for its integration step.

## Context

- Attempt to disprove the design against `.grove/BRIEF.md`, especially direct
  execution, restart/finish recovery, configuration completeness, migration,
  wrapper commands, Git/jj parity, and removal of harness knowledge.

## Done when

- Findings identify contract violations, hidden harness coupling, ambiguous
  substitution or parsing behavior, unsafe recovery windows, and missing test
  coverage; no design fixes are applied.

## Findings

Reviewed artifact: `docs/specs/config-driven-sessions.md`, the reworked
`docs/specs/doubt-grove-review-mechanics.md`, `docs/adr/complete-session-configuration.md`,
and `docs/adr/grove-owns-escalated-review.md`, against `.grove/BRIEF.md`,
`CONTEXT.md`, and the code the design replaces. Cite these as
`config-driven-sessions-review-k7 D<n>`. No fixes applied.

Verdict: the configuration half of the design is complete and falsifiable — the
KDL shape, substitution grammar, aggregate diagnostics, validate-before-mutate
ordering, and removal list all answer `plan-review-k3`'s blocking set. The
lifecycle half is where it fails to close: two specs written by this same task
contradict each other on who may promote (D1); a declined finish leaf can
permanently starve later work with every removal path forbidden (D2); and
legacy-vs-current detection rests on a predicate that is never stated and that
the filename grammar makes ambiguous (D3). D1–D4 are the blocking set.

### Blocking contradictions

**D1 — The mandate and the current pick are permitted to diverge, and promotion
requires them to be identical.**
`config-driven-sessions.md:245-247`: "A leaf inserted earlier during the launch
window does not preempt a session already mandated to another stable handle; it
becomes the next iteration's work."
`doubt-grove-review-mechanics.md:101-102`: "Promotion accepts only the live
currently mandated producer, **which must also be the current driver-selected
leaf**." `src/tree_promotion.rs:72-80` implements exactly the second half — it
recomputes `pick_unlocked` and refuses anything else.
In the case the first spec explicitly blesses, a mandated producer that hits an
unexpected doubt cannot run `leaf-promote-chain`, which is the *only* escalation
`grove-owns-escalated-review.md:5-10` offers once its single in-session reviewer
is spent. There is no stated fallback, and `config-driven-sessions.md:512` puts
"promotion semantics" out of scope, so nothing in this design resolves it.
Compounding it: `grove-llm` cannot observe the mandate at all — it exists only
as prose inside `${prompt}` (`:131`, `:239`) — so "only the currently mandated
producer" is an agent-side obligation being asserted as a machine-checked gate.
The only predicate the CLI can actually evaluate is the current pick, i.e. the
one the other spec says may legitimately differ.
*Why it matters*: it is the one place the new "driver picks, session obeys"
split touches a fail-closed transaction, and it fails in the direction that
blocks legitimate work rather than permitting illegitimate work.
*Action*: choose one authority and reconcile all three sites. Either (a) drop
the pick gate and accept any live producer the caller names by handle — the
tree-shape gates at `:107-112` already carry the real safety property; or (b)
keep the pick gate and state that a session whose mandate is no longer the pick
must stop without promoting, which then contradicts `:245-247` and needs saying
there.

**D2 — A declined finish leaf can starve every later leaf, and the design
forbids all three ways of removing it.**
`config-driven-sessions.md:296-297`: "A finish leaf left live by an earlier
decline makes the liveness read non-empty and is therefore reused without any
allocation." `:307-308`: it is never retired, and "Generic terminal verbs reject
it". Same contract at `CONTEXT.md:17-23`.
The finish leaf is appended at the **root's** next position (`:286-288`).
`leaf-add` also appends at its parent's next free child position
(`content/SKILL.md:283-285`), so root-level work added *after* a decline sorts
**after** the finish leaf. Depth-first pre-order then returns the finish leaf
first on every subsequent iteration — forever — and each iteration launches a
`finish` session against a tree that has live work in it.
Every exit is closed: `leaf-retire` and `leaf-prune` refuse it (`:307-308`);
deleting the file is the one operation the tree scheme explicitly forbids
(`CONTEXT.md:660` — "that lowers the max and the next `leaf-add` re-issues a
live key"); and whether `leaf-insert` may target it is undecided (see D13).
*Why it matters*: the decline path is the *expected* outcome of a finish session
that surfaces remaining work, and it is unattended-loop-visible — the loop
relaunches a HITL leaf indefinitely.
*Action*: specify one. The cheapest is to make finish-leaf materialisation
symmetric: the driver **removes** a live finish leaf whenever the liveness read
finds any other live leaf (it is working-tree-only and uncommitted by
construction, so removal costs nothing but the key — state whether the key is
reclaimed or burned). Alternatives: pick prefers any non-`finish` live leaf; or
`leaf-insert` is explicitly permitted against it and the decline diagnostic says
so.

**D3 — Legacy-vs-current detection has no witness, and the predicate the
grammar implies misfires on ordinary slugs.**
`config-driven-sessions.md:316` ("An already-current tree is a no-op"), `:374`
("before ordinary **format detection**" — named, never defined), `:355-357` ("A
tree without a migration witness must be wholly legacy or wholly current. A
mixture of kind-bearing and body-kind leaves is diagnosed…").
The only available discriminator is the leaf grammar at `:199-206`: does the
filename start with a member of the closed kind set? That is not a reliable
signal, because a legacy slug may itself begin with a kind token. A legacy
`01-design-notes-k3.md` carrying `**Kind:** impl` parses as *current* with kind
`design`, slug `notes`, handle `notes-k3`. Two reachable failures:
(a) a tree whose legacy slugs all begin with kind tokens is classified current,
never migrated, launched against the wrong configuration target, and its handles
silently shift (`design-notes-k3` → `notes-k3`) — dangling every
`**Reviews:**` / `**Integrates:**` marker this same design preserves (`:217-218`),
every commit-message handle, and the sibling-claimant lookup at
`doubt-grove-review-mechanics.md:107-109`;
(b) a tree with only *some* such slugs is neither wholly legacy nor wholly
current, and the leaves in question are simultaneously "kind-bearing" and
"body-kind", so `:356-357`'s mixture rule does not even classify them — bare
`grove` refuses to drive with no stated remedy and no dual-format reader
(`:428-429`).
*Why it matters*: this is the gate every existing grove passes through exactly
once, unattended, and its failure mode (a) is silent.
*Action*: give migration a positive discriminator that does not depend on slug
content — a tracked one-line format marker written by the migration commit is
the obvious one, and it doubles as the "already migrated" fact `:355` currently
infers. Failing that, state the predicate precisely (e.g. migrate iff **no**
leaf parses as current **and** at least one body carries `**Kind:**`) and give
the mixed case a documented manual remedy.

**D4 — Nothing reports a configured command that starts and immediately fails.**
`config-driven-sessions.md:183-185` covers only "failure to resolve or spawn the
selected literal executable". `src/loop_driver.rs:461-465` deliberately ignores
the child's exit status ("the signal file — not the exit status — decides
relaunch"), and `:280-282` keeps that: an absent signal simply "stops without
changing tree state".
Today three pre-spawn guards catch the common instances of a session that never
really started; `:407-419` removes all three — the PATH pre-flight, the
per-kind model check, and the codex sandbox probe (`src/launch.rs:333-372`),
which exists *because* codex exits 1 in ~0 ms under a read-only sandbox and "the
loop then stopped on what looked to the operator like a mute non-signal exit".
With opaque targets the probe genuinely cannot be reinstated — that is the
accepted cost of the brief — but the design owes the replacement diagnostic, and
the new surface is strictly larger: a typo'd wrapper, an `env` that exits 127,
or a harness that refuses its own flags all now land in the same mute stop.
*Action*: specify that when a session ends with no completion signal the driver
reports the child's exit status and elapsed time, and that a non-zero exit
within a short window names the session kind, the configured word zero, and the
config path. Add it to the test seams at `:489-490`.

### Missing behavior

**D5 — `${prompt}`'s contents are specified two ways, and the difference decides
whether skill provisioning is still load-bearing.**
`config-driven-sessions.md:131` — "The complete embedded Grove workflow plus the
selected leaf's stable-handle mandate". `CONTEXT.md:8` — the binary "uses its
embedded **launcher prompt**". These are different artifacts: today's prompt is a
689-byte launcher (`content/prompts/continue.md`) whose whole job is to say "use
the grove skill", and the methodology plus its reference files
(`TASK-FORMAT.md`, `BRIEF-FORMAT.md`, `grilling.md`, `driving.md`) are read from
the provisioned dir.
If it inlines the workflow, `content/prompts/` and per-verb selection die and the
spec should say so. If it stays a launcher, the session depends on a skill dir
that `:430-434` now writes only for **already-installed known** harnesses — so a
configured command that is not one of the three known harnesses (the very thing
opaque targets are supposed to permit) gets no methodology at all, and no
diagnostic. The spec is also silent on `start.md` and `retire.md`, which lose
their verbs at `:259-263`.
*Action*: state exactly what `${prompt}` contains, which `content/prompts/`
files survive, and the behaviour when the launched target has no provisioned
skill dir.

**D6 — `grove-llm` resolution and the version-skew guard are unmentioned.**
The `${herdr_settings}` payload embeds an absolute `grove-llm` path
(`src/launch.rs:588-604`), and the driver refuses to start a session when the
`grove-llm` the agent would invoke differs in version from the driver
(`src/loop_driver.rs:167-188`; `Stop::VersionSkew` in `src/herdr.rs:104`,
`:163-166`). Neither appears in the lifecycle flow (`:37-51`) or the removed
list (`:405-419`), while `:412` removes "user-settable Grove executable …
overrides" — which is the seam both the guard and the suite use
(`GROVE_LLM_BIN`, `tests/loop_driver.rs:36`, `:147`).
Relatedly, `:412` does not distinguish *user-facing configuration* from
*internal test seams*. `$HOME` isolation already replaces `GROVE_SKILL_DIR`
(`src/provision.rs:88`), but `GROVE_KILL_GRACE`/`_KILL` bound every
completion-signal case at ≥2 s + 5 s (`src/loop_driver.rs:430-432`), and the
seam list at `:463-500` asks for many such cases.
*Action*: state how `grove-llm` is located, whether the version-skew guard
survives and where it sits in the iteration, and which internal seams remain
(and are documented as internal).

**D7 — Standalone `research` becomes unrepresentable *and* unmigratable, with no
escape.**
`:327-330` maps only "the first and second `research` children of an unambiguous
legacy vendor pair"; `:336-338` makes "a standalone `research`" a hard stop.
`research` is a legal kind today (`src/leaf.rs:81`, `content/TASK-FORMAT.md:258`)
and `leaf-add --kind research` writes one. Because there is no dual-format
reader (`:428-429`), such a tree cannot be migrated *or* driven — the user's only
recourse is to hand-edit filenames into a grammar the binary has not yet taught
them.
*Action*: state the remedy — mapping a standalone `research` to `research-a` is
the obvious one and costs nothing — or keep the stop and specify the exact
instruction the diagnostic prints. Also state whether a hand-authored lone
`research-a` is legal.

**D8 — The vendor pair's "two independently configured surveys" guarantee is
dropped silently.**
`leaf-add-pair` today requires `--harness-a`/`--harness-b` and refuses an equal
pair — that refusal is what makes "two corpora" a fact in the tree
(`content/SKILL.md:230-233`, `CONTEXT.md:111-116`). `:414` removes the flags and
`:219-221` moves the guarantee into configuration, where `research-a` and
`research-b` may name the identical command and Grove cannot tell, because
targets are opaque by construction (`:156-161`).
*Action*: record it as an accepted loss with its reason (it belongs in
`complete-session-configuration.md`'s considered options, beside the Herdr
inference entry), and state `leaf-add-pair`'s resulting signature — including
whether it survives as a distinct verb at all.

**D9 — Two of the three driver-side mutations are specified without the tree
lock every other mutator takes.**
`promotion-transactions-fail-closed.md:3-5` binds "every participating task-tree
command" to the shared/exclusive lock on the `.grove/` root descriptor. The spec
assigns it to migration only (`:362-363`), while finish-leaf allocation reads
`max(tree key) + 1` (`:286-289`) and root-init writes the scaffold
(`:268-270`) with no lock stated. A `grove-llm leaf-add` running in a second
terminal can therefore be issued the same key.
*Action*: state that all three driver-side mutations hold the exclusive lock,
and what the driver does when it is already held.

### Recovery and VCS gaps

**D10 — "a missing or invalid config leaves … a tree byte-identical" is false
across the mutate-then-reload window.**
`:187-193`. The driver validates, performs a mutation, then "loads the file
again before launch rather than reusing the pre-mutation value". A config edited
into invalidity between those two reads leaves a scaffolded, migrated (and for
migration, **committed** — `:379-380`) tree with no launch, which the sentence at
`:191-192` denies.
*Action*: restate the guarantee against the *first* read ("a config invalid at
the pre-mutation read leaves the tree byte-identical"), or drop the second load
and state that one validated snapshot governs the whole iteration.

**D11 — The Git deletion commit reuses a pathspec form that behaves differently
for a deleted directory.**
`:390-397` specifies stage-with-exclusion, then commit "with the same explicit
`.grove/` pathspec in only/path mode", and `:302-303` / `:308` reuse that shape
for the finish deletion. After `.grove/` is removed from the working tree, a
`--only -- .grove` pathspec matches nothing there; the deletion has to be staged
first and partial-commit semantics over a removed directory still need stating.
The initial-commit case (no `HEAD`) is also unstated, and it is reachable — a
grove can be started in a fresh `git init` tree. Note this *changes* today's
behaviour, which is `git add -A -- .grove` plus a plain commit
(`src/tree_migrate.rs:164-171`).
*Action*: give the exact command shape for both commits, including the deletion
and no-`HEAD` cases, and add a seam at `:491-492` asserting a deletion commit
lands with unrelated staged work preserved.

**D12 — "never separately committed" is not true under jj, and only
accidentally true under Git.**
`:305-308` and `CONTEXT.md:20-22`. A jj working copy *is* a commit: a finish leaf
left live by a decline is snapshotted immediately, and any later task session's
`jj commit` carries it. Under Git it is untracked until an agent's `git add -A`
sweeps it in. The end state still cancels on the success path, but the stated
invariant is what a reader will rely on when reasoning about D2's stale leaf.
*Action*: restate as "no commit is made *for* the finish leaf; its addition and
deletion cancel across the finish commit", and say what a task commit that
picks it up means.

### Ambiguous substitution and reservation rules

**D13 — `finish`'s reservation is stated once for three different verb
families.**
`:220-221` — "refused by every generic grow, retire, and prune operation" —
conflates refusing `--kind finish` at *creation* with refusing a finish leaf as
an *operand* (`leaf-retire`, `leaf-prune`, `leaf-insert`, `leaf-decompose`,
`leaf-promote-chain`). `doubt-grove-review-mechanics.md:104-105` independently
uses the operand sense for promotion.
*Action*: enumerate per verb. D2's escape hatch depends entirely on the answer
for `leaf-insert`.

**D14 — Three template-grammar cases are left undecided.**
`:127-141`, `:165-179`.
(a) `${herdr_settings}` is the only substitution with arity ≠ 1 (`:135`,
`:152-154`), yet nothing says whether a template may place further words after
it, or what an *empty* expansion does to a literal that was written expecting
it. The two-argument form also hard-codes claude's `--settings` spelling and
hook schema — confirmed policy (`CONTEXT.md:288-293`), so not a finding in
itself, but a target that splices it into a non-claude command gets a launch
Grove cannot diagnose and the spec should say so.
(b) "a substitution in word zero … [is an] error" (`:138-139`) does not say how
`env VAR=value prog` is counted, though the illustrative config at `:63` and
`:87` depends on those words.
(c) `${prompt}` is "Required exactly once" (`:131`), but the aggregate
diagnostic enumeration at `:173-179` never lists a missing `${prompt}` among the
template errors it must report.
*Action*: pin each; (c) is a one-line addition to the diagnostic list.

### Checked and sound

Recorded so `config-driven-sessions-integrate-k8` does not re-litigate them:
the nineteen-kind arithmetic and the `research-a`/`research-b` split are
consistent across spec, ADR, glossary and viewer contract; `${prompt}` is
correctly relaxed to "any nonzero argv position" (closes `plan-review-k3` R16);
configuration is validated before *every* tree mutation (closes R13 — D10 is
only the narrower second-read window); `**Producer launch:**` is in migration's
removal set (closes R11); terminal leaves, the `work` aliases, and the ordered
two-phase directory-then-kind conversion are all specified (closes R10b–d);
receipts, `GROVE_SESSION_TARGET`, diversity warnings and the structured routing
peek are removed coherently across ADR, both specs and the glossary (closes R17);
the review-ownership predicate is moved to the prompt mandate in both the ADR and
the doubt spec (closes R7); and the viewer's filename-only kind parse is recorded
as a breaking-change obligation (closes R18).

## Notes

Review target: claude / opus-5, against a producer launched on codex / sol-xhigh
per this leaf's `**Producer launch:**` line — materially diverse in both harness
and model. Recorded as a fact, not a gate; the design under review removes the
mechanism that emits it.

No in-session reviewer was materialised: a `review-*` leaf spawns none
(`grove-owns-escalated-review.md:9-11`).
