# plan-review-k3

**Kind:** review-requirements
**Reviews:** plan-k1

## Goal

Adversarially review `plan-k1` and record concrete findings for its integration step.

## Context

- Review `.grove/BRIEF.md`, the resolved Grove language in `CONTEXT.md`, and the
  producer task. Treat the confirmed decisions as the contract, not suggestions.

## Done when

- Every requirement is checked for missing behavior, falsifiability,
  contradictions, accidental solution detail, migration gaps, and recovery
  gaps.
- Findings are concrete and scoped for `plan-integrate-k4`; no fixes are applied.

## Findings

Reviewed artifact: `.grove/BRIEF.md`, the `CONTEXT.md` language rewritten by
`plan-k1`, and the three-chain decomposition. Cite these as
`plan-review-k3 R<n>`. No fixes applied.

Verdict: the confirmed contract is coherent and mostly falsifiable, but three
glossary entries were **not visited by the rewrite** and now assert capabilities
the new contract deletes (R1–R3); one traversal rule as written rejects every
node directory (R4); and the largest reconciliation surface in the repo is named
in no `Done when` (R5). R1–R5 are the blocking set.

### Blocking contradictions

**R1 — The harness stamp is in the removal list and still live in the glossary.**
`.grove/BRIEF.md` `Done when` bullet 4 removes "stamps". `CONTEXT.md:22`
(**Grove name**) still ends: the grove name "names … the harness stamp
(`<repo>/.grove-stamps/<name>`)". The stamp is real code and tests
(`src/harness_stamp.rs`, `tests/harness_stamp.rs`, `src/cli.rs`,
`docs/CONFIGURATION.md`, `README.md`, `.gitignore`).
*Why it matters*: **Grove name** is the entry that defines what the name is
*for*; leaving a removed artifact in its enumeration means a future session
reintroduces the stamp as an established fact.
*Action*: strike the stamp clause from `CONTEXT.md:22` and add the stamp files
to the removal surface `implementation-slices-k10` will plan.

**R2 — "Grove injects no hidden argument fragments" contradicts the `--settings`
turn-hook injection, which is unchanged in the same file.**
`CONTEXT.md:256-257` states the template supplies every harness-specific flag and
"Grove injects no hidden argument fragments"; `CONTEXT.md:243-250` states Grove
"neither knows nor infers which harness the command ultimately runs".
`CONTEXT.md:455-498` (**Session-boundary visibility**) is unrevised and states
that "on every **claude** launch the driver appends `--settings` with an inline
JSON hook block" (`CONTEXT.md:462`) — a harness-conditional, Grove-authored argv
fragment, i.e. exactly what the new rule forbids, decided by exactly the
knowledge the new rule denies.
*Why it matters*: this is the whole mid-turn `blocked`/`working` status surface
(ADR *herdr-turn-boundary-hooks*). The requirements neither preserve it nor
retire it; `${herdr_settings}` (R3) looks like the intended bridge but is never
connected to this entry.
*Action*: decide explicitly — retire the turn hooks, or define `${herdr_settings}`
as their transport — and rewrite **Session-boundary visibility** to match. This
changes a confirmed requirement, so per `plan-integrate-k4`'s own `Done when` it
is resolved **with the human**, not inferred.

**R3 — `${herdr_settings}` is introduced and never defined; and `HERDR_AGENT`
requires the harness knowledge the contract removes.**
`CONTEXT.md:255` lists `${herdr_settings}` in the substitution set. It appears
**nowhere else in the repository** — no definition of what it expands to, whether
it is one argument or several, or what it becomes outside a herdr pane. The old
design's property was that argv is *byte-identical* with no herdr present; a
substitution that expands to empty leaves a dangling `--settings` with an empty
operand, which is not that property.
Separately, `CONTEXT.md:500-537` (**Pane mis-detection**) is unrevised and states
Grove sets `HERDR_AGENT=<harness.name>` "on the harness child at **both** launch
sites (`launch::set_herdr_agent_hint`)", **unconditionally**, and records it as a
live fix verified 2026-07-28. That value *is* the harness identity
`CONTEXT.md:243-250` forbids Grove from knowing. Implemented in `src/launch.rs`
with coverage in `tests/launch.rs` and `tests/loop_driver.rs`.
*Why it matters*: without the hint, herdr re-acquires grove panes as `codex` — the
documented regression this repo already fixed. The requirements delete the
capability silently.
*Action*: define `${herdr_settings}` (expansion, arity, absent-pane behavior) or
drop it; and state the disposition of `HERDR_AGENT` — a `${herdr_agent}`-style
substitution, or accepted loss of the fix — updating **Pane mis-detection**
either way. Also human-facing per R2.

**R4 — The malformed-name rule, read literally, rejects every node directory.**
`CONTEXT.md:63-65`: "A live filename with the task position/key shape but a
missing or unknown session kind is malformed and **stops traversal** with the
path and valid kind set; it never degrades to `impl`."
A node directory is `NN-<slug>-k<key>/` (`CONTEXT.md:548`) and carries **no**
session kind by design. Every node in this very tree —
`.grove/01-plan-chain-k2/`, `.grove/02-config-driven-sessions-chain-k5/`,
`.grove/03-implementation-slices-chain-k9/` — has the position/key shape and no
kind, so a literal implementation halts on the first one.
*Why it matters*: it is a fail-closed rule over the walk every command depends
on, and the failure is total rather than partial.
*Action*: scope the rule to `.md` leaf files and state that node directories are
kind-free by construction. While there, resolve the adjacent ambiguity: a node
slug that *begins* with a kind token (`NN-design-tokens-k5/`) must not be
kind-parsed, and the "longest member of the closed set" matcher must be stated as
leaf-only.

**R5 — `content/` — the provisioned methodology — is named in no `Done when`.**
`.grove/BRIEF.md` `Done when` bullet 5 covers "user documentation and the minimum
coherent ADR/spec set". `content/` is neither: `CONTEXT-MAP.md:9-11` makes it the
methodology, a distinct artifact class, and `docs/specs/doubt-grove-review-mechanics.md:519-523`
already lists `content/SKILL.md`, `content/driving.md` and `content/TASK-FORMAT.md`
as canonical surfaces for a *smaller* change than this one.
It is the largest inconsistent surface under the new contract: session-side pick
(`content/SKILL.md:116`, `:86`, `:97`, `:165`, `:425`, `:454`;
`content/driving.md:411`; `content/TASK-FORMAT.md:101`;
`content/prompts/start.md:3`), `**Kind:**` as a task-file field
(`content/TASK-FORMAT.md`), the seventeen-kind taxonomy, `--harness` on the grow
verbs, the finish cycle as an in-session step rather than a `finish` leaf, and
`grove do` spelling throughout — plus `content/prompts/` itself, whose per-verb
prompt selection `CONTEXT.md:8` has already replaced with one embedded prompt.
`plugins/linkuistics/skills/doubt-driven-development/SKILL.md` is in the same
position via R7.
*Why it matters*: `content/` is what every future grove session reads. Left
unreconciled it will actively instruct sessions to do the removed thing, and
`implementation-slices-k10`'s "docs" bullet does not obviously reach it.
*Action*: add `content/` (and the doubt skill) explicitly to `.grove/BRIEF.md`
`Done when` bullet 5, so `implementation-slices-k10` must plan it.

### Missing behavior

**R6 — The session's mandate has no stated transport, yet "does not pick again"
rests entirely on it.**
`CONTEXT.md:283-291` says the driver "passes the selected leaf to the launched
session as its mandate". The substitution set (`CONTEXT.md:253-256`) contains no
leaf, path, or handle substitution. The only candidate is `${prompt}`, which would
make `${prompt}` a Grove-authored value carrying tree state rather than the
user's prompt.
*Action*: state what `${prompt}` contains and that it carries the mandate — or add
an explicit substitution. Also state what a session started *outside* the driver
(the skill's supported hand-driven case) does, since it has no mandate and is
forbidden to pick.

**R7 — The doubt-composition ownership predicate changed, silently, in a
glossary edit.**
`CONTEXT.md:129-132` now reads "the Grove driver launched the current session for
a selected leaf and the session ran Bootstrap for that mandate … The driver's
selected-leaf mandate is the discriminator." The prior predicate — the session
invoking `pick` itself — is the binding text of
`docs/adr/grove-owns-escalated-review.md:3-5` and `:12-14`, and of
`docs/specs/doubt-grove-review-mechanics.md:33-45`, which additionally states
"Grove ownership is activated by a procedural fact, **not an environment probe**"
and that launch context "is inherited metadata and cannot decide which
methodology a session is following". The new predicate *is* inherited launch
context.
*Why it matters*: that ADR's rejection of "disable doubt whenever a checkout
contains `.grove/`" turned on precisely this distinction. Reversing it needs an
argument, not a redefinition; no `Done when` names the reversal.
*Action*: record it as an explicit requirement with its rationale, and list both
records plus the doubt skill as reconciliation targets.

**R8 — `finish` is classified HITL against the entry's own generating rule.**
`CONTEXT.md:186-190` gives the rule: HITL when "**a human's own words are the
session's input or its deliverable**", and adds `finish` to the HITL set. A
finish session's human input is a *confirmation*, not words that become the
deliverable — the same shape as `integrate-review-requirements`, which
`CONTEXT.md:195-197` reasons into **AFK** and calls "the borderline cell".
*Action*: either justify `finish` as a stated exception, or mark it AFK whose
single confirmation is the loop's one routine gate (`CONTEXT.md:209-229`), which
the **Confirmation boundary** entry already says is a *gate*, not a kind
property. Also state whether `leaf-add --kind finish` is refused (driver-only).

**R9 — Closed set with no defaults makes every future kind a breaking change for
every user, with no stated compatibility policy.**
`CONTEXT.md:260-264`: all nineteen kinds must appear exactly once; "a missing …
kind is an error"; "no defaults, families, profiles, or inheritance". A twentieth
kind therefore hard-fails every existing `config.kdl` until hand-edited. The
nineteen-kind strictness is human-confirmed (`plan-k1` Notes) and is **not** the
finding; the absent policy is.
*Action*: state the intended policy — kind additions are breaking and versioned,
or a new kind is tolerated-with-error-on-use.

### Migration and recovery gaps

**R10 — Migration's input domain is underspecified in four ways.**
`CONTEXT.md:71-79` covers leaves storing `**Kind:**`/`**Harness:**` and the
ambiguous standalone `research` case. Unstated:
(a) a legacy leaf with **no** `**Kind:**` line — deterministic (the old read-side
default was `impl`) or ambiguous?
(b) the legacy `work` alias, and `review-work` / `integrate-review-work` —
`docs/specs/doubt-grove-review-mechanics.md:111-113` and `:566-569` still accept
`work`;
(c) whether **terminal** (`DONE`/`ABANDONED`) leaves are renamed — if not, the
tree is left in mixed grammar, and `CONTEXT.md:604` makes retired leaves
load-bearing for key allocation;
(d) whether the **pre-v2 directory schemes** (`NNN-slug/`, the v1 flat
dotted-decimal tree) that `content/SKILL.md:88-94` says `grove` migrates today are
retained — a tree can need both conversions, ordered.
*Action*: enumerate the domain and the disposition of each case.

**R11 — Migration does not remove `**Producer launch:**` receipts.**
`CONTEXT.md:74-76` removes "both obsolete metadata lines" (`**Kind:**`,
`**Harness:**`). Receipts are removed by `.grove/BRIEF.md` `Done when` bullet 4
and `CONTEXT.md:296-302`, but live trees carry `**Producer launch:**` lines
(`docs/specs/doubt-grove-review-mechanics.md:328-337`) that migration leaves in
place, contradicting "receipts are removed".
*Action*: add the receipt line to migration's removal set.

**R12 — Migration claims atomicity and commits, with no transaction mechanism
and no dirty-tree rule.**
`CONTEXT.md:71-78` calls it "one-time, atomic" and says it "lands the conversion
in one focused commit before ordinary selection". Two gaps:
(a) *no mechanism*. This repo already holds the precedent that such a claim needs
a fail-closed witness plus the tree lock — `docs/adr/promotion-transactions-fail-closed.md`
and `docs/specs/doubt-grove-review-mechanics.md:158-235`. An interrupted
migration otherwise leaves a half-renamed tree that the R4 rule (once fixed)
still rejects.
(b) *no dirty-tree rule*. This is the only place Grove commits unprompted. Over a
dirty working tree the "focused commit" either sweeps unrelated changes in or must
refuse; neither is stated. Under jj there is no clean/dirty state at all — the
working copy *is* a commit — so git and jj need different, stated answers, which
`.grove/BRIEF.md` `Done when` bullet 5 ("Git and jj behavior … verified") assumes
exist.
*Action*: name the mechanism and the dirty-tree/jj behavior.

**R13 — Mutating steps may precede configuration validation.**
`CONTEXT.md:264-268`: the driver "re-reads and fully validates the file
immediately before every launch"; "Validation failure launches nothing and leaves
the selected leaf live and resumable". But `root-init` (`CONTEXT.md:35-45`),
migration (`:71-79`), and finish-leaf materialisation (`:14-17`) all run *before*
selection and launch. Only the missing-file case is stated non-mutating
(`CONTEXT.md:271-273`). A user with a malformed config can therefore have their
tree scaffolded, migrated and **committed** before the error.
*Action*: state the order — validate configuration before any tree mutation — or
state explicitly which mutations are allowed to precede it and why.

**R14 — The finish leaf's own lifecycle is unspecified, and a declined finish can
livelock the loop.**
`CONTEXT.md:14-18`: the driver "mechanically adds the finish leaf and runs the
ordinary pick/config/launch path again"; "A declined, interrupted, or crashed
finish remains a live leaf and is resumed normally." Unstated:
(a) which commit folds the added leaf in — the compare case, `root-init`, states
this explicitly (`CONTEXT.md:42`), and the finish leaf's only natural terminal
commit is the one **deleting `.grove/`**, so on the success path the leaf is
created and destroyed with no commit ever naming it;
(b) its position and key;
(c) what "declined" does to the loop. If the session declines and runs
`grove-llm complete` (relaunch), the driver re-picks the same finish leaf and
relaunches — an unbounded loop on a HITL leaf. If declining must exit without
signalling, that needs saying, and it is the one case where a session
deliberately *withholds* the mandatory last action.
*Action*: specify (a)–(c), and whether re-materialisation is idempotent when a
finish leaf already exists.

### Scope and accidental solution detail

**R15 — The requirements already fix the design that `config-driven-sessions-k6`
is chartered to produce.**
`config-driven-sessions-k6` `Done when` bullet 1 is to record "the KDL shape,
substitutions, validation/errors, filename grammar, authoritative pick,
automatic requirements/migration/finish transitions, and removed public
surfaces". `CONTEXT.md:243-281` already fixes: KDL as the format, the exact
five-member substitution set, `${prompt}` required exactly once **and last**,
POSIX shell-word quoting with direct argv execution, "a wrapper should `exec`",
error behavior, and the "complete copyable nineteen-kind example". The root
brief's Notes carry the same.
*Why it matters*: not fatal — much of it is human-confirmed — but it either makes
the design leaf's first bullet a transcription exercise or leaves it unclear which
of these the design may revisit.
*Action*: mark which items are **confirmed and closed** to the design leaf and
which it may still decide. Flag one concretely:

**R16 — "`${prompt}` required exactly once as the final template element" is a
strong constraint that may be unsatisfiable.**
`CONTEXT.md:253-254`. Prompt-last forbids any trailing flag after the prompt
operand, which several real harness invocations use (`… -p "<prompt>"
--output-format …`), and forbids a wrapper that needs a terminating argument. It
is also the only positional constraint in an otherwise opaque template, so it
partially contradicts "the template chooses … every user-controlled argument".
*Action*: confirm the constraint against the actual harness command lines in
`docs/CONFIGURATION.md`, or relax to "exactly once, anywhere".

**R17 — Orphaned public surfaces are absent from the removal list.**
`.grove/BRIEF.md` `Done when` bullet 4 enumerates removals. Not named, though all
lose their reason to exist:
- `grove-llm kind --with-harness --json` — the structured routing peek, whose
  payload is the declared harness and the review receipt evidence
  (`docs/specs/doubt-grove-review-mechanics.md:277-298`, `:374-387`), both gone.
- `leaf-add-pair --harness-a/--harness-b` — superseded by the `research-a` /
  `research-b` kinds (`CONTEXT.md:68-69`, `:175-176`).
- `leaf-add`/`leaf-insert`/`leaf-decompose` `--harness` — the `**Harness:**` field
  is removed.
- `GROVE_SESSION_TARGET` and its scrub list / `tests/env_hygiene.rs` obligations.
- `docs/adr/promotion-transactions-fail-closed.md` is **not** in the brief's
  "Existing records in scope", yet `docs/specs/doubt-grove-review-mechanics.md`
  interleaves receipts with the promotion transaction and tree lock in one
  document — so that spec must be **split**, not deleted: its receipt/diversity
  sections die while its promotion, lock and test-seam sections survive.
*Action*: extend bullet 4 and the Pointers list.

**R18 — `CONTEXT.md`'s claim that the tree viewer has one contract is now false.**
`CONTEXT.md:363-366`: "Its **only** contract is the [[Node directory]] naming
scheme … the live leaf's session kind comes from its filename and no task body is
opened." Reading the kind from the filename means the plugin must now carry the
**closed nineteen-kind set** to parse `NN-<kind>-<slug>-k<key>.md` — a second
contract, and one that changes whenever the set does (R9). Previously it read one
`**Kind:**` line and needed no set.
*Action*: correct the "only contract" claim and record kind-set changes as a
plugin-compatibility obligation, as `CONTEXT.md:369-371` already does for the
directory scheme.

### Checked and sound

Recorded so integration does not re-litigate them: the nineteen-kind count is
arithmetically right (5 producers × 3 = 15, + `research-a`/`research-b`/
`combine-research` + `finish` = 19); the `research-a`/`research-b` split
genuinely removes the one shape per-kind routing could not express; "the fact
wins" is correctly retired now that only the driver picks; the three-chain
decomposition covers the goal with implementation properly deferred; and
`Reviews`/`Integrates` survive untouched, which R11 relies on.

## Notes

Review-target diversity notice for this session was `uncheckable`
(producer receipt missing, producer target unavailable; review target
claude/"opus"). Advisory only — it did not gate this review, and the contract
under review removes the mechanism that emits it.
