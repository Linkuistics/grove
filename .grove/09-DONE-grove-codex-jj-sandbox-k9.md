# grove-codex-jj-sandbox-k9

**Kind:** work

## Goal

Establish whether grove's codex launch path needs a jj analogue of the
gitdir grant (`--add-dir`, ADR `codex-gitdir-grant`), and adjust
`launch.rs` if so. Execution in Linkuistics/grove (`~/Development/grove`);
this leaf only tracks.

## Context

- The existing grant reopens the git common dir because codex's
  workspace-write sandbox carves `.git` out of every writable root
  (per-rule `require-not`, verified against codex-cli 0.144.5 in the ADR).
- Open questions to verify by probe (the ADR's method: live `codex exec`
  probes in scratch repos, plus codex source if needed):
  - jj-native tree: is `.jj/` carved out too, or writable by default? If
    writable, no grant is needed — confirm and record.
  - Colocated tree: jj writes both `.jj/` and (on export) `.git/`. Does the
    existing gitdir grant suffice once `repo.rs` is jj-first
    (`grove-jj-plumbing-k8`)? The grant derivation currently shells
    `git rev-parse --git-common-dir` — fine in colocated, impossible in
    jj-native.
  - Secondary jj workspace: `.jj/` there is a pointer to the main
    workspace's repo store — writes land in the main repo's `.jj/repo`,
    outside the sandbox cwd. That likely NEEDS a grant analogous to the
    linked-worktree case.
- Depends on `grove-jj-plumbing-k8` (repo resolution must exist first).

## Done when

- Each of the three probes above has a verified answer (probe transcript
  or codex source citation — the codex-gitdir-grant ADR sets the evidence
  bar).
- `launch.rs` grants whatever the probes prove necessary; the
  `codex-gitdir-grant` ADR is reworked in place to cover jj (no
  superseding ADR).
- One focused commit in Linkuistics/grove; this leaf retired with a
  bookkeeping commit here naming `grove-codex-jj-sandbox-k9`.

## Notes

If the probes prove no change is needed, the ADR note recording that fact
is still the deliverable — absence of a needed grant is a finding.
