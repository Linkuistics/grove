# grove-native-cleanup-k45

**Integrates:** grove-native-cleanup-k44

## Goal

Triage the new cleanup review and settle the optional manual procedure before
dependent work or publication.

## Context

Read grove-native-cleanup-k44 in its committed form. Its reviewed artifact is
the earlier Codex installations section in `docs/USAGE.md` at `c22acbde`.
The delivery specification's Existing installations and active readers section
owns the preservation contract. Consume the review's source evidence and limits;
its counterexamples were not executed. Earlier disposable exercises are recorded
on grove-native-cleanup-k43.

## Done when

- Classify each finding as a contract stated unclearly, a real issue, a visible
  trade-off or noise, and record the reason.
- Apply accepted changes to the guidance that owns them. Keep foreign,
  uncertain and still-needed entries safe. Settle what can establish an
  entry's removal before expanding the search further.
- Exercise the resulting procedure in disposable layouts appropriate to the
  accepted findings. Record inputs, commands, observations and remaining
  limits. Preserve the owner's real installations.
- Run applicable checks after the final edits and reconcile earlier claims
  with the final procedure. Apply the leaf-wide review allowance if further
  substantive doubt remains.

## Notes

This integration owns all fixes and post-fix verification. Add no automatic
cleanup, cleanup CLI, owner-configuration change or release. Preserve the
inherited ending executable and command.

## Decisions (running log)

- The artifact is `docs/USAGE.md:694-1001` at `c22acbde`, unchanged at this
  session's base `e1aed143`. The contract is the delivery specification's
  section Existing installations and active readers. Each finding was checked
  against the text and the old writer's recorded behaviour.
- The owner's real layout was read without change: 15 snapshots in 13 MB, and
  real snapshots hold `__pycache__/ste-lint.cpython-315.pyc` beside
  `ste-lint.py`. Removing `current` and the snapshots saves little space. The
  duplicate-skill effect comes from the skill links.
- Settled preservation rule, before any search change: no search proves that
  nothing needs an entry. A search reports only what it can reach in the
  locations given to it. Therefore `current` and every snapshot stay in three
  cases. A search shows a dependency other than a skill link of the inventory
  in an exact provisioner form. The Grove root holds anything that is not a
  provisioner entry. A snapshot is uncertain. When none of these is true, a
  link of the inventory that stays keeps only what its text names. This
  replaces manual chain following with a mechanical rule.
- Finding 1 is a real issue. The search selects links by final real path. The
  fix walks the text of each link with a `..` component, prefix by prefix.
  It shows the link when a prefix resolves into the Grove root. A text without
  `..` that enters the Grove root ends there, unless a link inside the root
  leads out. Such a link is foreign content that keeps everything.
  A link in a component is a link of its own, and its own line covers it.
- Finding 2 is a real issue. It is closed by the rule, not by a wider search.
  Any entry in the Grove root other than the provisioner's entries keeps
  `current` and every snapshot.
- Finding 3 is a real issue. The comparison no longer excludes `__pycache__`.
  A difference is accepted only as a `__pycache__` directory in the snapshot
  that holds only `STEM.*.pyc` files beside a `STEM.py`. Such a file is a
  bytecode cache of that source. A foreign file with exactly that name is
  indistinguishable from the cache. This is a visible trade-off.
- Finding 4 is a real issue. Each link that `realpath` cannot follow is now
  shown with the reason, whatever its text. Exceptions are a missing path, a
  component that is not a directory and a loop. Such a link fails for every
  reader now, and removal cannot break it further. This reverses the k43 M3
  choice to show dangling links that name `.agents`, by that argument. A
  missing path on a volume that is not mounted is a residual limit, so the
  guide tells the user to mount such volumes first.
- Finding 5 is a real issue. The holding directory is now new and unique, made
  by `mktemp -d`. No move can overwrite an entry, and step 6 removes only what
  the procedure moved.
- From the dispositions: the transcript sentence is a real issue. A resumable
  transcript is a dormant lazy reader. The Codex directory explanation was too
  broad. Its prescribed action stays. The traversal cost and the broad
  patterns are visible usability trade-offs, unchanged.
- Found in the exercise (N1b): when `~/.agents` is itself a link, the search
  shows it as an alias, and the strict rule would keep everything forever.
  That link is the layout's own path, through which the provisioner named each
  link. The guide now says to ignore the line of `~/.agents`, `~/.agents/skills`,
  the Codex home or the Codex skill directory when it is itself a link.
- The link-removal condition tests NAME as a whole path component, except in
  links of the inventory in the provisioner form. A substring test would block
  the link `grove` forever, because each provisioner text contains `.grove`.
- The spec section gains one sentence: a search cannot prove absence, so a
  dependency that the procedure cannot limit keeps the pointer and every
  snapshot. No ADR: this is a rule of one procedure, owned by the guide.
- The leaf-wide in-session reviewer is spent on the rewritten section, with
  the artifact, the contract and the old writer only.
- In-session reviewer result, each finding checked against the text:
  - L1 (case variant on case-insensitive APFS) is real. The component test
    and the case-sensitive text search miss `.../skills/GROVE`.
  - L2 (chain through a link outside the searched roots) is real. The
    argument that a component link has its own line holds only inside the
    roots. Fix: a line keeps only a known link when its text has an exact
    absolute form without `..`. Every other line, except an alias, keeps
    every link. This also closes L1 in the link search.
  - L3 is real. The exemption for a skill directory that is itself a link
    must apply only to an alias line. It must not apply to a link into the
    Grove root. The
    table also misdescribed a link to the Grove root itself.
  - L4 is real. `find` without `-L` does not enter a directory link that
    leaves the roots, and nothing names it. Fix: show such links as
    `(leaves the search, real path R)` and tell the user to search R too.
  - L5 is real for readers in another mount namespace. A broken link here can
    work in a container, a virtual machine or another host that mounts the
    layout. Such a mount is now an alias, and it keeps `current` and every
    snapshot. The silence for broken links stays, limited to same-path readers.
  - L6 is real. Alias patterns now use the path after the home directory,
    which also matches `~/` and `$HOME/` spellings.
  - L7 is real. A regular file named `__pycache__` and a free-form `.pyc`
    name passed. Fix: each `__pycache__` must be a directory, and each file
    must have the name `STEM.cpython-NN[N].pyc` or with `.opt-1`/`.opt-2`.
    A foreign file with exactly such a name stays a visible trade-off.
  - Usability items applied: `${hold:?}` guards and a destination for staged
    directories in the skill directory. Also applied: full restore in step 5,
    the `..?*` glob, the release tag and a relative or other `HOME`. Wording at the
    Grove-root condition clarified. Names with a newline stay a recorded limit.
- These fixes are substantive and follow the one reviewer, which is the
  second need. This session applies them to a reviewable boundary and
  exercises them. A new `review-impl` leaf reads the result before
  publication.

## Verification record

Base `e1aed143`. The scripts and outputs are in this session's scratchpad
`ex/`. `blocks.py` extracts each `sh` block of the section verbatim. `build.sh`
builds disposable homes by hand in the exact provisioner form. `run.sh` runs
the link and text blocks in one `/bin/sh` per home, through `env -i` with
`PATH=/usr/bin:/bin`. `steps.sh` runs steps 4 to 6 by the stated rules.

Block sets: `old` is `c22acbde`, the review's artifact. `draft` is this
session's first revision, read by the in-session reviewer. `new` is the final
text, digest `fa90c0b071c6fcd8`. The last edits after that digest changed
prose only, and the extracted blocks stayed identical.

### Platforms

- macOS 27.0.1, APFS case-insensitive: BSD `find` and `grep`, macOS
  `realpath`, the system `/bin/sh`.
- Linux: two disposable TestAnyware clones of `testanyware-golden-linux-24.04`,
  kernel 6.17.0-19, user `admin`, not root. `/bin/sh` is dash. GNU `realpath`
  9.4, `find` 4.9.0 and `grep` 3.11. Each clone was stopped after its run.

### Per-item results

The results were the same on both platforms, except where stated.

| Item | new | control |
|---|---|---|
| N0 benign relative `..` link | not shown | (old: not shown) |
| N1 `~/guide` through `current/../..` | passes | old: missing |
| N1 relative text from inside `~/.agents` | passes | old: shown, no chain |
| N1b `~/.agents` is a link, relative text | passes | old: shown, no chain |
| N2 `restore.sh` in a staged directory | kept by the rule, readable after step 6 | old rules: snapshot moved, read fails |
| N4 link to an unreadable directory | unresolved: Permission denied | old: missing |
| N4 missing path, non-directory, loop, missing below `current` | not shown | old: one shown |
| N5 existing `~/grove-codex-cleanup` | untouched | old: both owner files lost |
| R1 to R5 earlier classes | shown | shown |
| R6 snapshot with a link | printed by `find` | printed |
| R7 link into a foreign real skill directory | not shown, by design | old: shown |
| V1 `.../skills/GROVE` on APFS | shown, keeps every link | draft: component rule passed it |
| V1 on Linux ext4 | broken, not shown | expected: it fails for every reader |
| V1 `.Agents` in a setting | found by `grep -i` | draft: missing |
| V2 chain through an outside link | shown, keeps every link | draft: passed the link rule |
| V3 Codex skill directory linked into the root | real path line, keeps all | draft: ignored as an alias |
| V4 directory link leaving the roots | leaves the search. With R searched, the inner link shows | draft: nothing shown |
| V6 `$HOME/myskills` setting | found by `-e /myskills` | absolute pattern: missing |
| Release, genuine `__pycache__` | complete | complete |
| Foreign `__pycache__/notes.txt` | uncertain | old: complete |
| Orphan `gone.cpython-315.pyc` | uncertain | old: complete |
| V7 regular file `__pycache__` | uncertain | draft: complete |
| V7 `ste-lint.notes.pyc` | uncertain | draft: complete |
| `${hold:?}` with an empty `hold` | command stops, link stays | (not applicable) |

On Linux the guest fetched v22.8.0 with the guide's own `curl` line. The first
Linux snapshot run was invalid, because macOS `tar` added AppleDouble files to
the package. A clean package with no `._` entries gave the results above.

### Owner's real installation

Only reads touched it: `ls`, `du`, `find` and `diff`. `bundle-aZoobr`, the
current snapshot, equals v22.8.0 apart from one genuine bytecode cache. The
other 14 snapshots differ from v22.8.0. They came from checkout builds. The
listing digest was `056afd8158eab129` before the exercise.

### Remaining limits

- No real harness session ran in the disposable homes. Readable `SKILL.md`
  files through each kept path replace that step.
- The removed writer did not run. The layouts were built by hand in its form.
- Mount aliases in containers, virtual machines and other hosts were not
  exercised. The rule keeps everything when such a mount exists.
- A tmpfs on Linux (k43 S12) was not run again. The search still enters other
  file systems, because no `-xdev` exists.
- Names with a newline can split an output line. Busybox, musl and other
  shells were not exercised. An unknown `realpath` message is shown, which
  keeps entries.
- Dependencies through locations that the user does not search stay
  undetectable. The guide states this principle and the spec now owns it.

### Final checks

- Direct `ste-lint.py` on the final section: 0 hard, 0 advisory, 1 accepted.
  Control: a copy with one added semicolon gave hard findings.
- `GROVE_TEST_RAVEL=/opt/homebrew/Cellar/ravel/1.1.0/bin/ravel mise exec
  bun@1.4.3 -- bash scripts/check.sh` on the complete change, after
  retirement: all 20 principal checks, 1,323 nextest tests and 5 books pass.
  The digest of the whole working-copy diff was `bd02fa1ba0faf41d` before and
  after the run. This section was written after that run.

### Reconciliation with earlier records

- k43's M3 choice showed dangling links that name `.agents`. k45 drops all
  links that fail for every same-path reader, by the argument in the log.
- k43's statement that chain following settles which entries a link needs is
  replaced. The lexical line rules decide it, and `current` and the snapshots
  stay for any line outside the provisioner forms.
- k43's `-x __pycache__` comparison accepted foreign files. Its S13 result
  for links in a snapshot keeps its meaning.
- The second rewrite's other per-item results keep their meaning for R1 to R5.
- grove-native-cleanup-k46 reads this result before publication.
