# methodology-changes-k91

## Goal

Land the lessons that **bind** as changes to Grove's methodology payload, and
leave the ones that cannot bind to the write-up. The root decomposition's step 3.

Three bindings are named and their sites are verified; the leaf's job is to write
them, keep the embedded payload / rebuild / corpus-test triangle consistent, and
decide honestly whether a fourth is owed.

## Context

**The lessons are adjudicated and you are not re-opening them.** Read
[`docs/candidate-lessons.md`](../docs/candidate-lessons.md) — six verdicts with
citations and falsifiers — and its §*What binds, and what only gets written down*,
which is the list below with the reasoning. [`docs/loop-record.md`](../docs/loop-record.md)
and [`docs/review-yield.md`](../docs/review-yield.md) are the measurements behind
it. **`.grove/` is still perishable and still must not be finished** until the
whole lessons workstream lands; nothing here needs it, because all three
documents are outside it.

**The three bindings, with their sites verified at `content/` rather than at the
provisioned skill copy** — though those are currently byte-identical, which is
itself worth re-checking before you edit, because you will be editing the source
of a payload that is embedded at build time.

### B · The provenance discipline → `content/references/execute.md`

`## Verifying a claim about the repo itself` (line 48 at the time of writing)
carries the two-controls rule and the enumerate-then-classify rule, and carries
**nothing about the run itself**. Grep confirmed: no `freeze`, no `digest`, no
`one writer` anywhere in `content/`. What the campaign paid for, four sessions
over:

- **Finish all edits, then measure.** `obligation-placement-k63` invalidated
  three scope runs and about two hours by editing `models/run.sh` while all three
  were executing it. *A suite is an instrument, and an instrument you adjust
  mid-reading has not read anything.*
- **Digest every subject before and after, and say which.** `honest-classification-k80`
  and `finish-scope-k76` both froze the full subject set — a runner reads its
  manifest out of a spec and its declared gaps out of a README, so those are
  subjects and not bystanders.
- **One measurement, one writer**, and never infer a background job is finished
  from the return of the thing that launched it. `obligation-placement-k68` had
  two sweeps writing one log; the log showed `KILLED FN_31d` and then did not.
- **A re-run is confirmed item by item, not by matching totals** — `k80`'s
  command-by-command diff, "since two runs can agree on a total while disagreeing
  on which command did what."

### C · The review chain's structural defect → `content/references/decompose.md` and `integrate-review.md`

**This is the most actionable thing the harvest produced and the text currently
invites the defect.** `decompose.md:133` reads *"…it can carry the specific case
its producer could not cover, **or the findings verbatim**"*. `review-yield.md`
measured what that costs: in **five of nine** chains the integrating session's own
task body was written by the reviewer, so the charter *is* the finding list and
`Done when` is the finding list restated as obligations. An integration in that
position has no structural place to disagree — **no tree-level review finding has
ever been rejected in this grove**, 45 of 45, while the in-session channel (where
the reader owes the finding nothing) rejected 4 of 24.

The repair `review-yield.md` states: an `integrate-review-*` body should carry the
review's **handle**, and its findings should be read from the review's own
commit — the handoff a `review-*` step already uses — so that rejecting a finding
is not rejecting one's own charter.

**Two cautions, both from the same document.** The repair moves the hazard rather
than removing it: the finding list is still the reviewer's, so an integration
still cannot see a finding nobody wrote down. Say that where you land the rule
rather than letting the rule read as a fix. And the *other* half of
`decompose.md`'s sentence is right and must survive — a `review-*` leaf's body
carrying the producer's specific doubt is what `honest-classification-k84` shows
working, and cutting late is the whole payoff.

### A′ · One clause of cluster A that binds → `content/references/execute.md`

**A control that has never been seen to fail is not a control.** It is already
this repository's operating rule — `obligation-placement-k68` ran
`contested-property-only` against the *pre-fix* runner and required it to FAIL
before trusting its pass; `review-yield.py` carries three deliberate mutations
for the same reason, and the first found a real hole. It appears **nowhere** in
`content/`, while `execute.md` already asks sessions to control a claim in both
directions. The rest of cluster A — a control shown to kill for the reason
claimed, a module's environment as part of its control, every module run against
every claim — is about model suites, which Grove does not run, and goes to the
write-up.

## Done when

- Each of B, C and A′ is landed in `content/`, in that corpus's own register:
  the seven-constraint spine holds, `SKILL.md` stays a register of conditions
  rather than gaining prose, and a rule that belongs to one kind lands in that
  kind's reference file rather than in the shared ones. **A binding you decide
  against is recorded as declined with its reason**, not silently dropped — the
  write-up needs the *which is which* split to be honest.
- The payload triangle is consistent and checked, not assumed: `content/` edited,
  `build.rs`'s re-embed trigger still covering what changed, and
  `tests/lifecycle_invariants.rs` green — it asserts structural claims *about*
  the corpus (loaded-path membership, the routing table's shape, per-kind file
  reachability), so a new rule in the wrong file can turn it red for a reason
  that reads unrelated.
- **The preservation ledger is honoured or an exception is recorded explicitly.**
  Nothing here should touch CLI surface, configuration keys, the
  `session-kinds-v1` format, workspace-layout outcomes, packaging or MSRV — but
  the ledger is the root brief's and it binds whether or not you expect to reach
  it.
- Whatever verification you claim is verified: the repo-wide claims here are
  *this rule appears nowhere in `content/`* and *nothing else cites the sentence
  I changed*, and both are exactly the claim `execute.md`'s own section says a
  clean grep does not evidence. Enumerate and control in both directions.
- **The three harvest documents are reachable from a durable owner.** Today
  nothing outside `docs/` links `candidate-lessons.md`, and the root brief that
  does is in `.grove/`, which finish deletes — the exact reachability failure
  `handoff-audit-k66` was chartered to catch (*a link that only works from
  `.grove/` is a failure*). `loop-record.md` and `review-yield.md` are
  hand-edit-forbidden derived files, so a pointer into them belongs in
  `scripts/*.py`'s template rather than in the file. Fix it where a reader of the
  architecture would look for *what did the formal phase conclude*, or hand it to
  the write-up leaf explicitly — but do not leave it unowned.
- **The write-up leaf is cut as this session's last act, whatever you decide
  binds** — the root decomposition's step 4, at the grove root, and it is the one
  piece of the lessons workstream still unstarted after this. Its body must carry
  *which lessons bound and which did not, and why*, which is this leaf's output
  and is the reason it was not cut earlier. The root brief states its charter:
  readable by someone who has never seen this repository, and honest about cost —
  what the campaign spent, what it returned, and which parts were not worth the
  money. `loop-record.md`'s cost account is the raw form; it is not to be
  re-derived.

## Notes

**`docs/candidate-lessons.md` is a subject, not a bystander.** If landing a rule
makes one of its sentences wrong — a citation you move, a `content/` line it
quotes by number — repair it there rather than leaving the write-up to inherit a
stale quotation. The three documents are meant to stay mutually consistent after
teardown.

**Do not reopen a verdict to make a binding easier.** Where a candidate was
weakened, the weakened form is what binds; candidate 3's surviving conditional
form is not the headline the root brief wrote, and the write-up says so.

**Nothing in `.grove/` is edited beyond this leaf's own body**, and the retired
bodies stay the record.

## Decisions (running log)

- **The three repo-wide claims are verified with both controls, before any edit.**
  (a) *The provenance vocabulary appears nowhere in `content/`* — enumerated
  `freeze|frozen|digest|checksum|sha256|mid-run|in.flight|background job|concurrent|one writer|instrument`
  over `content/`; the only hits are `execute.md:60,62`'s *broken instrument*
  metaphor, which is the existing two-controls rule and not a provenance rule.
  (b) *A control that has never been seen to fail appears nowhere* — enumerated
  **every** occurrence of `control` in `content/` (seven; four are unrelated
  senses — VCS control directory, user-controlled template, version control,
  Grove control variables — and three are `execute.md:60,64,66`'s two-controls
  block), plus `kill|mutation|mutant|falsif|never seen|red` (no evidence-sense
  hit). Positive control: the same command finds `positive control` in
  `content/`. Cross-tree control: the same patterns still find the class in
  `docs/`, `scripts/`, `crates/*/models/`.
- **`content/` and the three provisioned skill copies are byte-identical** apart
  from `.grove-content-hash`, re-checked with `diff -rq` before editing, as the
  brief asked. `content/` is therefore the single edit site.
- **No new `SKILL.md` condition sentence.** B lands under trigger sentence 4
  (*When about to make a repo-wide claim…*), which is where
  `candidate-lessons.md` §*What binds* puts it. A 27th sentence would move the
  trigger table's 302-word total, the `SKILL.md` arithmetic budget and all 19
  kinds' `measured_static`, buying reachability that trigger 4 already gives.
- **Six inventory rows, not three.** The rule grain is *one row per rule a
  session could violate*; B is four such rules (frozen subject, one writer,
  background-job completion, item-by-item re-run) and C is two, because the
  session that **writes** an integration's body and the session that **reads**
  its findings are different violators in different kinds' files.
- **Three landed, in `content/`:** A′ as a fourth paragraph inside
  `execute.md`'s two-controls block; B as a new `## The provenance of a
  measurement` section in `execute.md`; C split across
  `decompose.md` (what a review **writes** into the body it cuts) and
  `integrate-review.md` (where an integration **reads** findings from). The
  split is single-source rather than duplication: two kinds, two violators, two
  registers. `decompose.md`'s *"or the findings verbatim"* is deleted and the
  producer's-doubt half of the same sentence is kept verbatim, as the brief
  required. Both cautions from `review-yield.md` are stated at the rule: the
  landed text says in as many words that it *moves* the hazard rather than
  removing it, because an integration still cannot see a finding nobody wrote.
- **A fourth binding is owed and DECLINED: candidate 5b, the predicate half.**
  *A check written by calling the definition it is about stops measuring the
  moment that definition is repaired.* `candidate-lessons.md` §5 splits 5a from
  5b precisely because 5b rests on **one** instance (`honest-classification-k80`)
  and "stating it in the same breath as five borrows their weight", and §*What
  binds* assigns B to 5a alone. Landing it beside the four provenance rules would
  re-join the sentence the adjudication split, which is reopening a verdict to
  make a binding easier — the one thing the brief's Notes forbid. It goes to the
  write-up. Recorded here so the split is not silent.
- **Also declined, and named because the brief asks for the which-is-which:** the
  rest of cluster A — a control shown to kill *for the reason claimed* via an
  isolating measurement, a module's environment counted as part of its control,
  every module run against every claim, and candidate 2's model-level form. All
  four are about model suites, which Grove does not run; the methodology has no
  session that could violate them, so an inventory row would have empty `Bound`.
  Write-up.
- **The budget ceilings are not re-fitted.** `loaded_path_budgets.rs` fits a
  ceiling once and lets it stand while the corpus moves — re-fitting on every
  edit is what turns a drift alarm into a rubber stamp. Measured after the edits
  by mutating every ceiling to 1 (digest either side, `shasum -c` verified):
  `integrate-review-*` static 1361 / 1425, the tightest row; every reachable path
  ≥ 490 words clear. Green with headroom, so the recorded measurements stay.
- **Reachability landed in `docs/ARCHITECTURE.md`'s *Documentation ownership*
  table**, which is where a reader looks for *what did the formal phase
  conclude*. Rows added for `candidate-lessons.md` (over `loop-record.md` and
  `review-yield.md`), `formalism-findings.md` and `preservation-baseline.md` —
  the last two were unowned there as well, found by the same enumeration.
  Pointers *to* the derived files, never into them, so `scripts/*.py` is
  untouched. The paragraph under the table said *"The four files directly under
  `docs/`…"* while nine sit there; it now states the structural fact and says
  explicitly not to replace it with a count — `execute.md`'s own
  `no-self-invalidating-count` rule applied to the summary layer the finding
  reached.
- **No ADR.** The three rules pass *hard to reverse* and *a real trade-off*, but
  the durable record already exists and is better: `candidate-lessons.md` carries
  the verdicts with their falsifiers, `review-yield.md` the measurement, and
  `docs/specs/corpus-rule-ownership.md` the six rows. An ADR would restate them
  and rot against three documents at once.
- **Verified, not assumed.** Full `cargo test` green (0 failures, every binary);
  `cargo fmt --check` and `cargo clippy --all-targets` clean. The payload triangle
  is checked at the binary rather than at the source: a `--release` build then
  `strings` over `grove-llm` finds all three new rules **present** and the deleted
  *"or the findings verbatim"* **absent** — the positive and negative halves of
  one control over `build.rs`'s re-embed. The provisioned skill copies and the
  installed `grove-llm` still carry the pre-change hash, which is
  `build-boundary-is-the-binary` working, not a fault, and is not this leaf's to
  resolve.
- **Preservation ledger honoured, no exception needed.** No CLI verb, argument,
  help shape, output field or exit status changed; no configuration key,
  environment override or default; `session-kinds-v1` untouched; no
  workspace-layout, packaging, MSRV or glibc change. `removed_surface`,
  `help_surfaces`, `session_config` and `workspace_layout` are green as part of
  the full run.
- **`06-impl-lessons-write-up-k92` cut at the grove root**, with the
  which-is-which split — three bound, four declined with reasons — written into
  its body, since that split is this leaf's output and the reason the write-up
  was not cut earlier.
