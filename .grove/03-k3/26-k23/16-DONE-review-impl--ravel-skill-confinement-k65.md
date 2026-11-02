# ravel-skill-confinement-k65

**Reviews:** ravel-skill-confinement-k59

## Goal

Read the confinement read grants of Ravel commit `e343587583eb` adversarially.
Read them against `docs/specs/skill-discovery.md` at `822c0ed2`, "Read
access and delegation", and against the clarifications that the commit adds.
Produce findings, not fixes. The parent `4b0f54c575a7` is the base of the
diff.

## Specific doubts

- `crates/ravel/src/confinement.rs`, `skill_trees`, canonicalises each tree
  before the launch. `keyed_launch::command_with_grants` canonicalises it
  again at the launch. Can a replaced path between the two grant a different
  tree? Can a harness run, which skips the probe, receive a tree that the
  managed process never probed?
- The protected set for a tree is the configuration directory, the settings
  file and the inherited `RAVEL_EXIT_FILE`. The writable set is the working
  directory, the private run directory and `--exit-dir`. Is a known result
  channel missing, such as the channel that a harness run allocates, or an
  `--exit-dir` that is a symlink?
- A writable grant that overlaps the whole cache root refuses. The spec said
  only snapshots and the descriptor. Is the wider rule right, and does it
  refuse a layout that an owner can reasonably use?
- The Seatbelt profile allows `file-read*` and `file-map-executable` on each
  tree, and relies on the default deny for writes. The producer checked by
  hand that a hard link, `xattr -w`, `touch -t`, `chflags`, `chmod` and `mv`
  of a tree file fail. Is another write path open, such as a clone, an
  `exchangedata` or a write through an open descriptor that a parent passed?
- The read probe runs `ls -a` on each tree in a sandbox that grants only the
  trees. Does that prove the grant that the real launch receives, which also
  has writable roots and runtime reads? Can the probe pass on Linux and the
  real `--ro-bind` fail?
- On Linux, a nested launch in a confined parent sees no settings and runs
  with delivery disabled when its entry is readable. The commit records this
  in the spec in place of a refusal. Is that a contract change that needs a
  design decision?
- `crates/ravel/tests/skills_confinement.rs` probes reads and writes through
  `sh -c`. Can a probe report `denied` for a reason other than the sandbox,
  such as the mode bits of the snapshot? Which probes have a firing control?

## Done when

- Each doubt has a finding, or a reasoned statement that the code holds.
- The findings cite file and line at the reviewed commit.

## Decisions (running log)

- Review boundary: Ravel commit `e343587583eb83c450d68b390be7d1e1d1727425`,
  parent `4b0f54c575a7`, against the reviewed discovery contract at
  `822c0ed2` and the producer's explicit clarifications. This is an
  inspection-only review. No tests, builds, lint, formatting or reproduction
  commands run, and no Ravel files change.
- Graph evidence: Verify tier. Indexed the previously unindexed
  `Ravel.skill-directory-delivery` workspace with persistence disabled;
  generation `2026-11-02T05:30:19Z`. Searches and bounded traces identify
  the grant preparation and launch routes, with no remaining pagination.
  Coverage reports no recorded gaps in the inspected Rust files. The graph
  describes the later feature tip `f1aff8e5`, so `jj file show` and the
  revision diff supply the reviewed bytes and line numbers. The confinement
  implementation and its fixtures are unchanged between those revisions.
  The fast index excludes `docs`; direct reads supply the contract, ADRs
  and book evidence. Graph edges include heuristic noise, so material
  relationships are confirmed in source.
- Path identity: a second canonicalization can replace a previously checked
  tree with a different directory. The lower-level runner repeats the
  writable-overlap check but has neither the protected set nor an identity
  binding to the earlier check. This warrants integration as finding 1.
- Result-channel protection: the snapshot check includes inherited
  `RAVEL_EXIT_FILE`, but source acquisition protects only configuration,
  settings and cache paths. A source containing that known channel remains
  admissible. This warrants integration as finding 2.
- Inserted `ravel-skill-confinement-k69` as `integrate-review-impl`
  immediately after this review, before `ravel-local-handoff-k61`. It reads
  this review's handle and owns triage, fixes and verification. The parent
  still has live integration and handoff work, so retirement closes no
  parent node.

## Findings

All locations below refer to Ravel commit `e343587583eb`. Both cases are
derived from source, without an executed reproduction. P2 means medium
priority. Integration must triage them against the current feature tip.

### 1. P2 — Launch can grant a different tree from the one checked and probed

Location: `crates/keyed-launch/src/confinement.rs:211`, reached from
`command_with_grants` at line 200. The earlier protected-path check is
`crates/ravel/src/confinement.rs:298`.

Let `S` be a published snapshot tree and `P` the protected configuration
directory, separate from every writable root. `Confined::prepare` resolves
`S`, checks it against `P`, and probes it. During configuration evaluation,
another process of the account moves `S` aside and replaces its name with a
symlink to `P`. The managed launch still holds the path string `S`, not the
directory identity that passed validation. `read_trees` canonicalizes that
string again, obtaining `P`. It checks only that `P` is a directory other
than `/` and does not overlap a writable root. It then grants `P` to the
child: a Seatbelt subpath on macOS or a read-only bind at `P` on Linux.
Neither the protected configuration set nor the snapshot's identity reaches
this check. Reading the protected directory by its actual path is therefore
admitted, even if the original snapshot path in the descriptor no longer
works on Linux.

A harness run has the same final interval between its `prepare_for` and the
runner. Its preparation also resolves the original paths again and skips
the probe (`rhai_worker/harness.rs:273`). A redirect that already exists at
that preparation is caught by the protected check; a replacement after it
is not. The managed process's earlier probe supplies no identity binding
for either route. Even the probe canonicalizes its input again, at
`keyed-launch/src/confinement.rs:130`.

This case requires an external same-account cache replacement. The confined
child cannot perform that replacement through its ordinary grants, and the
cache is not an authentication boundary against its owner. The narrower
defect is that a changed cache path silently becomes a grant of an expressly
protected directory. The documented permission limitation does not explain
that widening. The launch should reject changed validated resources rather
than treat a newly resolved destination as the approved one. No fixture in
this diff places a replacement between grant validation and command creation.

### 2. P2 — A source containing the inherited result channel is still captured

Location: `crates/ravel/src/confinement.rs:298`. Related acquisition code:
`crates/ravel/src/skills/launch.rs:126` and
`crates/ravel/src/skills/mod.rs:344`.

Configure an ordinary local source `pack`, outside configuration, settings,
cache and the working directory. Supply an absolute inherited
`RAVEL_EXIT_FILE` beneath `pack/channel/`, with a quiet ordinary directory
there. For example, an enclosing session can have allocated its result
channel beneath a channel parent inside that source. The source can have
normal guidance alongside the channel; no symlink or concurrent edit is
necessary.

`launch::prepare` protects only the configuration entry's directory in
addition to `Protected::new`'s settings, default configuration and cache
paths. The inherited result path is absent from that set, so `origin` admits
`pack` and capture copies it. The new confinement check compares the
inherited channel only with the *retained snapshot* paths. Those are in the
cache, disjoint from `pack/channel/`, so this check also passes. Ordinary
channel contents present at capture can consequently enter retained skill
resources instead of causing the promised refusal. With delivery unconfined,
the new snapshot check does not run at all.

The owning contract at `docs/specs/skill-discovery.md:719` explicitly covers
both source boundaries and snapshot grants, and line 724 says that a cached
copy must not bypass an original protected boundary. This is an inherited
gap in acquisition that remains in the newly enabled confined route. It is
not a claim that copying a channel file alone transfers live completion
authority. The missing behaviour is the refusal of a source overlapping a
known result channel before its capture or reuse. The alias fixture at
`tests/skills_confinement.rs:441` tests a channel pointing into a snapshot;
it does not test a channel inside the original source.

## Disposition of the specific doubts

- **Repeated canonicalization and skipped harness probe:** Finding 1.
  Independent of protected-directory exposure, an ordinary same-path
  replacement can give a harness different resource bytes from the ones
  the managed preflight listed. A path string and a successful listing do
  not establish retained object identity.
- **Known channels and canonical aliases:** Finding 2 covers the missing
  source check. Under stable paths, the newly allocated managed and harness
  channels are covered structurally: `run.rs:476` canonicalizes
  `--exit-dir` before preparation, `confinement.rs:134` includes it and the
  private run directory in the writable set, and `skill_trees` checks both
  directions of overlap. `run.rs:113` and `rhai_worker/harness.rs:290`
  allocate each channel beneath one of those checked parents. There is no
  separate static omission for the later allocated harness channel or a
  stable symlink used as `--exit-dir`. Replacement after checking remains
  subject to finding 1's path-identity limitation.
- **The wider cache rule:** `confinement.rs:315` deliberately refuses a
  writable root inside, containing, or equal to the cache, including
  staging. A writable staging directory could alter a publication in
  progress; protecting only retained snapshots would not cover that
  producer. The updated owning specification at lines 731–732 records the
  reason. Ordinary project working directories outside the cache remain
  usable. A working directory that contains the cache would already
  contain the retained objects and violate the narrower rule. No additional
  defect was found in the stated widening.
- **Seatbelt writes, links and inherited descriptors:** The added tree
  rule at `keyed-launch/src/confinement.rs:334` grants reads and executable
  mapping, without a write allowance. With stable, non-overlapping grants,
  the existing default deny still applies to writes. The direct runner
  fixture at `tests/confinement.rs:169` uses writable `shared.md` and
  writable directories, so its write, link-target write, create, remove and
  rename failures are not all explained by snapshot modes. Its separate
  `locked.md` makes the chmod operation substantive. The producer also
  records manual hard-link, xattr, timestamp and flags checks. Those checks
  are historical evidence, not new measurements here. Both confined launch
  modes take the detached path; `keyed-launch/src/run.rs:1412` marks
  descriptors above stderr close-on-exec. Arbitrary inherited descriptors
  therefore do not survive to the backend. Standard output and error remain
  intentional output capabilities for the managed launch, and are pipes for
  a harness run. This review does not certify every macOS filesystem
  operation, clone or exchange syscall; it found no demonstrated extra
  write path in these rules.
- **What the probe proves:** `confinement_reads_available` at
  `keyed-launch/src/confinement.rs:129` lists directory entries under the
  platform's system reads and tree grants. It proves that listing succeeds
  for that invocation. It does not verify every file, future availability,
  or a later mount operation. The full launch additionally mounts writable
  roots and runtime files; under stable, validated disjoint paths those do
  not supply a concrete conflicting-directory counterexample. A later
  removal, replacement or backend resource failure can still make the real
  Linux bind fail. Both launch routes stay confined and have no unconfined
  retry. Finding 1 is the concrete unchecked change of grant destination,
  beyond the general limit of a preflight. The probe is not evidence that
  the real launch used the same object.
- **Linux nested delivery:** `docs/specs/skill-discovery.md:773` explicitly
  replaces the earlier unconditional refusal with the backend observations.
  The fixture at `tests/skills_confinement.rs:333` expects Linux to run a
  readable explicit entry with delivery disabled, while macOS refuses the
  unreadable settings. An absent settings file is already a valid disabled
  configuration; the inherited descriptor variable is not an instruction
  to enable delivery. The child remains within its parent's confinement,
  and the ordinary delegate can still reuse the retained descriptor. This
  is a documented qualification of the stronger earlier wording, not a
  demonstrated permission escape or a need for a second broker design.
  It must not be reported as successful fresh delivery by the nested child.
- **Denials and controls:** The public fixture's snapshot files are sealed
  to `0444` or `0555` by `skills/snapshot.rs:871`, and its directories have
  no write bit. Thus several `denied` results from the shell helper at
  `tests/skills_confinement.rs:37` alone are compatible with ordinary mode
  denial, not just sandbox denial. The chmod probe changes a real mode bit,
  and the direct runner fixture supplies writable subjects for the other
  operations. The producer records separate firing mutations for managed
  reads, harness reads, a writable Seatbelt tree, the protected check, the
  writable check, the probe, whole-cache exposure and the runner overlap
  check. It also states that the protected/writable multi-case fixtures
  stop at their first failing case, so later cases have no individual
  mutation evidence. Those controls support the tested boundaries, without
  establishing an independent firing control for every public probe or
  either finding above. No extra fixture defect is asserted merely because
  the public and runner tests divide this evidence.

## Verification limits

The producer records macOS checks, a Linux arm64 container run with
bubblewrap 0.11.1, and the subsequent Linux-only import repair and checks.
Those results belong to the producer; this review ran no verification
suite and adds no platform execution evidence. In particular, it does not
qualify amd64 Linux, every same-account attack, or arbitrary external
filesystem changes. Both findings require integration triage and executable
regression evidence. Changes from this session are limited to Grove's
review and task-tree artifacts.
