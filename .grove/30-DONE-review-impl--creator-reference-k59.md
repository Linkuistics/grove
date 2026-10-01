# creator-reference-k59

**Reviews:** creator-reference-k38

## Goal

An adversarial, inspection-only read of the whole `creator-reference-k38` node:
the methodology amendment that makes a finishing session name its run on a
producer's reviews (`creator-methodology-k39`), and the Grove lifecycle cases
and owner documentation built on it (`creator-lifecycle-k40`). Produce
findings, not fixes.

## Context

The amendment changes rules every future session follows. Conformance rows
prove that wording is present on a loaded path. They do not prove that the
steps are coherent where a session meets them, at retirement and at node
close. The lifecycle cases prove what a review's launch selects from once a
fake session has followed the step. They do not prove a real session can
follow it from the text alone.

Read against the spec's `#identity-and-creator`, `#grove-integration` and the
Grove launch-boundary row of `#test-seams`, and against
`docs/adr/a-review-carries-its-creator-reference.md`. Each leaf's running log
records what it chose and the mutations it saw fail. A mutation shows only that
a test notices the break it was written for.

No session in this grove has yet run under the amended skills. Claude Code
receives them through the marketplace after the release push, and Codex
through the binary. So read the files. No session's behavior is evidence.

## What to doubt

- **Whether the step is coherent in the order a session meets it.**
  `references/retire.md` puts *Naming your run on what you finish* between
  retirement and the commit. A producer cuts its review as its last act. So
  which comes first, the cut or the naming, and does the text say? A fake
  producer that named its run before it wrote the cut review's `**Reviews:**`
  line left that review with no creator, and the review refused (k40's log).
  Walk a real session through the text for three shapes: a leaf that cuts its
  own review; a node close, where step 4 carries the step and the node's
  review is cut by the closing leaf; and a multi-level close.
- **Whether "search `.grove/` for the handle" finds what it must, and only
  that.** A review cut earlier can sit anywhere. A terminal review must not be
  rewritten. A handle quoted in prose or in a fenced example is not a
  `**Reviews:**` line. The fake sessions match the whole line, and skip
  filenames with a `DONE` or `ABANDONED` infix. Does the text's "every live
  review leaf whose `**Reviews:**` line already names its handle" lead a
  session to that same set?
- **Whether the fake sessions follow the convention or the test's
  convenience.** `SESSION` in `crates/grove/tests/loop_driver.rs` is the
  procedure. Compare it clause by clause with the retirement procedure. The
  dispatched attempt in the direct-finish case names its run and dies before
  it retires. Is that an attempt the methodology can produce, or one invented
  so the removal has something to remove?
- **Whether the lifecycle cases would notice a wrong creator.** Each asserts
  the reviewer launched and the creator the run recorded. Look for a wrong
  outcome every case still passes: a review that selects from the right
  provider for the wrong reason, a second live review the fake never reaches,
  a review nested inside a node, an abandoned producer.
- **A hand-written run line is kept as a second remedy.** The spec gives the
  declaration for a producer finished with no run. It is silent on a
  dispatched session that omitted its step. The documents and the adapter's
  `creator_line_missing` remedy lead with the declaration, and keep
  `**Creator:** run <run-id>`, written by the owner, for that case. Is that
  consistent with the retirement procedure's "from your own environment and
  from nowhere else", and with what *execution-recorded* is allowed to mean?
- **Four documents carry an account of the line.**
  `docs/CONFIGURATION.md#a-reviews-creator-line` is the whole one. The usage
  guide's review-composition section, configure-grove's *The creator line* and
  the dispatch README's Grove review section are shorter forms. Do they agree
  with one another, with the spec, and with the retirement procedure? Does any
  restate the step closely enough to drift from its one owner while staying
  under the conformance row's pinned phrase?
- **The scrub in `driver_command`.** It removes `HARNESS_DISPATCH_RUN_ID` and
  `HARNESS_DISPATCH_STATE_DIR` from every driver this suite launches. Does any
  other fixture that starts a session, in this crate or another, still inherit
  an ambient run? Does a real direct harness launched inside a dispatched
  session inherit one, and does the methodology's text guard a session against
  writing it?
- **The rescoped statements.** k39 scoped every statement that nothing reads
  the relationship lines to Grove's own code. Its sweep's classes are in its
  log. Look for one it classified wrongly, and for one written since.

## Done when

- Every doubt above has been read against the methodology text, the code and
  the tests, and each finding names its file, its line and a failure scenario,
  or the doubt is recorded as examined and found sound.
- A review with findings worth acting on cuts its `integrate-review-impl` leaf
  where `pick` reaches it next.

## Decisions (running log)

**The review stays inspection-only.** Its producer boundary is commits
`509456b30e34` (`creator-methodology-k39`) and `9d55039c8557`
(`creator-lifecycle-k40`). Source, committed diffs, the specification, the ADR
and the producers' recorded mutation evidence are the inputs. No test, build,
lint or format command is run, and no implementation or test file is edited.

**Review creation must precede creator settlement, but the procedure does not
state that dependency.** The retirement procedure runs naming immediately
after leaf retirement, and again inside node-close step 4; decomposition calls
cutting the review the producer's last act. A session taking those steps in
that order can cut a review after the naming pass, leaving it without a line.
The ordinary-chain fake instead creates and fills the review before `finish`.
This is finding F1 below.

**The lifecycle fixture does not establish the whole review set.** Every case
has just one live review, in the root, for `parser-k1`; the multi-level case
has no review of its inner producer. The implementation iterates recursively,
but its tests could not notice first-match-only settlement, a root-only search
or naming just the outermost closed node. This is finding F2 below.

**The configure-grove reference repeats the operational step despite naming
its owner.** Its run-form bullet states the replacement/removal branches, and
its later paragraph repeats the forbidden ID sources. Those are procedural
content, rather than an account of the field's meaning. The single-owner ADRs
forbid a second statement even when it agrees, whereas the row's exact lead
phrase does not detect this paraphrase. This is finding F3 below.

**The direct-finish test seeds a noncompliant attempt.** Its first dispatched
session calls `name_run` without retiring, although the procedure explicitly
says a leaf left live writes nothing. The stale-line cleanup remains a useful
case, but its claim to model a convention-following attempt is false. This is
finding F4 below; it concerns the case's provenance, not the correctness of
the removal or declaration assertions.

**Integration is placed before dependent work.**
`creator-reference-k60` is the `integrate-review-impl` leaf inserted at the
first live root sibling after this review, ahead of
`package-entry-resolution-k52`. Its charter names this review rather than
transcribing the findings, so independent triage can accept or reject each.

## Findings

### F1 — P2: State the review-creation dependency before creator settlement

**Location:** `plugins/grove/skills/grove/references/retire.md:24`, and the
node-close invocation at `plugins/grove/skills/grove/references/retire.md:120`.

The naming section follows retirement and calls the manual write the next
step (`retire.md:17`). Node-close step 4 also invokes naming during the close.
Neither says to create and finish the new review's `**Reviews:**` body before
that pass, or to settle a newly cut review if it is cut afterward.
`references/decompose.md:138` calls the cut the producer's last act. A session
that retires, names the reviews already present, completes its close cascade,
then cuts the required review can therefore commit a new review with no
creator. Its dispatched launch refuses with `creator_line_missing`, although
its producer finished under dispatch. For nested closes, a review cut for an
inner node after that node's naming pass has the same failure.

The ordinary-chain test supplies the missing order itself: it creates and
fills the review at `crates/grove/tests/loop_driver.rs:2938` before calling
`finish`. The producer log records that reversing body creation and naming
made that case refuse. The conformance pins establish the naming clause's
presence, not this dependency. The canonical procedure needs an unambiguous
sequence that covers a newly cut leaf review and newly cut reviews of every
closed node, while keeping creator settlement in the producer's commit.

### F2 — P2: The lifecycle cases cannot detect settling only one review

**Location:** `crates/grove/tests/loop_driver.rs:2522`, with the multi-level
case at `crates/grove/tests/loop_driver.rs:2748`.

The fixture plants one review of `parser-k1` at the grove root. The teardown
case replaces it with one newly cut review. The multi-level case still has
only that outer review: there is none for `grammar-k4` or `tokens-k5`. There
is no second live review, nested review, or terminal review whose creator
must remain untouched. Consequently, a `name_run` that takes just its first
match or searches only the root, a `finish` that names only the outermost
closed node, or a writer that also changes terminal reviews can satisfy all
four cases. This is a source-level counterexample to the coverage claim, not
a mutation executed in this review.

The procedure's obligation is every live review anywhere in the tree, for
every producer finished by the cascade (`retire.md:24`). Leaving another
live review on an earlier run can select its reviewer against the wrong
provider; rewriting a terminal review changes its frozen provenance. The
fixture's recursive loop implements those distinctions, but its current
subjects and assertions do not observe them. Lifecycle evidence needs to
distinguish the complete intended set from these wrong sets, including both
closed-node handles.

### F3 — P2: The configure reference is a second procedural statement

**Location:**
`plugins/grove/skills/configure-grove/references/dispatch.md:166` and `:179`.

After citing `references/retire.md` as the owner, the run-form bullet states
the step again: use the session's own environment, replace the line, remove
either form with no run, and exclude earlier attempts. The later paragraph
adds the rule's forbidden copying sources again. These are the operational
branches of `finishing-session-names-its-run`, not just the forms and writers
an owner needs to understand the field.

`docs/adr/corpus-rules-have-one-owner.md:28` prohibits another statement, and
`docs/adr/restatement-declares-its-class.md:29` makes a second procedural
statement a defect even while it agrees. A future repair to creator
settlement can leave this copy authorizing the old procedure. The row at
`plugins/grove/conformance/rules.tsv:226` pins only the lead phrase; the
current copy does not contain it. In addition, `conformance.sh:494` indexes
the spine and `grove-*` kind skills, rather than `configure-grove`, so that
reference is outside this ownership sweep. The recorded clean conformance
result does not establish the claimed absence of another procedural owner.
Keep the field explanation and operator remedies, with the finishing
procedure reached through its canonical owner.

### F4 — P3: Identify the stale attempt as an injected convention violation

**Location:** `crates/grove/tests/loop_driver.rs:2846`, and the compliance
claim at `crates/grove/tests/loop_driver.rs:2414`.

The first attempt calls `name_run "$handle"`, exits without retirement and
leaves the producer live. `retire.md:41` explicitly says a leaf left live
finishes nothing and writes nothing, and the ordinary `finish` helper retires
before naming (`loop_driver.rs:2451`). This attempt is therefore not one the
documented procedure produces. The test detects cleanup of a planted stale
line and the subsequent declaration path; it does not show a compliant
dispatched attempt producing that line before a compliant direct finish.
Describing every fake as following the convention can make this passing case
look like evidence for an ordering the methodology forbids. Distinguish the
fault-injection premise from the compliant finishing procedure in the case's
contract and recorded evidence. The stale-line removal remains worth testing.

## Examination of the remaining doubts

- **Which files are affected:** the procedure says live review leaves with
  the producer's actual `**Reviews:**` line, rather than every textual handle
  match. The fake's whole-line grep and terminal-name filter implement that
  distinction for its subjects. A mention in ordinary prose or an indented
  example is not a marker. An unindented standalone marker inside a fence
  intentionally counts under the adapter's documented grammar
  (`docs/specs/harness-selection-and-execution.md:664` and
  `worker/grove/index.ts:42`); it is not a markdown-aware exception. F2 records
  the unexercised set boundaries, rather than alleging an observed wrong write.
- **Node-close semantics:** the real procedure checks `Done when` before
  naming, and adds missing work or escalates instead of closing when it fails.
  The fake only walks structurally terminal ancestors, in empty synthetic
  briefs. That abstraction is adequate for its supplied complete work; it
  does not prove closure after pruning. A pruned leaf does not invoke naming,
  and the procedure explicitly leaves its review deliberately uncheckable
  (`retire.md:61`). No abandoned-producer admission defect was established.
- **Wrong creator in the cases they do exercise:** retirement/reordering
  records the old provider and full candidate, and launches intervening work
  under the remapped catalog as a control. The node case asserts the exact
  closer's run reference and its child task identity, as well as inspection.
  Those assertions distinguish the selected provider from the right provider
  reached using a different recorded creator. F2 concerns other reviews and
  closed producers that the cases do not contain.
- **Owner repair of an omitted run line:** the adapter and the configuration
  reference expressly allow the owner to restore the actual finishing run,
  excluding earlier attempts. The producer-only environment rule governs the
  producing session's automatic step; this is a separate owner repair. The
  specification's execution-recorded class describes the provider snapshot,
  not proof of the artifact association (`spec:601`), so this repair does not
  falsely promote an owner-transcribed provider to recorded evidence. The
  spec's normal writer description does not describe this exception, but the
  operational documents make it explicit. No enforcement defect follows
  from the exception under the accepted attestation boundary.
- **Agreement of the four operator accounts:** configuration, usage,
  configure-grove and the dispatch README agree on the run and declaration
  forms, finishing-session identity, removal with no run, the same-store
  requirement and the observation association. The longer remedies and
  adapter retain owner recovery of an omitted line. F3 concerns duplicated
  procedure, rather than contradictory provider or evidence semantics.
- **Ambient run identity:** `loop_driver.rs:157` scrubs both dispatch variables
  on the driver-command path, including the PTY driver's use of that helper.
  Other launch fixtures do still inherit them: `skill_provisioning.rs:40`,
  `config_examples.rs:95`, `lifecycle_cutover.rs:123` and
  `grove-loop/tests/driver_lease.rs:130`; the shared
  `testing/support.rs:350` scrub list has no dispatch entries. Their inspected
  fake sessions do not perform creator settlement or assert dispatch identity,
  so no new concrete assertion failure was identified. This is a local
  fixture scrub, not a global environment guarantee. A real direct harness
  nested under dispatch can inherit another session's run. The spec at `:605`
  and the creator-reference ADR explicitly accept that attestation risk;
  “own environment” does not authenticate the value. No new guard is claimed.
- **Rescoped relationship statements and delivery:** the changed glossary,
  task format, architecture, usage and review-mechanics spec scope their
  no-parser claims to Grove's own code. The adapter reads `Reviews` and
  `Creator`, not `Integrates`; the architecture states that exact pair. The
  inspected walkthrough hits concern the Grove source they reconstruct,
  rather than claiming dispatch has no reader. The provisioning test compares
  installed file bytes to the plugin's and pins the amended clauses. The
  conformance controls remove the clause, remove its node-close occurrence and
  repeat its exact lead phrase. These provide wording/delivery evidence, not
  session compliance or the ordering required by F1.

## Evidence and limits

Tier 2 graph verification began with the nearest indexed project
`Users-antony-Development-grove.new-tool-for-harness-selection-and-execution`,
generation `2026-09-29T11:18:53Z`. Relevant search and trace pages were complete.
Coverage was checked for every relied-on path. Most changed files were
`metadata_changed` or `not_tracked`; `docs/`, `CHANGELOG.md` and the conformance
test script were excluded. The graph's snippet for the old `run_driver` line
range did not identify the current function. Findings therefore rely on the
current source and committed diffs, read directly, rather than graph absence
or stale line ranges. The graph provides no completeness proof.

The producers' logs report the pins failing against unamended wording and the
lifecycle mutations failing before restoration. They are recorded producer
evidence, not fresh results of this review. The review ran no tests, builds,
linters or formatters, as its family procedure requires. Integration owns all
fixes and their verification.
