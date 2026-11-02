# owner-pack-checkouts-k86

**Integrates:** owner-pack-checkouts-k85


## Goal

Triage the committed design review and integrate valid findings before
skill-loading-replan-k80 consumes the owner-delivery contract.



## Context

Read owner-pack-checkouts-k85's findings and inspection limits from its
commit. Its reviewed artifact is Grove `80ce74974797`, the output of
owner-pack-checkouts-k84. Compare its evidence with the current design and
Ravel's local-source contract `7a894c782037`.

The human's three decisions of 2026-11-02 remain closed: mise owns the
owner's checkouts, each checkout stays at a published commit, and product
contracts remain tool-neutral.

## Done when

Each finding has an evidence-based disposition. Valid corrections agree
across the owning specifications, evidence, views and downstream handoffs.
Record relevant verification and leave a coherent contract for the planner.

## Notes

Use the integration family's triage procedure. The review is an input to
assess, not a list of mandatory repairs. This leaf owns repairs and their
verification. The review made no design, production or test-code changes.

## Decisions (running log)

**Base.** On 2026-11-02 this checkout was at `a13ce9f37bc7`, the commit of
owner-pack-checkouts-k85. That commit changed only `.grove/`, so the design
documents equal the reviewed artifact `80ce74974797`. Ravel's local-source
contract is `7a894c782037`.

**Triage.** Each of the four findings reproduces against the cited text.

1. P1, a real issue. Step 9 compares a file with its copy and then renames.
   No file-system call joins the two, so a write between them is lost. Step
   1 excludes every descendant of finish. Those descendants include the
   probes and the MCP servers of finish, and `codebase-memory-mcp` is also a
   hook installer. Step 1 also lists no job that can launch a writer later.
   The inverse reads, edits and renames with no check at all.
2. P2, a real issue. The complete state needs only a receipt in the backup.
   It can hold together with each other state, and nothing withdraws a
   receipt after an inverse.
3. P2, a real issue. A moved entry's state reads only its path. Step 3 takes
   the entries of the switch set out of the baseline comparison when a
   backup exists. A changed entry at either path therefore skips the
   classification that a first run requires.
4. P2, a real issue. The drifted route prepares a new manifest and goes to a
   new backup without a rehearsal. A first-run classification in step 3 has
   the same gap. Nothing binds the rehearsal of order 6 to a manifest.

None of the four needs a redesign. Each repair keeps the switch's steps, its
states and its owners. The three human decisions stay closed, and no repair
adds a Ravel or Grove interface.

**A platform measurement for finding 1.** A scratch test on this machine,
macOS 27.0.1 on APFS, called `renamex_np` with `RENAME_SWAP`. The path held
the new content, and the other name held the replaced content with its
original inode. `RENAME_EXCL` refused an existing target with `EEXIST`, and
it created an absent one. An atomic exchange can therefore publish a file
and keep the content that it replaced in one step.

**Repairs.** The owner specification changes in these places:

1. Step 1 lists each enabled job that can launch a writer. It excludes only
   finish's harness process and the driver chain. Another descendant of
   finish counts as another process unless the staging evidence clears it.
   A new staging trial names each descendant that writes a file of the
   switch set. Step 9 runs as one prepared command and publishes each file by
   an atomic exchange. It compares the displaced content with the copy and
   verifies the switched state at its end. The new section "Concurrent
   writers" states the exclusion and the detection. It accepts one case: a
   cleared descendant that writes through a descriptor that it opened
   before the exchange. The inverse has its own interval and the same
   exchange.
2. The layout decides the state, and a receipt only tells complete from
   active. A file with unclassified displaced content is in neither state.
   Step 3 and the inverse withdraw a receipt that no longer applies, before
   they change anything. The owner's later inverse copies the withdrawal into
   the manual.
3. A moved entry's state also needs the change identity that the latest
   complete backup records. A changed entry or link at either path is in
   neither state, so it stops for classification on every run.
4. The new section "The rehearsed plan" binds each rehearsal to the digests
   of its manifest and inverse. Step 8 stops unless the evidence covers the
   prepared plan. A changed plan reuses the evidence only for items of a
   rehearsed kind or items that leave the switch set. In finish, a needed
   rehearsal is a stop, and the owner runs it as the repair. No ADR change
   follows, because finish still runs only a rehearsed operation.

Order 6 records the rehearsed plan, and the test seams gain the controls
that the review asked for. The coordinated recovery row points at the
rehearsal rule. The evidence document records the exchange measurement and
these corrections. The cutover view, its caption, its summary and the
design README follow.

**Verification.** The prose checker reports no hard finding in the four
changed Markdown documents. `diagrams.json` parses. A scratch resolver
found all 32 relative anchors in the changed documents. A control file with
a missing heading made it report one failure.

`task design:harness-selection` served the views. A browser rendered the cutover view with 17 nodes, each new
label and no syntax error. The only console error was a missing
`favicon.ico`. This session removed the browser tool's two files from
`.playwright-mcp/`.

**In-session reviewer.** One fresh reviewer received the switch, the new
sections, resumption, rollback, the cutover contract and the test seams. It
also received the four required properties and the settled premises, without
these conclusions. It reported three P1, nine P2 and several P3 findings.
Lines refer to the working copy before the corrections below. Each one
reproduces against the text:

- R1, P1, valid. Step 1 runs once. The native probe of the native row
  launches sessions that run `codebase-memory-mcp` before step 8. The owner
  can also launch a writer during steps 4 to 7.
- R2, P1, valid. The staging trial runs the residue prototype, not finish's
  native set. A descendant that never ran there passes the exemption by its
  absence.
- R3, P1, valid. The owner's inverse after finish runs neither step 1 nor
  step 2, so it has no exclusion.
- R4, P2, valid, from this repair. A failed part makes its file drifted, and
  the inverse changes only switched items. The plugin keys then stay
  disabled while the Ravel settings go, so native delivery breaks.
- R5, P2, valid, from k84. The Ravel settings file is compared as a whole.
  `configure-grove` owns its time bound, budget and grants, so an owner edit
  makes it drifted.
- R6, P2, valid. Step 13 writes the receipt without a second check of the
  layout. The probes and finish's harness run between steps 9 and 13.
- R7, P2, valid. The active and complete states ignore a changed manifest.
  A newly classified entry can stay unmoved under a receipt.
- R8, P2, valid, made worse by this repair. A classification leaves no
  record that step 3 reads. An accepted change of identity stays drifted on
  every run.
- R9, P2, valid, from k84. A partial state cannot tell an interruption from
  an outside write, such as a `mise dot sync`. The next run discards that
  write.
- R10, P2, valid, from this repair. A record of digests cannot decide kind
  reuse, and it does not cover the switch's own command.
- R11, P2, a contract stated unclearly by this repair. A rehearsal inside
  finish contradicts the finish decision, which allows only resume rows and
  the inverse after teardown.
- R12, P2, valid. A withdrawal reaches the manual only on a stop. Drift
  after a successful finish has no run that withdraws its receipt.
- P3 items, each valid:
  - The exchange drops the file mode and the extended attributes.
  - An interrupted exchange leaves a prepared file under the displaced name.
  - Step 9 does not say whether a failed verification is a failed part.
  - "Steps 1 to 8 change nothing" is false while step 3 runs inverses and
    step 5 advances a pin.
  - The order-6 "byte for byte" condition conflicts with the fault points.
  - A seam says that step 1 stops, but step 2 stops.
  - The drift seams before the switch need an existing backup.
  - Step 1 counts the switch's own shell as a descendant.

**Disposition.** This is the second need for review, and it is more than
that. The switch's recovery has now passed through three focused reviews
and three in-session reviewers. Each round found defects that the previous
repair introduced or exposed. R4, R5, R7, R8 and R9 need new parts of the
state model. Examples are a journal of parts, classification records,
manifest-bound states and key-level treatment of the Ravel settings.

That is a rethink of the recovery model, not a repair, so this session does
not apply it. A new `design` producer, `owner-switch-recovery`, goes before
skill-loading-replan-k80. Its body carries the case for a checked model.

This session repairs only R11, the seam wording and one false sentence of
its own. They are consistency repairs with no new mechanism. The repairs of
the four k85 findings stay, because each one closes its finding. The new
design owns everything else above, including what those repairs introduced.
