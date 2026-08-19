# config-resolution-k4

**Reviews:** config-resolution-k2

## Goal

Read the reworked `docs/adr/complete-session-configuration.md` adversarially,
before `local-config-kdl-k3` writes code against it. The producer could not
judge its own prose against the one failure mode this rework was warned about,
and the minimum-coherent-set calls it made were made without a second reader.
Findings, not fixes.

## Context

`config-resolution-k2` rewrote the binding statement wholesale and took the
*Considered options* from six entries to ten. Everything it changed is prose, so
`cargo test` proves only that the corpus still parses, the rules still have one
owner, and the loaded paths still fit — nothing about whether the record is
coherent, honest or minimal. The producer's own `Decisions (running log)`
carries its reasoning; read that after forming a view, not before.

**Four specific doubts**, in the order they are most likely to be real:

1. **Does the record defeat the skimming reader?** The producer was told a
   reader who skims will assume the precedence lattice came back, and answered
   with "search order is not merge order" in the binding statement plus a
   revised *Keep a primary harness and layer kind/family overrides over it*
   entry. Whether that lands is a reader's judgement and the producer is the
   worst judge of it. Read the record cold and report what you think resolution
   does before you check.
2. **Is the set still minimum and coherent, or did it bloat?** Four options
   are new — *Let a second file replace the personal one*, *Track the delta*,
   *Put the delta inside `.grove/`*, *Warn and fall back*.
   `linkuistics:decision-records` admits a rejection only when it is non-obvious
   *and* someone will otherwise re-propose it, and each needs a reopen trigger
   that is a gate rather than a tombstone. Say which of the four fail that bar.
   The three untouched entries — shell execution, research-target comparison,
   inline methodology — should be checked for whether the rework left them
   still true.
3. **Should the ADR have been split?** The producer argued against, on the
   grounds that opacity and two-file resolution are one claim seen from two
   sides. The contrary reading is available: a record that must state a *search
   order* and a *placement rule* and a *trackedness rule* alongside *Grove
   infers nothing* may be two decisions sharing a slug. If it should split, name
   the two records and which citations move.
4. **Does the record claim behaviour that does not exist yet?** The rework was
   deliberately allowed to state the *decision* ahead of the code, while
   `docs/CONFIGURATION.md`, `docs/ARCHITECTURE.md` and `CONTEXT.md` wait for
   `local-config-kdl-k3`. Check the line held: an ADR describing what the binary
   *does* today, in the present tense, about a `.grove.kdl` nothing reads yet,
   is over it.

Two smaller ones. The producer also edited `content/references/driver.md` — it
deleted the *no repository stamp* clause it was told to, and additionally
weakened "launched by `~/.config/grove/config.kdl`" to "launched from personal
configuration" and changed "no default, family or **fallback**" to
"…or **inheritance**". Neither extra edit was asked for; judge whether each is
a correct reading of *a clause rescued by its neighbour* or scope the leaf did
not have. And the producer edited `.grove/BRIEF.md` to hand two unowned
reconciliations (`CONTEXT.md`'s **Grove configuration** entry,
`content/references/decompose.md`'s "two entries") forward to `k3`; judge
whether the brief was the right instrument or a question for the human.

## Done when

- Each of the four doubts above has an explicit verdict, and each verdict is
  argued from the record's own text rather than from the producer's log.
- Any finding carries the artifact, the location, and what specifically is
  wrong — enough that an integrating session need not re-derive the intent.
- The record is read against its neighbours.
  `docs/adr/supported-workspace-layouts.md` it now cites twice.
  `docs/adr/task-tree-transactions-fail-closed.md` it does **not** cite, yet it
  describes that record's teardown evacuation and finish-commit pathspec as the
  reason the delta sits beside `.grove/` — decide whether that needs a citation
  or whether one would be the cross-reference chain the format warns against.
  `docs/adr/one-build-owns-a-session.md` cites it twice and was left untouched;
  confirm both citations still hold.
- A review that finds nothing worth acting on **creates nothing** and simply
  retires. Cut `integrate-review-design` only if there are findings to act on,
  and insert it where `pick` reaches it next rather than appending it after
  `local-config-kdl-k3`.

## Notes

No code, and no edits to the ADR. This is the adversarial read; the fixes belong
to the integration step.

The producer spent no in-session reviewer, so nothing here has been pre-checked
by a second context.
