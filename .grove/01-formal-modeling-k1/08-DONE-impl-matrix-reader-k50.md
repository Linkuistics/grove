# matrix-reader-k50

## Goal

Teach `models/run.sh` to read each family's **Q4 artifact/transition removal
matrix** out of its scope README, and to fail the run on a matrix that is absent,
incomplete, or self-inconsistent. The catalogue calls the matrix *a runner
obligation like any other: a removable artifact with no row fails the run*
(`docs/specs/semantic-contract.md`, *Q4 needs a matrix, not a claim*), and
nothing enforces it today.

## Context

`exits-k49` closed the alloy column of the finish scope and wrote the first
matrix — ten rows, in `crates/grove-finish/models/README.md` under
*`TODO.finish_process.md` Q4*. That session **decided this question and cut this
leaf**; its reasoning is recorded there under *Whether `models/run.sh` grows a
matrix reader*, and the short form is:

- **The row set is derivable from the catalogue**, which is the runner's founding
  principle (*the catalogue IS the manifest*). One sentence names the ten
  removable artifacts: *the reserved witness, the evacuation manifest, its ready
  mark, the correlation ticket, the quarantine, the cleanup marker, the replace
  transition, the index image, the recorded anchor, the deletion fingerprint*.
  Extract it the way the obligation manifest is extracted — a match outside
  fenced blocks — rather than transcribing it into the runner.
- **What a reader can assert is a two-direction coverage matrix at a second
  grain**, exactly as the command coverage already is: every named artifact has a
  row in every family whose column is closed; every row names an artifact the
  catalogue names; every row's cited obligation is one the catalogue defines, or
  `none`, or a declared `abstracted`.
- **What it cannot assert is *first broken*.** A row naming the wrong but real
  obligation reports identically to a right one. The discipline that reaches the
  content is the **citation** each row carries — a numbered mutation-matrix row,
  or an argument, or an *Abstractions* entry — and a reader can check that the
  citation RESOLVES even though it cannot check that it is the first.
- **A matrix is owed only once a family's column closes.** The finish scope's
  alloy column is the first that has; its quint column and every other scope's
  are still being built, so the reader must not turn `--no-coverage`'s own
  meaning inside out. Decide whether the matrix obligation rides with coverage
  assertion or is its own flag, and say which in the runner's header comment.

The alloy matrix's own shape is the only worked example. Two of its ten rows read
`none` and one is `abstracted`; two rest on argument rather than on a mutation;
and one (`Q4-6`) cites `TT-24`, an obligation **no command in the finish
directory may answer** under the placement rule — so a reader that required every
cited obligation to be answerable *in the same scope* would reject a correct row.
That case is the sharpest thing to design against.

## Done when

- `models/run.sh` reads each in-scope family's removal matrix out of that scope's
  `README.md`, and reports it in the coverage summary the way declared gaps are
  reported.
- The three assertions above are made in both directions and fail the run when
  broken, with a message that names the artifact or the row rather than the file.
- The row shape the runner reads is documented in the runner's header comment
  beside the `GAP` shape it already documents, and
  `crates/grove-finish/models/README.md`'s matrix is edited to match it if the
  chosen shape differs from what is written there.
- Whether the obligation rides with coverage assertion or with a flag of its own
  is decided and recorded, and the finish scope's alloy run stays green either
  way.
- `models/run.sh --scope finish --family alloy` still passes with coverage
  asserted, and a deliberately broken row (a typo'd obligation, a deleted row) is
  shown to fail it.

## Notes

Placed ahead of `quint-models-k10` deliberately: the quint column will owe a
matrix of its own, and a reader that lands after it would be checking a document
nobody was obliged to write.

This is a runner change, not a model change. It touches no `.als` or `.qnt` file
and states no obligation; the independence protocol between the two families is
untouched by it.

## Decisions (running log)

**The matrix obligation rides with coverage assertion; it gets no flag of its
own.** `--no-coverage` already means *this family is still being built*, and a
matrix is owed only once a family's column has closed — so the two conditions
are the same condition, and a second flag would let a run assert coverage while
excusing the matrix. The split that IS made is the one the coverage matrix
already makes and the runner already treats asymmetrically: the catalogue→row
direction (an artifact the catalogue names with no row) rides with coverage,
while the row→catalogue directions (an invented artifact, an invented
obligation, a dangling citation) are fatal always — exactly as a command naming
no obligation (`bad_commands`) is fatal irrespective of `--no-coverage`. A
broken row is not an empty cell. Recorded in the runner's header comment under
obligation 4 and in `models/README.md`.

**The row set and the README that holds it are both read out of the catalogue,
not transcribed.** The catalogue names the ten removable artifacts in one
sentence and names `crates/grove-finish/models/README.md` in the sentence before
it, so the runner extracts both by matching outside fenced blocks over the
catalogue joined into one line (the list spans a line break). The scope that
owes a matrix is then derived by matching that path against `scope_dir`, rather
than hardcoding `finish`. This is the runner's founding principle — *the
catalogue IS the manifest* — at a second grain; transcribing either would make
an artifact the catalogue later adds invisible to the check that exists to catch
exactly that.

**The row carries its family, as a fifth column.** Both families record into the
one `crates/grove-finish/models/README.md`, so a row must be self-identifying
the way a `GAP` line is. The shape read is
`| Q4-<n> | <family> | the **<artifact>**[, <gloss>] | <obligation> | <evidence> |`,
and the alloy matrix's ten rows were edited to match. The artifact key is the
leading `the`/`its` plus the bolded name, so the gloss after the comma stays
free prose while the key is the catalogue's own words character for character.
Rows of families a run did not select are ignored, exactly as their commands
were not run.

**The obligation check validates against the WHOLE catalogue, never the selected
manifest.** `Q4-6` cites `TT-24`, which the placement rule forbids any command
in `crates/grove-finish/models/` from answering — a scope-local check would
reject a correct row. That case was the sharpest thing to design against and it
is what fixes this choice.

**The citation-resolution direction is implemented, because it is the only one
that reaches the row's content.** Evidence class must be `mutation`, `argument`
or `abstracted`; a `mutation` must name a mutation-matrix row that exists in the
same README (resolved against the rows under its *mutation matrix* heading); an
`abstracted` requires an *Abstractions* section. What stays unmechanised, and is
said plainly in the runner's header, is *first broken*: a row naming the wrong
but real obligation reports identically to a right one.

**A catalogue finding, recorded rather than fixed.** The catalogue's own
*`models/run.sh` is the one repository runner* section still enumerates **three**
runner obligations while the matrix obligation it states under *Q4 needs a
matrix, not a claim* makes four. No session may edit `docs/specs/semantic-contract.md`
under the independence barrier, so the runner numbers four and the discrepancy is
recorded in `models/README.md` for `formal-synthesis-k16`, alongside the other
catalogue findings the Alloy column recorded rather than fixed.

**A row may cite a CLAIM, not only one of its sub-identities.** The first green
run caught this against `Q4-6` itself: the register's shared-safety list names
`TT-24`, while the manifest's unit is `(family, obligation)` and therefore
carries only `TT-24.a` – `TT-24.d`. A citation check that demanded an exact
manifest entry rejected the one row the brief singled out as the sharpest case.
The reader now accepts `<claim>` when any `<claim>.<letter>` is in the manifest,
which is the same identity relation the coverage matrix already uses in the
other direction.
