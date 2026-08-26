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
