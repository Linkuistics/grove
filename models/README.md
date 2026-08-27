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

**A finding about the catalogue, recorded rather than fixed.** The catalogue's
own *`models/run.sh` is the one repository runner* section still lists **three**
obligations while the matrix obligation it states under Q4 makes four. The
runner numbers four; renumbering the catalogue is `formal-synthesis-k16`'s,
since no session may edit the catalogue under the independence barrier.

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
