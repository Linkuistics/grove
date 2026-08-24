# experiment-baseline-k30

**Integrates:** experiment-baseline-k29

## Goal

Correct the Experiment 2 pre-registration and preservation baseline before
`design-model-contract-k5` starts, so the formal comparison is countable by a
fresh session and later refactors do not rely on false or incomplete coverage
claims.

## Context

`experiment-baseline-k29` reviewed producer commit `b10f88955c4f` against the
current artifacts and source. The review was inspection-only: it ran no test,
build, lint, or format commands. The codebase-memory index for this jj secondary
workspace could not be created because daemon coordination could not be verified,
so every source claim below was checked directly in the named file and broad
negative claims were swept with `rg`.

## Findings to integrate

### 1. The material-finding boundary does not sort the registered borderlines

`docs/formalism-findings.md:3002` uses “a reader would act on” while also saying
a too-small scope is a tooling artifact. Two sessions can therefore classify the
same model-driven edit differently. Replace that paragraph with an operational
rule based on the source and durable consequence of the defect:

- a finding is material only when it corrects the tool-neutral catalogue,
  requirements/design, shipped behaviour, durable documentation, or a Rust test,
  and the correction survives outside model syntax, runner settings, or bounds;
- a model-only syntax/transcription error or bounds/trace-limit change is M7
  tooling cost, unless it also invalidates a recorded tool-neutral claim, in which
  case that invalidated claim is the material finding and the bounds change is
  its evidence;
- record the affected tool-neutral claim and the durable correction at discovery
  time, before M1/M3 scoring. A classification with neither is not countable as a
  material finding.

State the three mandated borderline results explicitly: a merely too-small bound
is M7 (and, if it previously produced a believed green, M8); an ambiguous model
predicate is M7 unless the ambiguity came from and forces a correction to the
tool-neutral catalogue/design; shipped behaviour that is correct but omitted
from a catalogue required to describe it is material because the catalogue must
change. This last case may legitimately record `M4 = none` and falsify H7.

### 2. Six hypotheses are not falsifiable by the measures as written

Correct the `Hypotheses` and `Measures` sections together:

- **H4** (`docs/formalism-findings.md:2954`): its falsifier is not the complement
  of “each tool has a unique finding in at least two scopes.” Define the scope
  predicate exactly and falsify when fewer than two scopes satisfy it.
- **H5** (`:2959`): it predicts two directional misses, but “neither” lets one
  failed prediction pass. Falsify if either predicted unique class is absent or
  lands in the opposite formalism.
- **H6/M3** (`:2967`, `:3013`): “spread” and “between” have no statistic, and one
  score per finding cannot separate tool from defect. Define the comparison
  before results (including ties and insufficient paired findings) and record M3
  per `(finding, formalism)` for independently overlapping findings; otherwise
  mark H6 inconclusive rather than choosing a post-hoc statistic.
- **H8/M5** (`:2977`, `:3015`): the hypothesis says cheaper *per claim* while M5
  compares findings per hour. Record authoring hours and checked-claim count and
  compare hours per checked claim; retain findings/hour only as a separate
  descriptive value.
- **H9/M6/M7** (`:2983`, `:3016`): replace “materially larger” and “small” with a
  pre-registered numerical threshold and define total modelling effort's
  denominator. State how synchronization and tool-wrangling shares are compared.
- **H10/M8** (`:2988`, `:3018`): M8 times false-confidence incidents but no
  measure times genuine failing checks. Add the latter duration and define the
  summed-hour comparison, or narrow H10 to the data M8 actually records.

The pre-registration is not fit to bind Experiment 2 until these corrections
land; these are measure changes, so they must precede `design-model-contract-k5`.

### 3. The witness contract omits `PREPARING-FINISH-`

`docs/preservation-baseline.md:185` says the reserved classes are only
`MIGRATING-session-kinds/` and `FINISHING-<handle>/`. The ADR and source also
reserve `PREPARING-FINISH-<handle>-<attempt>/`, and
`tests/finish_lifecycle.rs:1461` asserts that an ordinary reader refuses it.
Correct §5, add its captured before-state to §8, and include it in §10's
reserved-witness row.

While correcting that §10 row, remove `tests/migration_transition.rs` and
`tests/migration_commit.rs` as evidence for *refusal*: the former checks legacy
transition/unknown-format refusal and the latter checks migration commit and
recovery mechanics. The refusal evidence is
`tests/session_kind_tree.rs:495` for `MIGRATING-` and
`tests/finish_lifecycle.rs:1293`, `:1424`, and `:1461` for the two finish witness
states.

### 4. §10's environment-coverage row names the wrong evidence

`docs/preservation-baseline.md:946` cites `tests/env_hygiene.rs` and
`tests/repo_environment.rs` for spawned-child hygiene. The first protects the
test runner's live signal channel; the second proves repository discovery ignores
ambient Git/TMP selectors. Neither asserts the full spawned-child scrub set.
Correct the row to cite the `src/launch.rs:132` and `:171` unit tests for the
enumerated scrub lists, plus the black-box seams that actually exercise launch
inheritance (`tests/lifecycle_cutover.rs:190` for loop control and the relevant
repository/finish child tests). Keep the two existing files under narrower claims
that match what they assert.

Also correct §7's scope statement: `GROVE_TEST_FINISH_EXIT_AT` and
`GROVE_TEST_FINISH_FAIL_AT` are live shipped failure controls in
`src/finish_transaction.rs:84` but are absent from `src/launch.rs`'s scrub list.
Record that measured gap explicitly; do not describe the six listed variables as
the complete set of actionable internal test seams. Product-code remediation is
outside this formal-baseline integration.

### 5. The “five nothing rows” table contradicts itself

`docs/preservation-baseline.md:953` says five rows read “nothing,” but formula
rendering is embedded in the `partially` row at `:951`. Split package/binary/install
coverage from Homebrew formula rendering so formula rendering is its own
`nothing` row, or change the prose to say five weak points. Preserve the useful
fact that `tests/methodology.rs:1080` pins eleven instructed verb names while the
full thirteen-verb set, options, and arity remain unpinned; do not weaken the
full-surface gap into “no verb names are checked.”

### 6. §11 mistakes reproducible gaps for unreachable ones and is incomplete

Reconcile the omission list with the evidence already in the repository:

- configuration diagnostics are reproducible in an isolated fixture using the
  stub-session rig at `tests/lifecycle_cutover.rs:190`; live-driver contention is
  not a reason to omit them;
- `finish-commit` success and refusal are reproducible in isolated Git, native-jj,
  and colocated-jj fixtures in `tests/finish_lifecycle.rs`; capture a bounded
  representative before-state rather than calling the whole verb unreachable;
- Linux execution is genuinely unavailable on this macOS host and may remain an
  explicit limitation; performance is outside the preservation contract and
  should be labelled an intentional non-goal, not an unreachable measurement;
- add the missing release-cut/formula/install limitation: §10 itself says formula
  rendering is checked only by a real cut, but §11 never lists it;
- add or capture the bare-driver/workspace-layout boundary. The admitted layout
  matrix and cross-device refusal are asserted at
  `tests/workspace_layout.rs:307` and `:369`, but §8 has no corresponding before
  transcript even though `extract-workspace-k25` will move this seam;
- add a representative malformed-current-tree refusal from
  `tests/session_kind_tree.rs:137`/`:152`, which `extract-task-tree-k24` will move
  and §8 currently omits.

After these additions, §11 must distinguish three things explicitly: measured
transcripts, source/test-backed read facts, and genuinely unmeasured behaviour.

## Done when

- The material-finding rule deterministically sorts all three mandated
  borderlines and carries the evidence needed for another session to reproduce
  the classification.
- H4, H5, H6, H8, H9, and H10 have falsifiers that are exact complements of
  their predictions and measures that collect every operand they compare.
- §5/§8/§10 include `PREPARING-FINISH-`; every §10 citation states what its named
  source actually asserts; the environment gap and the formula row are accurate.
- §11 is complete and labels each omission as fixture-reproducible,
  host-unreachable, or intentionally out of contract. The material
  fixture-reproducible baselines above are captured before modelling begins.
- No model, product code, architecture document, or user guide is changed.
- The corrected pre-registration says plainly that it is fit to bind Experiment
  2 before `design-model-contract-k5` becomes selectable.

## Notes

The ledger remains a contract; that framing is not reopened. The fixes above
change its accuracy and completeness, not its role.
