# composition-k10

**Reviews:** `composition-k7` — the composition function on the `methodology`
seam and the file-ordering directive it consumes. Find its commit by that
handle; read that diff against the current source.

## Goal

Disprove the five judgement calls below. Four of them are places where this
increment either departed from the spec's literal wording or added a second
exemption to a claim the design calls load-bearing, and none of them is
mechanically checked — which is exactly the class the review chain exists for.
The mechanical parts (grammar, gate, selection, ordering) are pinned by test and
are not where the doubt is.

## Context

The producer added, to `src/methodology/`:

- a **file directive** `<!-- file: order=<n> -->`, the body's first line of every
  embedded markdown file, carrying that file's position in a composed mandate;
- a **per-file** rule (every file carries one, and no directive sits anywhere but
  the body's first line) and a **whole-embed** rule (no two files share a
  position), both build errors;
- `methodology::compose(units, kind)` — triggering units whose scope admits the
  kind, sorted by `(file_order, offset)`, joined by a single blank line;
- positions 1–9 across `content/`, with `prompts/continue.md` at 1 so the
  `MANDATE.md` rename in `mandate-delivery-k8` moves no position.

It also edited `docs/specs/mandate-delivered-methodology.md`, `CONTEXT.md` and
`docs/ARCHITECTURE.md` to reconcile them with what landed.

## Done when

Each doubt below is either confirmed as sound or written up as a finding. A
review that finds nothing creates no integrate leaf and simply retires.

**1. The golden's grain was narrowed from bytes to unit ids, and the spec was
edited to match.** The spec previously said the per-kind goldens hold bytes, and
the *Every kind's mandate states exactly one session ending* requirement leaned
on that: its two prose limbs were "pinned for drift by the golden snapshots,
which hold the ending unit's bytes verbatim". The producer narrowed the golden to
each kind's **ordered unit ids** — arguing that a byte-level golden of nineteen
~48 kB mandates moves on every `content/` prose edit, and a golden regenerated
every session is one nobody reads — and rewrote both spec passages to say the
byte-level pin belongs to the ending increment, beside the claim that needs it.

Attack it two ways. Is the churn argument true, or does the golden actually move
less often than claimed? And, more important: does `session-ending-k9` still have
what it needs, or has this quietly removed a pin that increment was going to
rely on and left it to rediscover the need? If the second, the finding is worth
more than the first.

**2. Total partition now has a second exempt region.** The spec calls partition
"the design's load-bearing structural choice", and the preamble used to be the
only region no unit covers. The directive is a second. The producer reconciled
that in three places — the spec's *Units partition a file*, `CONTEXT.md`'s
*Methodology unit*, and `src/methodology/parse.rs`'s module header — and argued
the exemption is safe because it is bounded by position (one line, first) rather
than by judgement.

Check the reconciliation is **complete**, not just present: every statement of
the partition claim anywhere in the tree, including ones the producer did not
find. A statement left saying "every body byte belongs to exactly one unit" is
now false, and a false load-bearing claim in a document sessions read is the
silent drift this design exists to prevent. Then check the argument itself: is
there any input where the directive region grows beyond one line?

**3. The fault a contributor meets, for each malformed shape.** The producer
chose: `NoUnitDeclared` outranks `MissingFileOrder` (an empty file is not yet a
file whose position means anything); an **indented** directive yields
`BodyBeforeFirstMarker` rather than a directive-specific fault (strictness only
withholds directive-hood, exactly as it does for markers); a **second** directive
yields `MisplacedFileOrder` wherever it sits, reported before its attributes are
read.

The question the design cares about is fail-open: **is there any shape where a
directive is silently absorbed rather than reported?** The producer believes not
— an indented or fenced directive leaves the file with no position, which is an
error — but that is the claim to attack, and the marker grammar's own asymmetry
argument is the yardstick. `DuplicateFileOrder` is also located at the offending
file's **first unit** rather than at its directive line, since `Unit` carries no
directive coordinate; judge whether the message compensates or whether the
coordinate should be carried.

**4. The order 1–9 assigned to `content/`.** MANDATE, SKILL, TASK-FORMAT,
BRIEF-FORMAT, CONTEXT-FORMAT, ADR-FORMAT, SPEC-FORMAT, grilling, driving. This is
the reading order of every composed mandate and **no test can catch a bad call**
— the golden pins that it does not change, not that it is right. Read a composed
mandate as a session would (`GROVE_TEST_UPDATE_GOLDENS=1 cargo test --test
methodology` regenerates the id order, or call `compose` directly) and say
whether the sequence lands the way a session needs it. Position 1 is fixed by the
rename and is not in question; the other eight are.

**5. Gaps are legal.** `check_file_order` settles totality, not density, on the
argument that the composer sorts by the key rather than indexing on it, so
requiring contiguity would make inserting a file renumber every later one. The
current values happen to be contiguous. Judge whether legal-but-unused slack is a
liability here — a `content/` whose positions drift into arbitrary integers is
harder to read than one whose positions are its order.

## Notes

**Inspection only.** Do not run test, build, lint or format commands, do not edit
production or test code, and do not redo the implementation. Read the committed
diff, the source, the spec, and the recorded verification evidence below; write
findings. Any fix, and all post-fix verification, belongs to the
`integrate-review-impl` leaf — which this session cuts **only if** it has findings
worth acting on.

**Verification the producer recorded**, so it need not be re-run: `cargo fmt
--check` clean; `cargo clippy --all-targets` clean; `cargo test --locked` green
(40 result groups, 0 failures); `bash plugins/install.test.sh` 11 passed. The
build gate was additionally shown to bite on the real corpus, by hand, in both
new directions — a duplicated position and a removed one — each reported with an
openable `file:line:offset`.

**Out of scope.** The driver still emits the launcher whole; nothing consumes
`compose` yet, and that separation is the fault line the planning leaf named.
Wiring it is `mandate-delivery-k8`. Whether any unit's `kinds=*` scope should be
narrowed is `unit-scope-audit-k4` and is not a finding here.
