# Models

Every behavioural model in this repository, and the one runner that executes
them. The claims they check are **not** here: they are
[`docs/specs/semantic-contract.md`](../docs/specs/semantic-contract.md), the
tool-neutral catalogue both model families are written from, and this directory
holds instruments rather than statements.

## Layout

| path | scope | owns |
|---|---|---|
| `models/run.sh` | — | the single repository entry point |
| `models/system/` | lifecycle (`SY-`) | the joint of sessions, exhaustion, finish, interruption and recovery |
| `crates/grove-task-tree/models/` | task tree (`TT-`) | names, identity, selection, growth, retirement, terminality |
| `crates/grove-finish/models/` | finish / recovery (`FN-`) | the finish transaction and its recovery protocol |
| `docs/ordinal-fs-tree/models/` | the delegated boundary | ordered-tree algebra and filesystem mechanics — **consumed**, not restated |

Component models sit beside the component they constrain; the lifecycle models
sit at `models/system/` because they constrain the joint of several and belong
to none. A model file in none of these directories is a runner defect, and
`run.sh` says so.

## Running them

```sh
models/run.sh                                   # the whole repository, coverage asserted
models/run.sh --scope task-tree --family alloy  # one cell of the matrix
models/run.sh --list                            # the obligation manifest, as extracted
```

A bare run asserts obligation coverage over the whole catalogue, so it stays red
until both families cover all three scopes. That is the phase's remaining work
reported as such, not a broken instrument. A run that names a subset asserts
coverage over exactly that subset; `--no-coverage` runs the commands and reports
both matrices — obligation coverage and [Q4's removal matrix](#q4s-removal-matrix)
— without making an empty cell or an unrowed artifact fatal, which is what a
scope still being built uses. It never excuses a broken row.

## Conventions

`witness_` and `check` are the two existing `docs/ordinal-fs-tree/models/`
runners', unchanged. Two more exist because the catalogue's assumption table
needs controls whose expected result is *failure*:

| command | must |
|---|---|
| `check <OB>_<mnemonic>` | find no counterexample |
| `run witness_<OB>_<mnemonic>` | find an instance |
| `check expect_fail_<EN>_<OB>_<mnemonic>` | find a counterexample — a premise-break mutation |
| `run expect_unreachable_<EN>_<mnemonic>` | find no instance — an exercise-removal mutation |

`<OB>` is an obligation with its separators dropped: `TT-02.b` is `TT_02b`, and
a claim with no sub-identities is `TT_03`. Anything else prefixed `TT_`, `FN_`
or `SY_` names an obligation the catalogue does not define, and fails the run.

## Q4's removal matrix

The catalogue calls the artifact/transition removal matrix *a runner obligation
like any other: a removable artifact with no row fails the run*
([`docs/specs/semantic-contract.md`](../docs/specs/semantic-contract.md), *Q4
needs a matrix, not a claim*). It is recorded in the scope README the catalogue
names, one row per `(family, removable artifact)`:

```text
| Q4-<n> | <family> | the **<artifact>**[, <gloss>] | <obligation> | <evidence> |
```

`<obligation>` is a backticked obligation the catalogue defines, or `` `none` ``,
or empty when the row is `abstracted`. `<evidence>` opens with its class —
`mutation`, `argument` or `abstracted` — and a `mutation` names the
mutation-matrix row it fired (`mutation — row 17`, `mutation — row x2`).

The runner asserts the matrix in both directions, per family, exactly as it
asserts obligation coverage: every artifact the catalogue names has a row, and
every row names an artifact the catalogue names, cites an obligation the
catalogue defines or `` `none` `` or is declared `abstracted`, and carries a
citation that resolves. **The matrix obligation rides with coverage assertion
and has no flag of its own** — a matrix is owed once a family's column has
closed, which is what `--no-coverage`'s absence already says. The split that is
made is the one coverage already makes: an artifact with no row is excused by
`--no-coverage`, a broken row never is.

What no runner can check is *first broken*: a row naming the wrong but real
obligation reports identically to a right one. That is what the mutation
discipline is for, and the citation is how a reader reaches it.

**A finding about the catalogue, recorded rather than fixed — and since
DISPOSED.** The catalogue's own *`models/run.sh` is the one repository runner*
section listed **three** obligations while the matrix obligation it states under
Q4 makes four, and the runner numbers four. `routing-and-prose-k73` landed the fourth
in the catalogue as a manifest-neutral edit: the section now states *assert Q4's
removal matrix in both directions, per family* as obligation 4, with the rider
that it rides with coverage assertion and has no flag of its own. Nothing in
this runner changed — it already numbered four — so no cell moved and the
obligation count stayed at 128 across the edit.

## Reading a counterexample

Use `-t text`. Alloy's default `-t table` renders a temporal trace as an empty
grid — the tool reports a counterexample and shows nothing of it:

```sh
java -jar "$ALLOY_JAR" exec -q -t text -c <command> -o - <model.als>
```

## The phase's own run record

Recorded by `experiment-synthesis-k62`, the session that closed the formal
phase's measurement. **Every row was executed by that session**, not quoted from
the producing leaf that wrote the model — which is the whole point of a phase
gate, since each column's own run line was recorded before the other column
existed.

| cell | commands | coverage | result |
|---|---|---|---|
| `--scope task-tree --family quint` | 111 | 43 complete, 0 declared gaps, 0 empty, of 43 | exit 0 |
| `--scope finish --family alloy` | 180 | 61 complete, 0 gaps, 0 empty, of 61; Q4 matrix asserted | exit 0 |
| `--scope finish --family quint` | 228 | 61 complete, 0 gaps, 0 empty, of 61; Q4 matrix asserted | exit 0 |
| `--scope lifecycle --family alloy` | 73 | 25 complete, 0 gaps, 0 empty, of 25 | exit 0 |
| `--scope lifecycle --family quint` | 93 | 25 complete, 0 gaps, 0 empty, of 25 | exit 0 |

**The whole repository, in one invocation.**

```text
models/run.sh
-- commands run: 791
-- cells: 256 complete, 2 declared gaps, 0 empty, of 258
-- Q4 removal matrix (finish): alloy 10 of 10 rows, 2 `none`, 1 abstracted
                               quint 10 of 10 rows, 3 `none`, 1 abstracted
exit 0
```

All four scopes and both families, including the two delegated
`docs/ordinal-fs-tree/models/` runners (20 Alloy commands, 148 Quint). The
task-tree Alloy file contributes **104** commands here against the 103 its own
README records; the extra one is `cross-model-replay-k15`'s retained replay,
`witness_finding_a_world_write_during_an_open_scaffold_reaches_legacy`.

**The two declared gaps are the whole of what this run reports, and they are one
question.** The coverage section prints:

```text
  TT-24.c    alloy:gap quint:ok
  TT-24.d    alloy:gap quint:ok
```

`TT-24`'s two finish-context obligations were declared `out-of-bounds` by the
task-tree Alloy column because the runner's placement rule sends every `TT_`
command to that directory while the machinery each names lives in
`crates/grove-finish/models/`. The Quint column answered both, and one of its
answers had no control. Whether the two should be re-stated as `FN-`
obligations, kept as gaps, or given controls was a design question, not a runner
defect.

**SETTLED, AND THE FIGURES ABOVE ARE THE MEASUREMENT THAT PRECEDED IT.**
`obligation-placement-k63` retired both obligations and re-stated them under
`FN-` prefixes — `FN-32` and `FN-21.c` — on the rule that an obligation belongs
to the scope that can execute its context
([`obligations-follow-context-not-artifact`](../docs/adr/obligations-follow-context-not-artifact.md)).
The run line above is left exactly as `experiment-synthesis-k62` measured it,
because it is Experiment 2's frozen record of the tree **as the formal phase
built it**; the catalogue now carries **128** obligations rather than 129, and
these two rows no longer appear. `models/run.sh` also gained a **contested-cell**
report — one family answering what another declared out of reach, with whether
the answering family carries a control — so the next instance of this shape says
so in the runner's own output. After that leaf there are none.

**The report's own evidence sentence was false, and `obligation-placement-k68`
fixed it.** The coverage matrix calls a cell complete only with a property AND a
witness; the contested block credited a family that had supplied a property
alone and printed it as *answered*. A vacuous property — an antecedent nothing
reaches — is precisely what that report exists to expose, so the first version
could print the opposite of its subject, and a `--no-coverage` run prints it with
no counterweight at all. The line now distinguishes a complete answer from a
property-only one and counts the latter. Three durable controls went in with it —
`models/run-controls.sh` `contested-property-only`, `contested-control-seen` and
`contested-control-unseen` — the last two being the positive and negative pair
for `control_ob`, the extractor that decides whether the answering family carries
a control. They are the first controls in that file to assert a **line** rather
than a fatal exit, because this report is reported-never-fatal by design; the
file says so where they are defined.

## The run after the placement integration

Recorded by `obligation-placement-k68`, and it is a **second** whole-repository
run rather than a correction of the one above: that one is Experiment 2's frozen
record of the tree as the formal phase built it, and this one is the tree after
the placement rule, `FN-32`'s controls and the contested-cell fix. Every file the
runner reads had reached its final state before it was launched.

```text
models/run.sh
-- commands run: 794
-- cells: 256 complete, 0 declared gaps, 0 empty, of 256
-- Q4 removal matrix (finish): alloy 10 of 10 rows, 3 `none`, 1 abstracted
                               quint 10 of 10 rows, 3 `none`, 1 abstracted
exit 0        2h 00m 35s wall / 11931s CPU on a 16-core host
```

**Read the two runs against each other and three numbers moved, each for a
recorded reason.** The denominator is **256** rather than 258, because `TT-24.c`
and `TT-24.d` were retired to `FN-32` and `FN-21.c` and the catalogue carries 128
obligations; the **two declared gaps are gone** with them, so every cell is
filled by a command. The Alloy Q4 column reads **3 `none`** rather than 2,
because `Q4-6` — the cleanup marker — was re-decided from a cross-scope citation
to `TT-24.a` down to `none`
([`obligations-follow-context-not-artifact`](../docs/adr/obligations-follow-context-not-artifact.md),
clause 4, and `crates/grove-finish/models/README.md`). The command count moved
791 -> 794: three added — an Alloy and a Quint witness for `FN-32`'s marker half,
and the Quint control that isolates it — against one control replaced in place.

**Exit 0 established by enumeration rather than by the absence of the word
FAIL**, as `obligation-placement-k63` established it: the runner's distinctive
fatal diagnostics were searched for together — `placement error:`,
`runner error:`, `MISSING SCOPE`, `NO ROW`, *rows that do not resolve*,
*commands naming no obligation*, *declared gaps in BOTH families*, *not reported
by the run*, *failed to run*, *failed to complete*, *refusing to guess*, and a
`FAIL ` line. **Zero hits**, and the pattern was controlled against a synthetic
three-line input, which it matched three times. Both delegated
`docs/ordinal-fs-tree/models/` runners ran, and the **contested-cell section
printed nothing**, which is what zero contested cells looks like.

`models/run-controls.sh`: **10 passed, 0 failed** — the seven earlier controls
and the three the contested-cell report gained.

**Why the whole-repository run is still recorded separately from the six cells.**
A bare `models/run.sh` is not the union of `--scope`/`--family` invocations: it
additionally asserts obligation coverage over the **whole** catalogue in one
invocation, delegates to the two `docs/ordinal-fs-tree/models/` runners — a
positive control, since those suites are known green, so a repository run that
reports them clean while finding nothing anywhere else is reporting a broken
instrument — and fails on any `.als` or `.qnt` file in no known scope.

**No wall-clock figure is recorded here, deliberately.** The cells above were
run concurrently, and for part of that session the measurement host was also
compiling an unrelated package across sixteen processes. Command counts,
coverage matrices and exit status cannot be moved by contention; timings can.
The per-column timings that *are* worth having stay where they were measured, in
each scope's own README, each beside the run that produced it.
