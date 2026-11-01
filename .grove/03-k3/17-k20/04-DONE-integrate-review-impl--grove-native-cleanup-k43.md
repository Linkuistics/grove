# grove-native-cleanup-k43

**Integrates:** grove-native-cleanup-k42

## Goal

Triage the cleanup review and apply the accepted findings before dependent work
or publication of the optional manual procedure.

## Context

Read the findings from grove-native-cleanup-k42 in its own commit. The producer
is grove-native-cleanup-k33. The delivery specification's Existing installations
and active readers section owns the preservation contract. Consume the review's
source locations and evidence limits rather than treating its static scenarios
as executed verification.

## Done when

- Classify each finding as a contract stated unclearly, a real issue, a visible
  trade-off, or noise, and record the reason.
- Apply accepted findings to the guidance that owns them. Keep optional cleanup
  conservative when ownership or retained dependencies cannot be established.
- Exercise the final procedure in disposable layouts appropriate to the accepted
  findings. Record exact inputs, commands, observations and remaining platform
  limits. Preserve the owner's real installations.
- Run applicable documentation and conformance checks after the final edits.
  Reconcile the producer's earlier verification claims with the resulting text.

## Notes

This integration owns fixes and their verification. Add no automatic cleanup,
cleanup CLI, owner-configuration change or release. Preserve this session's
inherited ending executable and command. If substantive changes earn a further
review under the Grove rules, cut it before dependent work.

## Decisions (running log)

- The artifact is `docs/USAGE.md`, subsection `usage-codex-cleanup`, at producer
  commit `de9e634b226e` and unchanged since. The contract is the delivery
  specification's section Existing installations and active readers. Each
  finding was checked against that text.
- Finding 1 is a real issue. The `find` excludes all of `~/.agents`, so a nested
  link in a kept entry is not reported. The `grep` names only `.agents/`.
- Finding 2 is a real issue. Neither `case` predicate matches a target that is
  an ancestor of the agents directory, the Codex home or the Codex skill
  directory.
- Finding 3 is a real issue. `find` does not traverse a start point that is a
  link, and `2>/dev/null` hides each directory that the search cannot read.
- Finding 4 is a real issue. The rule for `current` checks only links that stay.
- Finding 5 is a real issue. The rule for a `link-XXXXXX` directory does not
  account for its other children or for recursive discovery.
- A related real issue was found in this triage. GNU `realpath` prints a
  missing last component without an error. The guide's statement that
  `realpath` reports a missing Codex skill directory holds only on macOS.
- No finding needs a redesign. All fixes stay inside the section and keep
  cleanup conservative. An ancestor alias with unknown readers keeps every
  entry. This is a visible trade-off of optional cleanup.
- The full check passed on the first revision. This session then spent its one
  narrow in-session reviewer on the revised section and its contract. It
  reported 13 findings. Each was checked against the source or by a direct
  run, and each is a real issue:
  - H1. The `find` output shows only the final real path. A link through
    `current` or through a skill link reads as a direct snapshot link, so the
    entries between them look unneeded.
  - H2. A link to the Grove root or to `snapshots` is not an alias by the
    definition. Its readers are never searched.
  - H3. `grep -r` does not follow a link below its start directories. This was
    reproduced with macOS `grep`: a linked settings file was not reported.
  - M1. The search block uses `codex_skills` from the inventory block. In a new
    shell, the Codex skill directory drops out of the search, and step 4 moves
    `/NAME`.
  - M2. Each pattern ends with `/`, so `"$HOME/.agents"` is not found. Reproduced.
  - M3. A link that `realpath` cannot resolve is dropped without a message.
  - M4. `-xdev` skips other file systems below the roots without a message.
  - L1. The old writer joined a relative `HOME` or `CODEX_HOME` to its working
    directory (`provision.rs` lines 34 to 36).
  - L2. The manual installer also writes `~/.pi/agent/extensions`.
  - L3. The old ownership test accepted only targets below `current/plugins` or
    `snapshots`, or a Grove plugin skill shape. Other entries stopped startup.
  - L4. The old writer never created a link in a separate Codex skill directory.
    It only repointed existing links, so the first link condition fails there.
    Step 4 still moved one.
  - L5. No command checks that a snapshot holds only directories and regular
    files.
  - L6. `--exclude-dir=.grove` skips every `.grove` directory.
- These fixes are substantive and follow the one reviewer. This is the second
  need. This session applies them to a reviewable boundary and exercises them.
  A new `review-impl` leaf then reads the result before publication.
- `provision.rs` is byte-identical at `92a8bbee0037` and `175e5c7d2598-`, and
  no other commit touched it. So the layout claims hold for every release from
  21.11 to 22.8.
- Second rewrite: the search starts at real paths and enters other file
  systems. It skips only the Grove root and shows the text and the real path of
  each link. It marks an unresolved link whose text names the layout. The text
  search uses `find -L` with patterns without a slash. The user adds each
  reported link to the patterns. Links in a separate Codex skill directory stay.
  A snapshot check lists entries that are not directories or regular files.
- grove-native-cleanup-k44 (`review-impl`) now reads the second rewrite. It is
  placed before grove-skill-mandate-k22, which moved to position 06. The second
  rewrite has had no independent read.

## Verification record

Base `c739c996348`. Scripts and outputs are in this session's scratchpad `ex/`.

### Method

`build.sh` builds disposable homes by hand in the exact provisioner form. Each
home has five absolute links
`$HOME/.agents/.grove/current/plugins/grove/skills/NAME`, one snapshot
`bundle-AAAAAA` and `current`. The removed writer does not run on Linux. The
producer's exercise already showed this form against the real 22.8.0 writer.

`blocks.sh` extracts each `sh` block of the section verbatim. Three block sets
were run:

- `old`: producer commit `de9e634b226e`, the control for round one.
- `r1`: this session's first rewrite, the control for round two.
- `r2`: the final text.

`run.sh` runs the inventory, search and text blocks under `/bin/sh` in each
home through `env -i`, with `PATH=/usr/bin:/bin`. S10 runs only the search
blocks, in a new shell. The only owner edit adds the reported link path to the
text patterns, in S2 and in S8 for `r2`. `steps.sh` runs steps 4 to 6 in S4 and
S5 by the final rules. It runs the earlier rules as controls in a separately
built set of homes. Step 6 runs only when the root holds only `install.lock` and
an empty `snapshots`.

Scenarios:

- S1, review finding 1: a nested link to the snapshot in a kept real directory.
  Texts naming `~/.codex/skills/grove-impl` and `$CODEX_HOME/skills/...`.
- S2, finding 2: `~/home-alias` links to the home. A launcher sets `HOME` to it.
- S3, finding 3: `HOME` links to `storage`. `storage/work/dep` links to a skill
  link. `CODEX_HOME` is a link to an external Codex home that holds
  `tools/art`, a link to a skill link. `storage/locked` has mode 000.
- S4, finding 4: a settings file names a path below `current`.
- S5, finding 5: `link-AbC123` holds a staged link and a foreign `diag.txt`.
  `link-XyZ789` holds only its link.
- S6, Linux `realpath`: `~/.codex` exists without `skills`.
- S7, H1: `~/bin/grove-current -> ../.agents/.grove/current` and
  `~/bin/grove-skill`, a link to a skill link.
- S8, H2: `~/grove-cache` links to the Grove root. A settings file names a path
  through it.
- S9, H3 and M2: `~/.config/tool/linked.toml` links to a file in `~/dotfiles`
  that names `~/.agents/skills/grove`. `env.sh` holds `A="$HOME/.agents"`.
- S10, M1: `~/cs` links to a separate Codex skill directory.
- S11, M3: a link into a self-loop and a dangling link, both naming `.agents`.
- S12, M4, Linux only: a tmpfs mounted at `~/mnt` holds a link to a skill link.
- S13, L5: a snapshot holds a link to `/etc`.

### Platforms

- macOS 27.0.1: BSD `find` and `grep`, macOS `realpath`, the system `/bin/sh`.
- Linux: two disposable TestAnyware clones of
  `testanyware-golden-linux-24.04`, kernel 6.17.0-19, user `admin`, not root.
  `/bin/sh` is dash. GNU `realpath` 9.4, `find` 4.9.0 and `grep` 3.11. Each
  clone was stopped after its run.

### Round one

The guide digest was `6e70c4730d83683a` before and after the runs. On both
platforms, the `r1` blocks found every item in S1 to S6. The `old` blocks found
none, except the S4 text. Finding 4 is a rule defect, not a search defect.

- S4: by the revised rule, `current` and the snapshot stayed. The settings path
  stayed readable after the hold was removed. Step 6 kept the root. Control: the
  old rule moved `current`, and the settings path broke.
- S5: `ls -A` showed `link` in `link-XyZ789` and `diag.txt link` in
  `link-AbC123`. Only `link-XyZ789` moved, and `diag.txt` stayed. Control: the
  old rule moved `link-AbC123`, and `diag.txt` was lost with the hold.
- S6: on Linux, the old block printed `.../.codex/skills` with no error. The new
  block printed `.../.codex/skills: no directory` on both platforms.

The full check then passed on the round-one text: all 20 principal checks,
1,323 nextest tests and 5 books. The digest of the documentation change was
`b54b43e892003593` before and after that run.

### Round two

The macOS run used guide digest `fd7c01cd4032137b`. Then one paragraph about
unreadable files changed. The blocks extracted after that edit were identical to
the exercised `r2` set. The Linux run used digest `dd62f82e7f38f6a5`. Later,
sentence splits for the linter changed prose only. The blocks were again
identical.

Per-item results, the same on both platforms except where stated:

| Item | r2 | r1 |
|---|---|---|
| S1 to S6, every round-one item | found | found |
| S7 link text naming `current` shown | found | missing |
| S7 link text naming a skill link shown | found | missing |
| S8 alias to the Grove root | found | found |
| S8 text that uses that alias | found | missing |
| S9 linked settings file | found | missing |
| S9 `"$HOME/.agents"` without a slash | found | missing |
| S10 Codex skill directory alias, search block alone | found | missing |
| S11 link into a loop | found | missing |
| S11 dangling link naming `.agents` | found | missing on macOS, found on Linux |
| S12 link on another file system | found | missing |

- S7 output: `bin/grove-current -> ../.agents/.grove/current (real path
  .../snapshots/bundle-AAAAAA)`. The text names `current`.
- S11 output: both links end with `(unresolved)`. GNU `realpath` resolves a
  dangling link, so `r1` found it on Linux only.
- S12: `stat` showed device 64769 for the home and 69 for `~/mnt`.
- S4 by the final rules: the agents links moved. The Codex links, `current` and
  the snapshot stayed, and each stayed readable. Step 6 kept the root. The
  controls broke as in round one.
- S13: the snapshot check printed `.../extra`. The text search followed that
  link into `/etc` and reported its unreadable files on standard error.
- Other standard error: only S3's `locked` directory.

The owner's real `~/.agents/.grove`, `~/.agents/skills` and `~/.codex/skills`
are unchanged. The producer's digest script gave `99a2418fc76a7ecb` before the
first run and after the last, the producer's recorded value.

### Final checks

- Direct `ste-lint.py` on the final section: 0 hard, 0 advisory, 1 accepted.
  Control: one added semicolon in a copy gave 1 hard finding.
- `GROVE_TEST_RAVEL=<Ravel 1.1.0> mise exec bun@1.4.3 -- bash scripts/check.sh`
  on the complete change, after retirement: all 20 principal checks, 1,323
  nextest tests and 5 books pass. The digest of the whole working-copy diff was
  `951ee9859113647d` before and after the run. This section was written after
  that run.

### Remaining limits

- No real harness session ran in the disposable homes. Readable `SKILL.md`
  files through each kept path replace that step.
- The removed writer did not run in this exercise. The layouts were built by
  hand in its exact form.
- The release comparison was not run again. Only the new `find` line of that
  block ran, in S13.
- No real macOS home with protected `~/Library` directories was searched.
- Other Unix systems, busybox tools and other shells were not exercised.
- The second rewrite has had no independent read. grove-native-cleanup-k44 owns
  that read.

### Reconciliation with the producer's record

- "The reference search now reports links to `~/.agents`, into it, and to or
  into the Codex skill directory" was incomplete. Review findings 1 to 3 and
  H1 to M4 name the missing classes, and the final search covers each one.
- "`realpath` reports a missing Codex skill directory" held only on macOS. The
  inventory now tests the directory itself.
- Layout A moved Codex skill directory links. By the final rules, those links
  stay, because the writer made no new links there.
- The producer's other layout results keep their meaning for the classes that
  they covered.
- The producer's macOS-only limit is replaced by the Linux runs above.
