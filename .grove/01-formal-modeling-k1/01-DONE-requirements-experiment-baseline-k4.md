# experiment-baseline-k4


## Goal

Freeze the observable and experimental baseline before modelling changes our vocabulary or implementation work changes the evidence.



## Context

This is the first executable leaf. The repository already contains prior Quint/Alloy experience in `docs/formalism-findings.md`; preserve it and pre-register this as a new, bounded experiment. The Grove binary/installed-methodology hash skew reported in the root brief must also be resolved before later sessions are driven.

## Done when

- A durable before-change ledger records CLI help, commands, output/error categories, exit meanings, configuration/default/env behaviour, current `.grove` format, package/binary names, embed/install behaviour, release targets, MSRV, and material dependencies.
- Representative successful and failing commands have captured, reproducible results, including current-format initialization and legacy/foreign-format handling.
- `docs/formalism-findings.md` contains an “Experiment 2” pre-registration without rewriting the prior findings. It states hypotheses and measures for unique/overlapping findings, Alloy 6 temporal versus Quint action modelling, counterexample usefulness, derived tests, model placement, synchronization cost, state-space/tool cost, and false-confidence hazards.
- Exact Quint, Alloy, Java/solver, runner, Rust, Git, and jj versions are recorded. Missing tools or pre-existing failures are separated from failures introduced later.
- The Grove methodology/binary hash skew is either repaired and verified or recorded as an explicit launch blocker with the exact remediation command.

## Notes

Do not write the new models, rewrite architecture/user documentation, or change product code here. Baseline facts may be added to existing durable documentation or a clearly named experiment artifact referenced from `docs/formalism-findings.md`.

## Decisions (running log)

**The methodology/binary hash skew is repaired, not a blocker.** Three sources
agree on `10db034c77d5afe455998ad5ac58c969c66aaa0d0312077172bc1bf1bf96444b`: the
installed `grove-llm --content-hash`, all three provisioned skill directories
(`~/.claude`, `~/.codex`, `~/.pi/agent`), and this working tree's own `content/`
re-hashed by the algorithm in `src/methodology.rs`. The skill directories carry
mtimes of 2026-08-24 17:50, after the tree was scaffolded at 17:09 — the
remediation was a re-provisioning run, which bare `grove` performs before it
touches a working tree. The stale `8501…` the root brief records is gone. No
later leaf needs to treat this as a launch precondition. Recorded in
`docs/preservation-baseline.md` §4 with the fourth row that matters most: this
checkout's `content/` and the driving binary's embed are the same methodology,
so a leaf that edits `content/` breaks that agreement until `grove` is rebuilt.

**The baseline lives at `docs/preservation-baseline.md`, framed as a contract
rather than as an experiment snapshot.** Put to the human, who chose it over
folding the ledger into `docs/formalism-findings.md` and over an
`experiment-2-baseline.md` naming. The consequence is deliberate: `documentation-k2`
and `implementation-k3` leaves are expected to check changes against it and to
record approved exceptions *in* it, which a frozen experiment artifact would not
invite. `docs/formalism-findings.md` references it; it does not own it.

**No grilling was staged.** `requirements.md` sets the threshold at three or more
open questions with interdependent answers. This leaf had one — where the ledger
lives — and the rest was pre-decided by the root and node briefs, which enumerate
the ledger's contents and the pre-registration's measures. Staging an interview
over settled ground would have cost attention and returned nothing.

**No test seams were agreed, because this leaf covers no code that will be
tested.** The brief forbids product-code change here. The seams that do want
human agreement are the crate-facing semantic contracts, and those are
`design-model-contract-k5`'s and `formal-synthesis-k16`'s to put.

**"Preservation baseline" is not added to `CONTEXT.md`.** It names an artifact of
this workstream, not a concept in grove's domain, and `CONTEXT-FORMAT.md` keeps
process vocabulary out of the glossary ("It is a glossary and nothing else").
The file's own header is its definition.

**Two coverage claims were corrected before they were written down.**
`tests/lifecycle_invariants.rs` is about methodology-corpus *delivery*, not tree
format; `tests/tree_access.rs` is about the advisory lock, not witness refusal.
Both were plausible from the filename. The §10 table now names what each file
actually asserts, and names the five preserved claims that **nothing** asserts —
the full verb surface, the exit-1-vs-2 split, MSRV, the release targets and glibc
floor, and the Homebrew formula rendering. Those five are the rows a refactor can
break with a green `cargo test`, which is the ledger's main reason to exist.

**Alloy's dead-tool hazard is a live condition on this machine, not a
hypothetical.** `java` on `PATH` is Corretto 16, below Alloy 6's floor;
`run-alloy.sh`'s own JDK search is what makes the suite green. A JVM that fails
to launch turns every `check` into a pass and every `witness` into a failure. The
single repository runner the node brief requires must keep that abort-on-launch-
failure property. Recorded in `docs/preservation-baseline.md` §1 as a named
carry-forward for the model-runner work.

**A `review-requirements` step was cut, and inserted rather than appended.** The
pre-registration is the one artifact in this phase whose value is destroyed by
being corrected late: reviewed after `alloy-models-k6` and `quint-models-k10`
have run, a change to its measures cannot be told apart from tuning them to the
findings. `decompose.md` says a `review-*` step re-derives and so may be appended
anywhere — that is about the soundness of its handoff, not about whether the
review is still worth running, and here timing is load-bearing. So
`experiment-baseline-k29` sits at position 02, ahead of
`design-model-contract-k5`, and its body names the three doubts this session
could not resolve about its own work: whether *material finding* sorts borderline
cases the same way for a session that was not in the room, whether §10's
remaining coverage rows survive the same filename-plausibility error that broke
two of them, and whether §11's omission list is complete.
