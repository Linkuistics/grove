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
