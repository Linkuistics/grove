# walkthrough-harness-routing-k19

**Kind:** impl

## Goal

Reconcile the Codex-harness paragraph in `docs/workflows/multi-step.md` with the
current per-leaf/per-kind harness routing contract.

## Context

The walkthrough currently says the harness exec'd for the loop is whichever was
chosen at bootstrap time and recorded in the grove's stamp. That was the old
contract. Current routing is leaf declaration → exact kind → kind family →
stamp; a review, integration or vendor-pair leaf may therefore run on another
harness while the same `grove do` loop is alive (ADR *model-per-task-kind* and
`CONTEXT.md` § Kind routing).

Surfaced by `confirmation-prose-review-k16` while cold-reading the walkthrough;
kept separate because it is not part of the confirmation-boundary decision.

## Done when

- The walkthrough describes the stamp as the harness fallback, not a binding
  that necessarily runs every task.
- It says that the harness may vary by leaf/kind without changing the on-disk
  tree or CLI cadence the walkthrough demonstrates.
- Any generated or nearby routing claim is checked for the same obsolete
  bootstrap-only model; the existing routing tests pass.

## Notes

**The claim was in two places, and the second one was the sharper falsification.**
`k16` found it in `multi-step.md`; the repo-wide grep found the same obsolete model
one page over, in `start.md`'s Multi-harness section: *"Later verbs read that stamp
and run the same harness, **so a single grove never spans harnesses mid-flight**."*
That is stated as an **invariant**, and per-kind routing exists precisely to break
it — `GROVE_REVIEW_HARNESS=codex` in a claude-stamped grove spans harnesses by
design. Fixed in the same pass: it is the finding's extent, not new work, and the
two pages are one walkthrough set (`multi-step.md` opens by picking up where
`start.md` left off, so a reader meets the false invariant *first*).

**The error is one of scope, not of fact** — worth stating because it shapes the
fix. The stamp still exists, is still written the same way, and is still read by
every later verb. What changed is that it went from being *the whole answer* to
being the **last** step of `leaf → kind → family → stamp`. So neither passage
wanted deleting; both wanted the missing three levels above the one they named.

**Checked and deliberately left alone**, so a later leaf need not re-derive them:

- `docs/grove.md:64` and `:79` describe the stamp as the grove's binding without
  claiming it runs every task, and `:77` already documents per-kind routing
  resolution — internally coherent.
- `docs/workflows/finish.md:137` ("written at bootstrap time … to bind this grove
  to its harness") is about stamp *cleanup* and makes no routing claim.
- `src/cli.rs`'s `--harness` help ("a lasting binding … read by every later `grove
  do`/`grove retire`") is true of the stamp, and `MODEL_ENV_HELP` — attached to the
  same commands via `after_long_help` — already spells out *leaf beats kind beats
  family beats stamp*. The generated help surface is current.
- `src/loop_driver.rs`'s doc comments already carry the four-level precedence
  (`:845`) and the reroute rule. No code claim was stale.

**No `CHANGELOG.md` entry, on precedent and on the preamble's rule.** The
behaviour being described shipped and was logged when it landed; this is stale
prose corrected against it, which is the same class as `stale-module-headers-k14`
and `confirmation-prose-integrate-k17` — both prose-only, both deliberately
CHANGELOG-free (verified by `jj diff --stat` on each). *A decision earns its entry
when its behaviour lands*; nothing landed here.

**`cargo test`: 622 passed, 0 failed** — including `harness_stamp`'s 14 selection
tests and the loop-driver routing tests. No test asserts on walkthrough prose, so
the suite is a regression check rather than evidence for the edit; the evidence
for the edit is the grep, re-run with `--hidden` (per `k17`, `.grove/` is a dotdir
`rg` skips by default), leaving only this task file's own quotation of the old
claim.
