# witnessed-observation-k26 — brief


## Goal

Make the production typed observer establish a witnessed mandate and its
verified relation to the captured tree. Disprove false attachment with process
and forced-reuse controls, obtaining the required macOS and Linux results.



## Context

Consume launch-witnesses-k25 through the existing `try_observe` and
DriverLease seams. Extend `crates/grove-loop/src/observation.rs` and
`crates/grove-loop/src/driver_lease/observation.rs`; retain namespace discovery
and mandatory record parsing. `TreeLifetime` already owns a directory File,
and `observe_with` currently captures tree/selected bytes before runtime reads.
Pass that accepted pin into runtime verification; do not reopen its path.

The complete algorithm and precedence are in the spec's One bounded observation
and Process death and tree identity sections. The prior idle-next-k22 review
and k23 integration accepted three old path-race injection gaps and a duplicate
selector call; they are not repairs owed here. New witness-acquisition/probe
windows and epoch-before-preparation controls are this leaf's responsibility.

## Done when

- Running carries the mandate and typed same-tree, previous-tree or no-readable-
  tree relation. Only directory contention on the captured pin can establish
  same-tree. Runtime identities, witness state and relation support independent
  comparison across captures; no caller must compare numeric identity to infer
  verified relation. Tree errors/absence still permit a fresh runtime summary.
- Execute the spec's directory-before-private shared probes under the short
  shared epoch guard. Unlock every successful probe immediately. A final
  successful private probe means Idle despite started leftovers or an earlier
  directory error; contended exact started evidence is Running only with the
  required binding. Equal numeric identity with an unlocked directory is Busy;
  a directory error with a locked started file is Unavailable. Locked empty or
  proper-prefix markers are Busy; invalid/extra bytes or missing/mismatched
  witness evidence are Unavailable. An inactive matching epoch is Idle.
- Controls remain read-only, nonblocking and close-on-exec; records retain the
  64 KiB bound and eight identity-race attempts, witnesses exactly eight bytes.
  Basenames cannot escape the namespace. Cover old versions, missing fields,
  malformed/truncated/oversized/mismatched records, FIFO/directory substitution,
  descriptor/path replacements and open/lock/probe errors. Admission's existing
  validation is unchanged and no observer probes the driver lease lock.
- Real launched-session positive controls reach `try_observe`. On both native
  macOS and Linux, independent shared probes prove both witnesses contended
  after acknowledged readiness; kill and actual reap then leave both probes
  successful despite started bytes. An exec'd surviving child cannot retain
  either witness or manufacture Running. Rename/remove and replace the root
  with a reused key; a newly arriving observer reports previous tree. Record
  actual test names, revision/source digest, command, kernel, filesystem,
  architecture and per-case results in this task. Keep it live until both
  platforms pass. The execution procedure is in the parent brief.
- Barrier controls independently force directory-first release with reused
  numeric identity/key, private-first release, both released, and release after
  verified directory probe but before private probe. Disable the directory
  check and observe the false attachment; independently disable epoch-before-
  preparation and expose replacement's new directory witness under the old
  epoch. The real Running positive control must still pass under each mutation.
  Restore production code and rerun. These simulate kernel interleavings and
  do not claim host inode reuse or a paused kernel teardown.
- Concurrent observers of unlocked started bytes cannot create contention or
  Running while a replacement retains old lease bytes. Test an old epoch reader
  delaying handoff and continuous viewers across repeated launches without
  observer-caused preparation failure. A separate foreign shared-holder process
  permits another shared probe and promptly defeats exclusive preparation.
- Retain separate after-capture and in-runtime pause controls, bounded handoff,
  recovery and no returned advisory guards. Snapshot tree and administration
  bytes for absent/non-jj/exact-alias/missing-namespace/multi-viewer cases. Use
  readiness/reap/lock events with failure timeouts, not elapsed sleep as proof.
- Existing Viewer callers compile and continue browsing conservatively until
  k27 consumes Running: withhold current row activity and NEXT for that new
  variant. Do not create a second provider or identity inference in the viewer.
  Record the intermediate user-visible behavior accurately, without exposing
  implementation staging in product messages.
- Reconcile observer/API/architecture descriptions, spec/glossary/ADR citations
  and affected source-derived books. G6's witnessed-view deferral remains for
  k27. Focused observer/lease/process tests and `bash scripts/check.sh` pass
  after all edits; preserve native-locking and suspension limitations.

## Notes

This leaf owns all cross-platform protocol evidence, not a later test catch-up.
Keep its controls close to the production observer and lease seams. Tests may
replace private lock/filesystem operations, never the public status provider.
If the verified route becomes unavailable, prepare the exact runnable source,
command, prerequisites and macOS result before asking for a Linux result under
the parent brief's fallback. Lack of Linux evidence cannot retire this leaf.

## Decomposition

The observer, deterministic interleavings and native process evidence exceed
one focused session. The original Done when remains this node's close condition.

- released-witness-k34 validates the versioned extension and privately probes
  released witnesses through production observation: Idle despite leftover bytes,
  conservative Unavailable while contended. It owns parser and ordinary witness
  acquisition rejection tests and the corresponding current documentation.
- witnessed-binding-k35 adds the typed mandate/relation and directory-before-
  private precedence, public observer positive controls and conservative Viewer
  handling. It owns marker states, directory errors, accepted-pin plumbing and
  new witness open/lock/probe replacement barriers with the eight-attempt bound.
- witness-interleavings-k36 owns forced numeric/key reuse, all release orders,
  both independent mutations with Running positive controls, epoch-before-
  preparation, delayed replacement and continuous observers/foreign shared holders.
- witness-platforms-k37 owns the real launched process controls and exact native
  macOS/Linux evidence, exec survivors and real root replacement. It preserves
  the parent brief's Docker route and keeps this node live until both hosts pass.

Every child updates its affected books and runs the principal gate. k35 preserves
the existing capture/runtime pause, no-escaped-guard and read-only controls;
k36 expands concurrent read-only snapshots; k37 reconciles the complete evidence
against this brief before node closure. G6's witnessed-view deferral remains for
k27, which commissions complete protocol review. No child closes this node by
substituting staged behavior for its inherited final contract.

## Decisions (running log)

Decompose at the production observer's released-private-witness seam. Proving
Idle is independently useful to existing Viewer callers and needs no tree
attachment inference. A contended witness stays Unavailable in k34; k35 must
add the directory probe before permitting Running. The complete native and
mutation evidence remains mandatory under this node.

## Interleaving and observer handoff

witness-interleavings-k36 and witness-replacement-observers-k39 deliver their
release-order, replacement-order, concurrent-viewer and foreign-process
controls through k38/k40/k41/k42. k38 records the directory-check mutation;
k41 records the independent epoch-before-preparation mutation. Both retain
real Running positives and exact restoration provenance. The current tests
still use the production observer, with internal synchronization and native
probes only; forced numeric/key reuse is a model, not host inode-reuse evidence.

k40 covers overlapping shared probes on released Started bytes, repeated real
launches under continuous viewers, and read-only snapshots across aliases,
absent trees, non-jj locations and missing namespaces. k42 adds separate
self-exec shared holders for each witness: acquisition readiness, independent
shared compatibility, bounded observation-only preparation failure, real
launch/admission, Unavailable, rollback, actual reap and recovery. Its
private-file barrier checks the containing-directory mutation lock while the
root witness is held. Existing reported-error, handoff and escaped-guard
controls remain green. The k42 task contains the full k39/k36 contract map.

For k37, run both `witness_foreign_directory_holder_preserves_launch_and_admission`
and `witness_foreign_private_holder_preserves_launch_and_admission` in the
required `witness_` suite. Their `witness_foreign_shared_holder_process` helper
returns without work when run alone; the two scenario cases establish process
evidence by launching it with the private fixture environment. The host run
here does not discharge k37's exact native macOS/Linux process-death,
exec-survivor and real replacement evidence. Keep k26 live for that leaf.
Complete protocol review remains assigned to k27; no competing reviewer was
used for these controls. No additional ADR decision was needed.

The k42 final gate passed all eight principal checks and six book validations.
Its full workspace run includes the error-matrix test isolated with the existing
fork-sensitive wrapper, preventing incidental inherited epoch locks from
short-circuiting its witness probes. k42 closes k39 and k36; their terminal
subtree stays in place with the mutation and restoration records.
