# loop-driver-kill-anchor-k176

## Goal

Repair one broken anchor citation in `crates/grove-loop/src/loop_driver.rs` line
86 — `(driver-side-kill)`, which names nothing anywhere in this repository —
inside the file's frozen 615-line count, and reconcile the page that reproduces
the changed bytes.

## Context

- **The defect.** `LOOP_CONTROL_ENV`'s doc comment credits `GROVE_HARNESS_PID`
  and `GROVE_CLAUDE_PID` as *the retired pre-watcher handles (driver-side-kill)*.
  The token resolves to nothing: it is **not** one of `docs/ARCHITECTURE.md`'s
  twenty-four `<a id="…"></a>` anchors, not a file in `docs/adr/`, and not an id
  in `plugins/grove/conformance/rules.tsv`. `grep -rn 'driver-side-kill'` over the
  repository returns **exactly one** site — the comment itself. The positive
  control for that search is its sibling token: the same command returns twenty
  sites for `self-driving-loop`.
- **The claim is true and only the address is wrong**, which is the `paths-k142`,
  `lease-stale-reader-name-k169` and `prompt-rule-id-prefix-k174` shape. The two
  variables *are* the residue of the pre-watcher design, and the argument for the
  driver-side kill is recorded — just under a different anchor.
- **A better anchor already exists and it is the one the same comment block
  cites twice.** `self-driving-loop` (`docs/ARCHITECTURE.md`, *Lifecycle and
  resumption*) argues exactly this: *that kill is the launcher's job because it is
  the session's parent, outside whatever sandbox the session runs under; an
  in-agent self-kill is silently denied by sandboxes such as Codex's Seatbelt.*
  Check it is still the best target before retyping it — k157's and k158's finding
  was that a comment's cited section can hold less than the sentence leans on it
  for, and that a better anchor sometimes sits two sections away.
- **No instrument in this repository sees it, and that is why it survived.** It
  is not an intra-doc link, so `cargo doc --no-deps --document-private-items` is
  silent — the crate's thirty warnings name `loop_driver.rs` in none of them, and
  all eight of its intra-doc links resolve. It is not a Markdown link, so no link
  sweep reads it. It is not an `ADR <slug>` citation, so
  `every_adr_citation_names_a_decision_record` does not read it either. It was
  found by enumerating the block's seven parenthesised citations and resolving
  each one.
- **One page reproduces the changed bytes** and must change in the same commit:
  `docs/walkthroughs/grove-loop/20-the-loop.md`, fragment `«loop-control-env»`
  (lines 76-115). Its *The third choice: what a child may not inherit* section
  adjudicates the address in front of the reader and must be rewritten to describe
  the repaired citation; `concept-index.md` carries an entry naming it.

## Done when

- The comment names an anchor that `docs/ARCHITECTURE.md` carries, and no bare
  parenthesised citation in `crates/grove-loop/src/loop_driver.rs` names a token
  that resolves to nothing.
- **`crates/grove-loop/src/loop_driver.rs` is still exactly 615 lines**, and the
  comment still occupies its existing line span, so no ownership range, manifest
  `lines` value or fragment range moves. Check the substitution's length:
  `driver-side-kill` is sixteen characters and `self-driving-loop` is seventeen,
  so the line may need rewrapping *within* its existing span.
- `20-the-loop.md` reproduces the new bytes and its adjudication is rewritten.
- `book-check --repo . --book docs/walkthroughs/grove-loop --check all` is green
  over the book at whatever slice it is proved at when this runs, and every other
  book the commit touched is green too.
- `bash scripts/check.sh` is no worse than it was before this leaf.

## Notes

**This leaf is deferred behind the `grove-loop` book and says so.** The bytes it
changes are reproduced by chapter 20, and the freeze rule requires one commit to
carry the source change, every affected ledger and page, and a green validator
run. Run it after `what-could-not-move-k130` has taken the book to green `--final`
validation, alongside the other `loop_driver.rs` and `prompt.rs` address repairs
if they are still open.

**Do not widen this into a sweep of the file's citations.** All seven bare
parenthesised citations were enumerated at `the-loop-k168` — lines 1, 13, 31, 68,
76, 86 and 91 — and six resolve: `self-driving-loop` three times,
`user-owned-worktrees`, the spine constraint `walk-away-able`, and the leaf handle
`guard-loop-signal-k37`. This was the only one that did not. The file's eighteen
backticked record and path citations were checked separately and hold.

## Decisions (running log)
