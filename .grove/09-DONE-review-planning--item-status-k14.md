# item-status-k14

**Reviews:** item-status-k4


## Goal

Adversarially review the item-status decomposition before implementation uses
it. Find missing acceptance ownership, unsafe intermediate behavior or slices
that cannot be independently verified or fit a focused session. Produce
findings; do not implement the plan.



## Context

Find the producer's commit by item-status-k4. Its artifact is the root brief's
working-increment plan, full-width-view-k9, lifecycle-rows-k10, idle-next-k11,
and the witnessed-activity-k12 node with planning child witnessed-activity-k13.
Compare them against the root requirements and current `docs/specs/item-status.md`,
including both integrated design review chains through item-status-k8.

Attack these specific boundaries:

- Do the full-width and lifecycle leaves each leave useful behavior, including
  60 × 10, hidden/revisited viewport state, Unicode, counts and color-disabled
  selection, with tests and current documentation in the same leaf?
- Can the intermediate idle observer truthfully establish Idle without probing
  the driver's lease or borrowing ancestor context? Does active legacy metadata
  withhold NEXT rather than guessing? Does the common selector validate the
  entire tree before exclusion, including terminal/branch duplicate keys and
  multiple finishes? Check the planned driver-refusal documentation.
- Is idle-next-k11 small enough, and can it introduce one useful typed observer
  without a parallel capture/status implementation? Does the later witnessed
  planner inherit a precise implementation question, all acceptance obligations
  and an independently working predecessor, rather than vague deferred work?
- Does witnessed-activity-k12 retain every load-bearing control: Started/Reaped
  events, lease-owned descriptors and unconfirmed reap, both close orders,
  forced identity reuse and mutation failures, epoch-before-preparation,
  compatible multi-viewer probes, real Linux/macOS process results, verified
  tree relation, read-only/no-config behavior and admission regressions?
- Is review placed usefully, with the complete runtime protocol reviewed before
  node closure and integration created only when findings warrant it? Can the
  principal gate stay green when implementation changes source-derived books?

Source evidence from planning used Tier 2 project
`Users-antony-Development-grove.make-item-status-obvious-in-tui`, generation
`2026-09-15T11:44:22Z`. Exact snippets confirmed the current split renderer,
formatted Row labels, selector validation, creating namespace method and
launch/watch boundaries. Checked Rust paths and test paths had matching metadata
and no recorded gaps. Docs/scripts are excluded; prose metadata has changed,
so those were read directly. Search pages completed; some heuristic call edges
pointed to unrelated helpers, and exact snippets governed material claims.
Reconfirm coverage when using the source; this is bounded planning evidence.

## Done when

Record actionable findings against the producer's commit with paths and reasons,
or an explicit no-findings result. If findings merit integration, insert it
before the first later sibling containing live work, full-width-view-k9 at
creation time. Give that integration this review's stable handle rather than
copying findings into its charter. Keep preferences and accepted design
trade-offs distinguishable from defects.

## Notes

## Findings

Reviewed at commit `d2202fe693bb` (`item-status-k4`): the root brief's
Working increments and Verification sections, `11-impl--full-width-view-k9.md`,
`12-impl--lifecycle-rows-k10.md`, `13-impl--idle-next-k11.md`, the node brief
`14-k12/_witnessed-activity.md` and its planning child
`14-k12/01-planning--witnessed-activity-k13.md`, against the root requirements,
`docs/specs/item-status.md` as integrated through `item-status-k8`, the
driver-lease ADR, the two design review chains, and the current source. That
commit is the working copy's parent with no working-copy changes, so every
`path:line` below is the reviewed commit's. Entry names are as the tree stands
after this review's own insert; at the reviewed commit each of the four entries
after this leaf sat one position lower, with identical contents.
Each finding names its anchor, the evidence, and the classification this
reviewer proposes; the integration grades every one itself and may reject any.

### What held

- **Grammar.** Positions 09–13 are contiguous; keys 9–14 are unique and above
  the previous maximum; the node is `14-k12/` with one node file
  `_witnessed-activity.md` titled `# witnessed-activity-k12 — brief`; every leaf
  header is its position-free handle; this review carries the `**Reviews:**`
  line; the producer's commit names its handle. Every kind the subtree uses has
  a launch template (`~/.config/grove/config.kdl:30`–`:48`), so each leaf can
  be launched as cut.
- **The slices are vertical and ordered, not layered.** full-width-view-k9
  depends on nothing. lifecycle-rows-k10 needs k9's full width: 58 inner cells
  minus the 22-cell fixed area leaves the 36 the spec budgets
  (`docs/specs/item-status.md:361`–`:363`), which the 55 % pane could not
  supply. idle-next-k11 needs k10's activity column. Each predecessor retires
  before the pre-order walk reaches its successor, so no leaf waits on a
  *later* sibling, and each handoff in the root brief's table is a viewer a
  reader can use.
- **Requirement and scenario ownership.** Every requirement bullet in the root
  brief and every row of the spec's acceptance table has an owner among k9, k10,
  k11 and the k12 node brief; the exceptions are the two ownership gaps in F3
  and F4. The 60 × 10 layout, hidden/revisited viewport state, Unicode, counts
  and color-disabled selection are each named with tests and documentation in
  the leaf that ships them (k9 `:24`–`:53`, k10 `:22`–`:49`).
- **The idle observer is honest.** k11 derives Idle only from an absent exact
  workspace, namespace or lease, or a matching inactive epoch; an active epoch
  without the extension is Unavailable; the lease lock is never probed; exact
  discovery excludes ancestors and accepts aliases by device/inode
  (`13-impl--idle-next-k11.md:14`–`:17`, `:53`–`:61`). That is spec steps 2–3
  (`docs/specs/item-status.md:192`–`:207`) verbatim in effect. The leaf's source
  claims are true: `selected` checks only multiple live finishes
  (`crates/grove-loop/src/task_tree.rs:566`–`:590`), the viewer keeps its own
  duplicate-key check (`crates/grove-tui/src/observation.rs:118`–`:122`), and
  `Workspace::control_dir` creates the namespace
  (`crates/jj-workspace/src/lib.rs:139`–`:142`). Whole-tree validation before
  exclusion, terminal/branch duplicates, multiple finishes, finish-only
  remainder, running-branch children and no sentinel from viewing are all
  stated (`:44`–`:52`), and the driver-refusal documentation the spec demands
  (`docs/specs/item-status.md:288`–`:290`) is owned (`:82`–`:84`).
- **The witnessed node keeps every load-bearing control.** Started/Reaped with
  failed spawn, immediate exit and escalation (`_witnessed-activity.md:24`–`:28`);
  lease-owned descriptors through unconfirmed reap (`:26`–`:28`, `:86`);
  epoch-before-preparation (`:31`–`:33`, `:73`–`:74`); both close orders and
  forced identity reuse with the two mutation checks that must flip while the
  positive control stays green (`:71`–`:77`); compatible multi-viewer probes and
  foreign holders (`:78`–`:83`); real macOS and Linux process results (`:61`–`:66`);
  verified tree relation (`:19`–`:21`, `:44`–`:47`); read-only/no-configuration
  snapshots and admission regressions (`:90`–`:93`). The forced-reuse barriers
  are retained beside the real-process controls, as k8 required.
- **k13 inherits a precise question, all obligations and a working
  predecessor**: cut session-sized leaves against the landed observer and
  selector, with every scenario owned. The one clarity gap is F5.
- **Review is placed usefully.** This review sits before the first
  implementation; each producer cuts `review-impl` only once its artifact exists
  (k9 `:57`–`:59`, k10 `:58`–`:59`), k11's review is mandatory because its seam
  is load-bearing (`:97`–`:99`), the node cannot close before the implemented
  protocol is reviewed (`:96`–`:97`, `:106`–`:108`), integration follows the
  directory-local insert rule, and nothing is pre-created.
- **A visible trade-off, not a finding.** k9 ships the current two-line header
  and k11 replaces it with the five-line chrome (`11-impl--full-width-view-k9.md:16`),
  so k9's 60 × 10 assertions are rewritten once at k11. The planner chose no
  placeholders over churn-free chrome; either is defensible and the choice is
  written down.

### item-status-k14 F1 — idle-next-k11 omits the walkthrough obligation its edits trigger, and that obligation makes the leaf larger than its body suggests (medium)

Anchor: `.grove/13-impl--idle-next-k11.md:24` (move validation and selection to
the loop seam), `:29` (namespace discovery in `jj-workspace`), `:82`–`:87` (the
documentation list and the check command), `:95` (decompose if the work exceeds
one session); `.grove/_BRIEF.md:158`–`:163`.

Evidence: the grove-loop book's corpus is `crates/grove-loop/src/**/*.rs`
(`docs/walkthroughs/grove-loop/walkthrough.toml:23`) and it reproduces
`task_tree.rs` (`:218`) and `driver_lease.rs` (`:248`) byte-exactly; the
jj-workspace book reproduces `crates/jj-workspace/src/lib.rs` (`:94`).
`scripts/check.sh:112` runs `book-check --final` over every book, which asserts
that every byte of the declared corpus is reconstructed from explained
fragments, and `docs/specs/walkthrough-books.md:41`–`:47` requires an accepted
source change to land with its ownership ranges and fragments in the same
commit. So every edit k11 makes to those three files, and any new grove-loop
source file it adds for `try_observe` — matched by the glob automatically —
must be accompanied by manifest changes and new walkthrough prose in the two
books, written to their authoring rules. k11's Done when enumerates four
documents and the check command but not the books; the root brief names them
only generically, two sections above the increment table. A session reading
k11 will meet the obligation when the gate goes red, after the observer is
built, which is the stranding the root brief forbids (`:163`).

The omission also hides the leaf's size. k11 bundles three seams that are
separately verifiable: the shared validated selector with the driver's
duplicate-key refusal and repair guidance (grove-loop, `docs/USAGE.md`, the
grove-loop book); exact-workspace discovery (jj-workspace and its book); and
the observer, chrome and NEXT (grove-loop, grove-tui, both books). The first
is a working increment on its own — the driver, `grove-llm pick` and the viewer
refuse the same malformed tree, and the viewer drops its private check for the
shared rule — and its book repair is bounded to one file.

Proposed classification: real, actionable. Name the book obligation in k11's
Done when (the node brief already names it at `_witnessed-activity.md:95`), and
split the selector increment out ahead of the observer so each leaf's book
repair is bounded. The Notes' delegation of sizing to the implementer (`:95`) is
a visible trade-off, but it does not cure a stated-exhaustive list that omits
the largest item.

### item-status-k14 F2 — the Linux process-evidence obligation has no executable path in the repository and no named escalation (medium)

Anchor: `.grove/14-k12/_witnessed-activity.md:61`–`:66` ("Real macOS and Linux
subprocess controls … a macOS run is not Linux evidence");
`.grove/14-k12/01-planning--witnessed-activity-k13.md:32`–`:33` (every scenario
has an owner, including Linux/macOS real process results);
`docs/specs/item-status.md:461`–`:464`.

Evidence: `scripts/check.sh:14` states there is no CI in this repository;
`scripts/` holds the check script and the release scripts only, and the only
Linux mentions in it are the cross-compilation targets in
`scripts/release-common.sh:23`–`:24`, which build and never run tests; no
container or VM recipe exists under `scripts/` or `docs/` (a search for the
usual tool names over those trees matched the Linux target lines and nothing
else, so the instrument was live). The design's own log recorded that Linux
evidence at design time was kernel source and made the Linux run an
implementation acceptance scenario (`05-DONE-design--item-status-k6.md:452`–`:454`);
the plan carries it into the node's Done when and k13's ownership rule with no
mechanism and no statement that a human with a Linux host is expected. Every
kind in this subtree is AFK. A session that reaches a condition it cannot
satisfy stops, which is legitimate, but then the node close escalates as a
scope judgement nobody planned (`references/retire.md`, step 3), after all the
code is written.

Proposed classification: real, actionable. Either name the mechanism in the
node brief — a Linux host or VM the session may use is personal environment,
so it belongs in the brief as a pointer rather than in the repository — or
state that the leaf owning platform evidence is expected to end by handing the
Linux run to a human, with the macOS result and the exact command recorded so
the human's run is a one-line confirmation. Which of the two is the human's
choice; the brief should not leave k13 to discover the gap.

### item-status-k14 F3 — idle-next-k11 activates the viewer's epoch guard and its accepted driver-liveness trade-off, but owns neither the control nor the user-facing documentation (medium-low)

Anchor: `.grove/13-impl--idle-next-k11.md:50` (tree capture before the
runtime-only epoch guard), `:55` (contention is Busy), `:74`, `:82`–`:85` (the
documentation list); `.grove/14-k12/_witnessed-activity.md:83`–`:84` (pause
controls); `docs/adr/one-live-driver-per-working-tree.md:83`, `:173`–`:177`;
`docs/specs/item-status.md:443`.

Evidence: k11 is the first increment in which a viewer takes the shared epoch
guard (spec step 3, `:196`–`:197`). The driver's epoch writes go through
`acquire_epoch_file` (`crates/grove-loop/src/driver_lease.rs:336`), which tries
nonblocking, prints `waiting for … session epoch lock for …` to the human's
terminal on contention (`:352`–`:355`) and waits up to the 30 s handoff bound
(`:35`) before the loop stops. Integration `item-status-k5` accepted this as a
visible trade-off and the ADR states it (`:173`–`:177`). From k11 on, running
`grove view` beside a driver can print that diagnostic during ordinary overlap,
and a suspended viewer can stop the loop `blocked`; both are new user-visible
behaviour of *this* increment. Yet the pause-outside/inside-the-guard control
(spec `:443`) is owned only by k12 (`:83`–`:84`), and k11's documentation bullet
asks for selection guidance and the seam description but not for this
consequence. The two-phase capture structure the control exercises exists in
full at k11; nothing in it depends on witnesses.

Proposed classification: real, actionable, small. Give k11 the
pause-outside/inside control and a usage sentence on the driver diagnostic and
the suspension bound, so the increment that introduces the behaviour documents
and tests it; k12 keeps the control for regression.

### item-status-k14 F4 — no leaf owns the user-guide coverage row that itself defers to the implementation (low)

Anchor: `docs/specs/user-guide-coverage.md:74` (G6);
`.grove/11-impl--full-width-view-k9.md:51`–`:53`;
`.grove/12-impl--lifecycle-rows-k10.md:47`–`:48`;
`.grove/13-impl--idle-next-k11.md:82`–`:85`;
`.grove/14-k12/_witnessed-activity.md:94`–`:96`.

Evidence: G6 says the item-status design "adds full-width Tree/File switching,
leading lifecycle glyphs/words/colors and persistent RUNNING/NEXT summaries;
document these and the shared selector's new driver duplicate-key refusal and
repair guidance with the implementation" — a deferred obligation the design
integrations wrote for exactly these leaves. `crates/grove/tests/user_guide_coverage.rs:17`–`:31`
checks row ids, anchors and the coverage map, not row text, so the stale row
survives a green gate. No leaf in the subtree names the file: k11's list is
USAGE, ARCHITECTURE, module-decomposition and CONTEXT-MAP, and the node brief's
is usage, architecture, module and context-map.

Proposed classification: trace gap, trivial. Name the file in k9, k11 and the
node brief so each leaf reconciles the row for what it ships.

### item-status-k14 F5 — the node brief reads as forbidding the only slicing its child planner can perform (low)

Anchor: `.grove/14-k12/_witnessed-activity.md:103`–`:104` ("Publishing
metadata with no useful reader is not a completed product increment");
`.grove/14-k12/01-planning--witnessed-activity-k13.md:29`–`:30` ("Each can be
demonstrated or verified without waiting for a sibling"), `:38` ("unused
witness metadata is not an independent product increment"), `:45`–`:46`;
`docs/specs/item-status.md:106`–`:108`.

Evidence: the spec makes the first RUNNING depend on runner events, the
lease-owned value, both witnesses, observer steps 4–5 and the viewer binding
together; no strict subset changes what a user sees, because a partial record
is by design unrecognized (`:106`–`:108`). So k13 can only cut steps that are
test-verifiable but not product increments — the spine's expand → migrate →
contract shape for work no vertical slice can land. The brief uses "product
increment" for the node and "verified" for the leaves, which is consistent on
a careful read, but the sentences sit beside "demonstrated" and a Notes line
about finding "a smaller useful product increment", and a planner reading them
together may either bundle the whole protocol into one leaf or stop to ask.

Proposed classification: contract stated unclearly, cheap. One sentence in the
node's Decomposition saying that its leaves may be verifiable steps whose
product behaviour arrives with the last of them removes the ambiguity.

## Decisions (running log)

- F1–F3 are actionable and change leaf bodies before implementation consumes
  them; F4 and F5 are cheap to settle beside them, so an integration step is
  warranted. Its leaf is inserted ahead of full-width-view-k9, the first
  later sibling entry in this directory with live work, as the charter
  and the directory-local rule both direct. Its body names this review by
  handle and carries none of the findings, so rejecting one stays an ordinary
  outcome.
- No in-session reviewer was spent; this session is the adversarial read. No
  test, build, lint or format command was run. Every line number above was
  read from the files at the reviewed commit, which is the working copy's
  parent with no working-copy changes.
- Source was read directly rather than through the graph, so no graph-coverage
  claim is made. The two negative claims — no Linux test path, no leaf naming
  the coverage row or the books — rest on enumerating `scripts/` and the five
  planned bodies, not on a clean pattern sweep.
