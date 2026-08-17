# kind-references-k22

**Reviews:** `kind-references-k5`

## Goal

An inspection-only adversarial read of the ten session-kind reference files as
`kind-references-k5` left them, against `docs/specs/corpus-rule-ownership.md`'s
inventory rows for those files. The producer cut you with three specific doubts,
below; they are where to start, not the whole scope.

It is inserted ahead of `loop-step-references-k11` deliberately. A `review-*`
step normally re-derives and may land anywhere, but `k11` and `corpus-split-k6`
remove the statements this leaf deliberately duplicated (in `grilling.md`,
`driver.md`, `execute.md`, `TASK-FORMAT.md`) — so a review that ran after them
would be reconciling a historical diff against a corpus that had moved
underneath it, on exactly the material in dispute.

## The three doubts, stated

1. **Completeness per kind, which no test covers.** The nine areas in
   `tests/lifecycle_invariants.rs` pin thirteen rules; roughly thirty
   kind-reference rows have no instrument at all. The producer's contract was
   *incremental means non-duplicating, not incomplete*, and the only check on
   the second half was the producer's own reading. Take each file's inventory
   rows and ask whether a session that has read only the guaranteed core and
   `SKILL.md` could run that kind from what is left. The small files are where
   this is sharpest: `design.md` is now 39 words and `prototype.md` 59, and
   *already incremental* and *under-specified* are different diagnoses.
2. **A deleted mirror whose owner does not yet carry the rule.** This is the
   `durable-artifact-set` failure `skill-router-k20` caught one leaf upstream,
   and the producer hit a live instance of it:
   `glossary-is-the-forcing-function` is `CONTEXT-FORMAT.md`'s by the inventory,
   `SKILL.md`'s trigger 24 already names that file, and **`CONTEXT-FORMAT.md`
   does not state the rule yet** — `references/grove.md` and `grilling.md` do,
   and `corpus-split-k6` performs the move. This leaf deleted two mirrors of it
   (`references/requirements.md`'s *sharpen `CONTEXT.md` inline* and
   `references/execute.md`'s *updating `CONTEXT.md` inline as terms resolve*)
   and judged the rule still delivered, because `grove.md` is reached from
   trigger 18 and `grilling.md` from the threshold sentence. **Check that
   judgement, and then check every other deletion this leaf made the same way**:
   for each removed sentence, which file states the rule now, and is that file
   on the path of every kind the row binds?
3. **Rows moved in early, ahead of the leaf that removes their source.** The
   `research.md` rows `walk-away-check-per-system`,
   `citation-per-failure-mode-claim`, `silence-is-a-finding`,
   `both-researchers-get-one-brief`, `researchers-are-not-adversarial` and
   `research-output-path-per-kind`, and `integrate-review.md`'s
   `integration-escalates-redesign`, are stated here **and** still stated in
   `driving.md` / `TASK-FORMAT.md` / `references/execute.md` until `k11` and
   `k6` land. That direction is deliberate (a transient duplicate, never a
   homeless rule), but it is only safe if the wording here is a **complete**
   statement of the row — a reader of the later removal must not find that the
   surviving sentence lost a clause. Diff the two statements of each and say so.

## Also worth an adversarial eye

- **`references/requirements.md`'s threshold statement.** It is the grove's
  first contradiction resolution and it is stated once. Does it actually say
  what the spec's canonical statement says — always establish *what*, run the
  full procedure only at three or more interdependent open questions, record and
  proceed below — without smuggling the always-form back in, and without a
  session below the threshold reading it as *no interview at all*?
- **`prototype.md` and `review.md` both say polish is a defect.** The inventory
  gives `prototype-is-throwaway` and `the-five-reads-differ` different bounds, so
  two files may legitimately carry the claim from the producer's and the
  reviewer's side. Judge whether what landed is two sides or one restatement.
- **The three `finish.md` rules and `finish-is-the-drivers-to-discover`.** This
  leaf left `references/finish.md` byte-untouched, on the diagnosis that every
  rule in it is `{finish}`-bound conduct and nothing in it belongs to a sibling
  or a loop-step file. That is a claim, and it was made by reading, not by a
  test. Check it — `k11` moves one rule out of that file and needs the rest to
  be where the inventory says they are.
- **`tests/lifecycle_invariants.rs`'s ninth area.** Three claims and three
  near-miss fixtures. Are the fixtures genuine near misses, or does one of them
  fail for a reason unrelated to the claim it controls? Is `Binds::OnlyRequirements`
  the right scope, given that the delivery check for a `static({requirements})`
  rule is what holds it in that kind's own file?

## Done when

Findings are recorded with `path:line` anchors, each classified as a defect, a
trade-off to accept visibly, or noise. Inspection only: no test, build, lint or
format command, no edits to `content/` or `tests/`. If nothing is worth acting
on, create nothing and retire — the chain is lazy.
