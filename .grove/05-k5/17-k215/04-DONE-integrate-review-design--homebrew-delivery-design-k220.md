# homebrew-delivery-design-k220

**Integrates:** homebrew-delivery-design-k219


## Goal

Triage the findings of `homebrew-delivery-design-k219` against the two
reviewed design amendments. Apply the valid corrections and leave a coherent
design for `homebrew-delivery-plan-k217`.



## Context

- Read the findings from the review's own committed task file, resolved by
  its handle. Its evidence names the reviewed Grove and Ravel commits.
- The producers are `homebrew-delivery-design-k216` and
  `ravel-default-skill-directory-k218`. Their running logs hold the human's
  decisions.
- The review spans Grove and the Ravel workspace
  `~/Development/AgenticOrchestration/Ravel.skill-directory-delivery`.

## Done when

- Each finding is classified against the requirements and the source, with
  its disposition recorded.
- Valid corrections and their affected consumers are reconciled in their
  owning repositories, with verification appropriate to each change.
- The design is coherent before the planning leaf runs. Any further work
  that needs its own producer is represented in the tree before planning.

## Notes

This leaf owns triage and corrections. The review recorded findings without
changing the design, implementation or recovery model.

## Decisions (running log)

**The findings (2026-11-06).** The findings are in the review's task file,
committed as `ee569a5428f1`. That commit changed only `.grove/`. The cited
documents therefore still hold the reviewed text of `93088bc51dfa` and
`a7e7be78aef7`, and each cited line was read again here.

**F1 (2026-11-06).** A real issue in k218's amendment. Item 5 of the
continuation and the owner's run after finish both start with the native
settings. In the active and complete states, the `skills` cell holds its
switched value, so item 5 of the native settings stops. Neither state then
reaches its row of the table.

**F2 (2026-11-06).** A real issue in k218's amendment. Item 4 of the native
settings exchanges the settings file and records no expected content. An
interruption before its comparison, or a late descriptor write, loses an
outside write without a report. That breaks the rule of "Concurrent writers"
and P1 of the model.

**F1 and F2 go to a new producer (2026-11-06).** Each repair changes the
recovery design. F2 needs a durable record before the first backup, new model
transitions and new rehearsal controls. F1 changes the entry point that
those transitions start from. A repair here would put an unreviewed recovery
redesign under a leaf that applies findings.

`native-settings-recovery-k221` (`design`) now sits before
`homebrew-delivery-plan-k217`. Its body names this review, the two findings
and one observation of this session. The continuation also omits the hooks
installation.

**F3 (2026-11-06).** A real issue in both amendments: a contract stated
unclearly. Four sentences still said that the settings name the delivered
sources. Ravel's amended contract delivers the installed sources with no
settings. The repair replaces each restatement with a citation of Ravel's
contract or with a true statement.

**F4 (2026-11-06).** A real issue in k216's amendment. The membership
decision still made the Codex marketplace file a part of WritingSkills,
beside its root. The delivery specification puts that listing at the root of
Grove's keg, and WritingSkills removes its own file. The repair takes the file
out of the WritingSkills parts and keeps the native proof as history.

**F3 repairs (2026-11-06).** The review named two sections of the delivery
specification and the glossary entry "Skill delivery". The sections are
"Ownership" and "Installation and bootstrap". A sweep of every line that holds "settings" in the delivery-facing
documents found the same class in four more places:

- "When configured" in the caller boundary specification.
- "A configured source" in the delivery specification.
- Two premises of the decision record "A skill states what binds when its
  dependencies are absent".
- The skill-delivery row of the harness views' README.

Each now states the installed-source default or cites Ravel's
contract. The owner design, `plugins/CONTEXT.md` and the delivery decision
record already agreed with the default.

**F4 repairs (2026-11-06).** The membership decision now keeps the Codex
listing out of the WritingSkills parts, and it records the native listing as
history. The view `membership-hooks` and its caption show the listing in
Grove's keg. The external-pack views list the change.

**The glossary conversion (2026-11-06).** The glossary entry "Skill delivery"
lay in a pending region. Its conversion to controlled English landed first,
as its own commit, before the repair of F3.

**Shipped guidance for planning (2026-11-06).** Two more files describe the
shipped release and change with the implementation. `README.md` says that
Ravel 2.0.0 reads the checkouts that its settings name. `plugins/README.md`
names the Codex marketplace of WritingSkills. They join the two files that the
planning inputs of k218 list.

**Verification (2026-11-06).** The prose checker reports no hard finding in
each edited document. The tests `composition_guidance` of `grove-llm` and
`reference_navigation` of `grove` read the glossary and every Markdown
reference, and all 29 passed. The edited `diagrams.json` parses as JSON. No
check renders Mermaid, so the edited view was read by eye.
