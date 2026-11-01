# grove-native-cleanup-k47

**Integrates:** grove-native-cleanup-k46

## Goal

Triage the committed cleanup review and settle the optional manual procedure
before dependent work or publication.

## Context

Read grove-native-cleanup-k46 in its committed form. Its artifact is the
earlier Codex installations section of `docs/USAGE.md` at `be750dee`.
The delivery specification's Existing installations and active readers
section owns the preservation contract. Consume the review's evidence and
limits; its counterexamples were not executed. k45 holds the preceding
disposable exercises and check record.

## Done when

- Classify each finding as an unclear contract, a real issue, a visible
  trade-off or noise, and record the reason.
- Settle the cleanup scope and preservation rule before changing the search.
  Apply accepted changes to the owning guidance. Preserve foreign, uncertain
  and still-needed entries.
- Exercise the resulting procedure in suitable disposable layouts and record
  inputs, commands, observations and limits. Preserve the real installations.
- Run applicable checks on the final change and reconcile earlier evidence
  with the resulting procedure. Apply the leaf-wide review allowance if
  substantive doubt remains.

## Notes

This integration owns fixes and post-fix verification. Add no automatic
cleanup, cleanup CLI, owner-configuration change or release. Preserve the
inherited ending executable and command.

## Decisions (running log)

- The artifact is `docs/USAGE.md:694-1137` at `be750dee`, unchanged at this
  session's base `82b24f7d`. The contract is the delivery specification's
  section Existing installations and active readers. Each finding was checked
  against the guide text and the old writer at `175e5c7d259-`.
- Settled scope, before any search change: the optional cleanup removes only
  owned links and `link-XXXXXX` staging directories in the agents skill
  directory. It keeps the whole Grove root, with `current`, every snapshot and
  `install.lock`. Reasons: the duplicate-skill effect, the one stated benefit
  beyond space, comes from the links. The root held 13 MB in the owner's
  installation. Findings 1 to 3 of k46, and k43/k44's earlier snapshot classes,
  all arise from proving that nothing needs an entry in the root. No search can
  prove that, as the contract already says. The contract permits but does not
  require snapshot removal. This narrows deletion authority; it adds none.
- Settled preservation rule for the remaining entries: a link inside the
  searched locations needs a skill-directory entry when resolving its text
  visits that entry. The search follows each link component by component,
  through every intermediate link wherever it lives, and compares directories
  by device and inode (`test -ef`). It reports each entry visited, a directory
  it cannot search, a directory reached outside the searched locations, and
  an alias of a skill directory. Each report keeps only the entry it names,
  except an unsearchable directory or an unsearched outside location, which
  keep every entry. This replaces final-real-path classification, the `..`
  prefix walk, the alias exemption and the message-substring filter.
- Finding 1 (exempted alias needs an old snapshot) is a real issue. The
  narrowed scope closes it: no root entry is removed, so a path through any
  snapshot keeps working. The alias exemption is gone. An alias now keeps no
  entry by itself, and its readers fall under the reader-need condition.
- Finding 2 (leave-and-return through an external link) is a real issue,
  reproduced: k45's search printed no line for `H/document` or
  `H/document-dir` in disposable homes on both platforms. For the root, the
  narrowed scope closes it. For skill links, the component walk follows the
  outside bridge and reports `uses H/.agents/skills/s1`.
- Finding 3 (lock removal by name) is a real issue. The narrowed scope closes
  it: `install.lock` and the whole root stay, and step 6 removes only the
  holding directory.
- Finding 4 (diagnostic substrings) is a real issue, reproduced: k45's search
  printed nothing for `H/No such file or directory`. The walk parses no
  message. It tests `-d` and `-x` on each directory and reports
  `unknown at DIR` for an unsearchable one.
- The practical-cleanup disposition is accepted as the scope change above.
  The `leads out to R` rule still keeps every entry until R is searched. That
  stays a disclosed cost, as in k45 (its L4).
- The spec section now states the narrowed scope and the intermediate-link
  rule. The test-seam row names intermediate links and the unchanged root.
  Both READMEs say that cleanup removes the links and keeps the root. No ADR:
  this is a rule of one optional procedure, owned by the guide and its spec.
- The leaf-wide in-session reviewer is spent on the narrowed section, with the
  artifact, the contract and the old writer only. It ran the guide's blocks in
  disposable homes on macOS. Its findings, each checked:
  - L1 (case-variant alias silent) is real, reproduced. bash 3.2 `pwd -P`
    keeps the link's spelling, and `reaches`/`leads out` compared strings.
    Fix: both tests compare each ancestor directory with `-ef`. `cd -P` is gone.
  - L2 (firmlink alias silent after its R is added) is real, reproduced.
    The same `-ef` fix closes it. A Linux bind mount is derived, not run.
  - L3 (target ending in a newline) is real, reproduced. `readlink` reads keep
    a sentinel, which resolves it exactly with GNU `readlink`. macOS
    `readlink` prints such a target without its own newline, so no sentinel
    recovers it. Fix for that and every other walk/kernel disagreement: at a
    silent stop (missing path, non-directory, hop limit), the walk asks the
    kernel with `[ -e "$link" ]`. When the kernel resolves the link, the
    search prints `unknown at P`. Silence now means the kernel also fails.
  - The staging-directory row was wrong: the old writer's `TempDir` removes
    the directory on every error return, so only a killed startup leaves
    one. The row now says so.
  - Wording: the device-and-inode claim is now true for every line. The
    internal handle `delete-provisioning-k19` became "earlier development
    builds". A long line was wrapped.
  - Usability items applied: `sh` before the inventory, because zsh stops
    the loop at an unmatched pattern. The text search omits `-e ""` when
    `~/.agents` is missing, and the guide says that nothing is then removable.
    Step 5 gives the restore command.
  - Visible trade-offs, unchanged: a text that names a skill by a relative
    path, a file link to a script outside the searched locations, and
    `reaches` lines for links to `/` or `~`. The guide already tells the user
    to add script and setting locations to the searches.
- No further review leaf. The narrowed design had one fresh read. Every loss
  it found has an executed reproduction, a fix and a control that fails, on
  both platforms. The kernel cross-check bounds the walk's remaining error to
  reports, not silence, for every link that a same-path reader can resolve.
  Residual doubt is usability, which a review cannot settle. Publication
  happens only through the finish session.

## Verification record

Base `82b24f7d`. Scripts and outputs are in this session's scratchpad `ex/`.
`build.sh` (dense home), `build2.sh` (ordinary home) and `build3.sh`
(reviewer cases) build disposable homes by hand in the exact provisioner
form. The guide's blocks were extracted verbatim from `docs/USAGE.md` and run
with `env -i HOME=<home> PATH=/usr/bin:/bin /bin/sh`. `steps.sh` runs steps
4 to 6 and digests the Grove root (modes, link texts, file bytes) before and
after. Final extracted block digest: `2386ef07b9b69f48` (shasum of the link
and text blocks), the same on both hosts.

Platforms: macOS 27.0.1, APFS case-insensitive, `/bin/sh` bash 3.2.57, BSD
`find`. Linux: disposable TestAnyware clones of
`testanyware-golden-linux-24.04`, kernel 6.17.0-19, user `admin`, not root,
`/bin/sh` dash, GNU `realpath` 9.4 and `find` 4.9.0. Each clone was stopped.

| Item | final | control |
|---|---|---|
| k46-2 `document` file link through outside bridge | uses s1 | k45 block: no line |
| k46-2 `document-dir` directory variant | uses s1 | k45 block: no line |
| k46-4 `No such file or directory` link, unreadable parent | unknown at E/private | k45 block: no line |
| `via-ag` relative text through an intermediate link | uses s2 | walk without splicing: no line |
| user links `myguide`, `rel`, root `foreign`, `staged-user` | uses the named entry | (positive cases) |
| `case-variant` text `.AGENTS/SKILLS/S1` | macOS: uses; Linux: broken, silent | (expected) |
| aliases `myskills`, `ag` | reaches | (positive cases) |
| `ext` outside directory | leads out to E/dir | (positive case) |
| loop, dangling, non-directory, file link, inventory links | silent | (expected) |
| R-L1 `myskills -> .AGENTS/skills` | macOS: reaches | string walk: no line |
| R-L1b `HOMEUP` (home in upper case) | macOS: reaches | string walk: leads out only |
| R-L2 firmlink `/System/Volumes/Data$H/.agents/skills` | macOS: reaches | string walk: leads out only |
| R-L3 target ending in a newline | macOS: unknown; Linux: uses s1 | without cross-check: no line, both |
| ordinary home, both searches | no output | (expected) |
| steps 4 to 6 in the ordinary home | three entries held and removed, Codex link reads, root digest unchanged | (expected) |
| empty `hold` | `${hold:?}` stops `mv`, link intact | (expected) |
| text search with a setting naming `~/.agents/skills/s1` | file shown | (positive case) |

### Remaining limits

- No real harness session ran in the disposable homes. Readable `SKILL.md`
  files through kept paths replace that step.
- The removed writer did not run. The layouts were built by hand in its form.
- A Linux bind mount, mount aliases in containers or virtual machines, busybox
  and other shells were not exercised.
- On macOS a target ending in a newline gives `unknown`, not the exact entry.
  If a sibling named without the newline also exists, the walk follows that
  sibling. This needs both names and is not handled.
- Output lines for names that contain a newline can split.
- Dependencies through locations that the user does not search, and texts
  that the patterns do not match, stay undetectable. The spec states this.
- Direct `ste-lint.py` on the final section: 0 hard, 1 advisory (`was killed`,
  actor unknown), 1 accepted. Control: an added semicolon gave a hard finding.

### Reconciliation with earlier records

- k45's `real path`, `alias`, `passes`, `unresolved` and `leaves the search`
  lines, its six line rules, the alias exemption and its keep-everything
  conditions are replaced by the walk's four line kinds and rules.
- k45's snapshot completeness check, `__pycache__` rules, root-content
  condition and root removal are gone with the narrowed scope. Their per-item
  results (R6, R7, V7, the release comparison) no longer describe the guide.
- k45's N4 argument (a link that fails for every same-path reader cannot be
  broken further) stands. The walk now establishes the failure with the
  kernel, not with message text.
- k45's holding directory, `${hold:?}` guards, Codex-directory rule,
  transcript rule and search-extension guidance stand unchanged.
- k43's chain-following idea returns in mechanical form: the walk follows
  every link, and the user follows none by hand.

### Final checks

- `GROVE_TEST_RAVEL=/opt/homebrew/Cellar/ravel/1.1.0/bin/ravel mise exec
  bun@1.4.3 -- bash scripts/check.sh` on the complete change, after
  retirement: all 20 principal checks, 1,323 nextest tests (21 skipped) and
  5 books pass. The digest of the whole working-copy diff was
  `5cc9db2614344ca5` before and after the run. This section was written
  after that run.
