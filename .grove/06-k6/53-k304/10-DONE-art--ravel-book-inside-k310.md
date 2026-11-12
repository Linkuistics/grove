# ravel-book-inside-k310

## Goal

Bring the figures of chapters 7 to 11 into agreement with the 2.0.0 text, in
the media that the book's contract admits.

## Context

- Ravel `docs/book/contract.md`, section 1, the figure needs.
- The figure lists of ravel-book-inside-k307 and -k308.
- The paused charter names `installed-files`, `source-components`,
  `process-authority`, `session-code-path`, `selection-cancellation` and
  `release-delivery`, and any other figure that names the worker.

## Done when

- No figure names the Bun worker, the SDK or the `libexec` layout as current.
- `release-delivery` shows the two formula channels and the promotion.
- `installed-files` shows the installation record and the keg layout of a
  skill source as the packaging guidance states it.
- `task docs:figures` passes. Each figure renders in the built book.

## Notes

Work in `Ravel.skill-directory-delivery` on `skill-directory-delivery`.

## Decisions (running log)

- The Ravel workspace has an unfinished art pass in working-copy change `wmvzxtmn`. It revises `source-components`, `installed-files`, `session-code-path`, `outcome-precedence` and `release-delivery`, and adds `session-authority`, `host-bridge` and `selection-deadline`. It updates the affected chapter links and descriptions. The work is not yet retired or sealed.
- The current render passes `task docs:figures`, `task docs:book:build`, `task docs:ste` and `task docs:breaks`. A check of the built HTML found all 128 embedded figure references on disk. The changed SVGs were inspected as rendered images, but not all were inspected in the built book at publication width.
- Resume by completing the whole-book art read and checking each item in the parent brief's art handoff against the revised figures and source. In particular, check the release activity's conditional promotion and recovery branches, the deadline cleanup sequence, figure legibility at publication width, and whether the unchanged cancellation and escalation figures need additions. Record each declined figure and its reason. Clear only the handoff entries actually closed. Re-run the final gates after the last edit. The proof sibling already exists.
- On resumption, the release activity gained an explicit channel decision: an unversioned cut ends without promotion; a versioned cut reaches the separate promotion after the compatible caller is ready. Its recovery branch directs the reader to correct the cause and retry promotion, which retains matching tap work and finishes installation. `task docs:figures:update` regenerated the SVG and layout. A `resvg` rendering shows the branch and labels; it is not yet an inspection in the built book at publication width.
- The promotion is optional even after a versioned cut. The decision now asks whether to promote the versioned release, so the no branch also covers a versioned release that stays in its own channel.
- The deadline sequence was compared with chapter 9's successful cleanup path and rendered at 750 pixels with `resvg`; the ordering and labels are legible. Its arrows describe attempts, as the key says; the prose separately covers cleanup failures. The release flow was rendered with `resvg` before the final wording correction. After that correction, `task docs:figures`, `task docs:book:build` and `task docs:ste` pass; `task docs:breaks` passed after the only prose change, and no prose changed afterwards. These checks do not replace the remaining whole-book art read and built-book inspection.
- A Linux Firefox guest opened the built chapter 11 at the release activity. Its SVG renders in the 750-pixel reading column; labels are small at that scale and the page supplies a full-size link. This inspected only the visible upper portion, not every changed figure or the full activity. The guest was stopped. The complete built-book visual inspection and whole-book art read remain open.
- The resumed pass measured figure text at the 750-pixel column. The release activity was 1106 units wide (about 9.5-pixel body text); the session sequence was 988 (about 10.6). Both fall below the figure guidance's approximate 12-pixel target. The release promotion now has its own activity partition, and the sequence uses shorter participant headings and narrower message wrapping. These retain the shared renderer and text size.
- One independent reader is checking the whole book against the art charter while this session owns the rendered inspection and corrections. This spends the leaf's one in-session review allowance.
- The complete browser inspection of the eight revised/new figures used Firefox in an isolated Linux guest, a 1200-by-1000 window and a measured 750-pixel book column. Scroll continuations covered every long figure and key. No overlap, clipping or missing image was visible. The narrower release activity is 788 units wide (about 13.3-pixel body text); the session sequence is 895 (about 11.7 pixels, approximately the guidance's 12-pixel target). Screenshots and measured bounds are in `/tmp/k310-shots/` for this session.
- The review's chapter 9 supervision-sequence omission is valid and actionable. The existing supervision-cancellation figure is now embedded beside the phase comparison with an explicit takeaway. The escalation figure's interval distinction is also actionable: its key now names the managed-process 500/10-millisecond polls and the harness's 20-millisecond pre-KILL polls. No new drawing or shared rendering change is needed.

- The independent review read all eleven chapters in full. Its remaining art findings were valid and actionable: the normal worker exits itself before parent reap, now shown as two self-actions; panel A continues after TERM rather than ignoring it. The shared supervision figure now names its INT sender without a page-relative “example below”. The two reported count discrepancies and the versioned-first installation route belong to proof and are handed forward in the node brief. No finding was classified as noise or an unclear contract.
- Declined figures, all editorial judgements: another formula-channel drawing would duplicate the two state tables; another chapter 11 component drawing would duplicate the linked source view and manifest table; intermediate cancellation diagnostics already have adjacent explanations and a numbered list; skill preparation already has an ordered list and recipient table. None was declined because the contract forbids it or the medium cannot carry it. The revised deployment view covers the skill payload and link chain. All art handoff entries are closed; proof entries remain. The existing proof sibling discharges the conditional successor rule.
- Final checks pass: `task docs:figures` (21), `docs:book:build`, `docs:ste` (zero problems, 35 files), `docs:breaks` (zero long runs, 11 chapters), `docs:examples` (27), and `docs:excerpts` (22). SHA-256 values for files under `docs/book`, `scripts` and `crates`, plus Taskfile and Cargo manifests/lockfile, did not change during the run. No executable example or excerpt changed in this art pass.
- Final browser inspection includes the revised worker-exit/reap sequence, escalation timing and reused supervision figure in full. Firefox decoded every chapter image, including hidden zoom copies, across all eleven chapter pages. This is visual/load evidence, not reader-comprehension evidence. The full-size SVG links remain. No art work remains open, and the proof sibling and publication leaves keep the parent node live.
- Ravel change `wmvzxtmn`, commit `7d9526e9`, seals the completed art pass. Its successor working copy is empty. The proof session starts from this completed figure set and the remaining proof handoff.
