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
