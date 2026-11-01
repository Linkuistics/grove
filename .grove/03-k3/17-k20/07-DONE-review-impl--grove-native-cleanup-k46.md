# grove-native-cleanup-k46

**Reviews:** grove-native-cleanup-k45

## Goal

Read the optional manual cleanup procedure adversarially again, after the
integration of grove-native-cleanup-k44. Find any case in which a user who
follows it literally loses a foreign, uncertain or still-needed entry.

## Context

The artifact is the section of `docs/USAGE.md` from the anchor
`usage-earlier-codex-installations` to the anchor `usage-tree-verbs`, as the
commit of grove-native-cleanup-k45 left it. The contract is the delivery
specification's section *Existing installations and active readers*, which
k45 extended by one sentence. The ground truth for the old writer is
`crates/grove/src/provision.rs` at `175e5c7d259-`. The manual installer is
`plugins/install.sh`.

k45 settled a preservation rule before it changed the search. It replaced
manual chain following with lexical rules. Each line of the link search keeps
links by a fixed list. Any line outside the provisioner forms keeps `current`
and every snapshot.

Its one in-session reviewer read the first
k45 revision and reported seven loss findings. All were applied, and the
result has had no independent read. The k45 verification record holds the
scenarios N0 to N5, R1 to R7 and V1 to V7, the platforms and the limits.
Treat them as the cases that the author already thought of.

## Done when

- The specific doubts below each have a finding or an argued dismissal.
- Other findings are reported with a concrete scenario and evidence.
- No fix is applied. Findings worth acting on get an
  `integrate-review-impl` leaf, as the review family directs.

## Notes

These doubts remain after k45:

- Whether the six line rules and the keep-everything conditions together
  prevent every loss inside the searched locations. Look for a line whose
  text has an exact provisioner or skill-link form but whose chain still
  passes another entry.
- Whether `(leaves the search, real path R)` and the instruction to search R
  close the class of dependencies through links outside the roots. Consider
  file links, links that leave and return, and an R too large to search.
- Whether silence for a missing path, a non-directory and a loop is safe
  beyond the stated mount-namespace limit.
- Whether the exemption for `~/.agents` and the skill directories when they
  are links can hide a real alias.
- Whether the strict rules now make cleanup impossible in an ordinary home,
  and whether a narrower procedure would serve the contract better. Examples
  are removing only skill links, or keeping the whole Grove root, which held
  13 MB in one real installation.

## Decisions (running log)

- The reviewed artifact is `docs/USAGE.md:694-1137` at `be750dee`, the
  grove-native-cleanup-k45 commit. The working copy initially had no changes.
  The delivery specification, the historical provisioner at `175e5c7d259-`,
  the installer and k45's verification record were read directly. This is an
  inspection-only review. No scenario, test, build, lint, format, installer or
  cleanup command is run here.
- Four issues warrant integration: an alias exemption that hides intermediate
  dependencies, external intermediate links that the search never reports,
  deletion of a lock file without establishing its ownership, and diagnostic
  matching that can suppress a permission failure. These are static
  counterexamples, not exercised results.
- Keeping every entry when an external search is impractical is conservative.
  It can defeat optional cleanup in an ordinary home. A narrower procedure
  that leaves the entire Grove root would avoid the snapshot-dependency
  question, but its adoption belongs to integration.
- grove-native-cleanup-k47 is inserted before grove-skill-mandate-k22, so
  integration runs before other work can move the cited text. The parent
  remains live with those two tasks; no parent closes here.

## Findings

Locations below refer to `docs/USAGE.md` at `be750dee`. Each issue is medium
priority and should be resolved before publication. No fix is applied here.

### 1. An exempted directory alias can still need an old snapshot

Location: lines 825-827 and 862-865; snapshot retention at 1066-1076.

Let H denote the absolute home directory. Suppose two complete owned snapshots
exist, and `current` has the exact provisioner text naming bundle-NEW. After
provisioning, the owner makes this Codex skill-directory alias:

```text
H/.codex/skills -> H/.agents/.grove/snapshots/bundle-OLD/../../../../shared-skills
```

Its final real path is `H/shared-skills`. The directory can contain a foreign
regular resource, accessed through `H/.codex/skills/resource.txt`, without any
old skill being needed. There are no other foreign links or references.

The search prints an alias line because the final real path equals
`codex_real`. `show` exits before the `..` prefix walk. The explicit exemption
then removes that line from every retention rule. The remaining provisioner
links keep at most bundle-NEW, so bundle-OLD is classified as removable.
Removing it breaks the retained alias and access to the foreign resource.
Checking that the new harness sessions load their required skills need not
exercise that resource path.

The same issue applies to `~/.agents` itself. For example, after relocation,
`H/.agents -> /data/agents/.grove/snapshots/bundle-OLD/../../..` resolves to
`/data/agents`. Every skill link can still have exact provisioner text through
`H/.agents`, yet its chain also needs bundle-OLD before reaching `current`.
An exact text form therefore does not establish the complete dependency set
when a layout ancestor is an exempted alias. The exemption must not discard
the alias's own intermediate dependencies.

### 2. A link that leaves and returns can conceal an external intermediate link

Location: lines 828-845; instructions to search R at 867-871.

Use an otherwise ordinary home with genuine bundle-OLD and bundle-NEW, no
foreign entry inside the Grove root, and `current` naming bundle-NEW. Consider:

```text
H/document -> /srv/bridge/readme.txt
/srv/bridge -> H/.agents/.grove/snapshots/bundle-OLD/../../../../archive
H/archive/readme.txt is an existing regular file.
```

The home link is inside the requested search. Its final real path is
`H/archive/readme.txt`, outside the Grove root. Its own stored text has no
`..`, so the prefix walk does not run. The file test at line 841 then exits
without printing anything. `/srv/bridge` is outside the physical search roots
and is never visited. Thus there is no reported R to add and no line to
activate the keep-everything condition. Removing bundle-OLD breaks the
foreign document link although the document's real file remains.

Changing `document` to a link to `/srv/bridge/subdirectory` gives the directory
case too: its final path is inside the searched home, so lines 842-843 silently
exit. Searching the final R cannot in general inspect links in the path that
led to R. The missing intermediate link is the dependency, even if R itself
is already searched. The separate text search does not find symlink contents
as text; a setting that says only `H/document` need not match its patterns.

This is a missed dependency of a link inside the requested search, rather than
merely an undeclared location that the owner forgot to search. The procedure
cannot infer safety from the final real path and the outer link text alone.

### 3. The final lock removal has only a name check

Location: lines 1060-1061, 1074-1076 and 1126-1132.

Suppose the installation otherwise meets all removal conditions, but
`install.lock` is a nonempty regular file containing an owner's retained data.
It can predate the provisioner: the historical `lock` function opens an
existing regular file with `create(true)` and `truncate(false)`, and rejects
neither nonempty contents nor foreign provenance. Provisioning can complete
normally without ever owning those bytes.

Both searches prune the Grove root. The root-content condition exempts
`install.lock` by name, and no later condition establishes that this file is
the empty owned lock from the layout table. After the other entries move,
step 6 says to run `rm` whenever the root holds only `install.lock` and an
empty `snapshots`. It deletes the foreign contents directly, outside the
holding directory and its restore procedure.

The general instruction to keep foreign entries and this specific name-only
removal instruction need a consistent ownership rule. A successful old
startup, the expected filename and the absence of other children do not
establish that the file is owned and disposable.

### 4. Error-message substrings can hide an unreadable chain

Location: lines 816-821.

The error classification searches the entire diagnostic, including the path
that `realpath` quotes. Consider a foreign link with the legal name
`H/No such file or directory`, pointing to `/srv/private/bridge`. The external
parent is not searchable by this user, and the bridge ultimately references
bundle-OLD. Its dependency is unknown to the cleanup user.

`realpath` fails with Permission denied, but its diagnostic also contains the
input path `H/No such file or directory`. The first substring pattern matches
the filename and silently exits. The physical home search can inspect the
link itself without entering the private directory, so it need not print a
separate permission diagnostic. The text search's default roots do not enter
this home-level link either. There is no unresolved line to keep entries.

This defeats the stated preservation of unknown chains and the assurance that
unreadable locations are reported. A filename containing `Not a directory`
or `Too many levels of symbolic links` has the same problem. The decision to
silence genuine missing-path, non-directory and loop failures must not be
based on an unstructured substring that can come from the filename.

## Disposition of the mandated doubts

- **Six line rules and exact provisioner forms:** finding 1 supplies a chain
  whose exact provisioner text still passes another snapshot. The ignored
  ancestor alias is the missing dependency. No claim that the other forms
  exhaust all filesystem arrangements is made.
- **External links and R:** finding 2 covers file links and directory links
  that leave and return. Searching an impractically large R has an explicit
  safe fallback: keep every entry. That fallback is sound when reached; it
  does not fix an R or intermediate component that is never reported.
- **Silence for failed paths:** for a genuinely missing path, non-directory
  component or loop, the same-path argument is sound for current reads:
  removal cannot turn an already failing open into a new failure. The guide
  separately requires mounting volumes and preserving mount aliases for
  other path views. Finding 4 concerns an unreadable chain incorrectly
  classified as one of those failures. Later repair of a currently broken
  link is not proved safe by the current-read argument.
- **Directory-alias exemption:** finding 1. Equality of the final directory
  does not make the path used to reach it independent of the Grove root.
- **Practical cleanup:** the procedure can be unusable for an ordinary home
  with unreadable Library directories, broad resumable histories or links to
  large external trees. This is a disclosed conservative cost, not additional
  deletion authority. Removing only independently owned, unneeded skill links
  while retaining the whole Grove root would eliminate snapshot reclamation
  from this procedure and save little less space in the recorded installation.
  It would still require honest provenance and dependency checks for those
  links. Integration should consider that narrower scope before extending
  the search again. This review does not choose or implement the replacement.

## Evidence and limits

This review did not run the counterexamples or repeat k45's recorded checks.
The verification record is evidence of its named cases, not a completeness
proof. No production or test file changed. Step 5 checks required skills; it
does not generally check retained foreign paths or lock-file contents.

Graph evidence used Verify tier in project
`Users-antony-Development-AgenticOrchestration-grove.gh-issue-25`, generation
`2026-11-01T19:43:40Z`. The installer query returned all 14 results with
`has_more: false`. Coverage reported a parse gap at `plugins/install.sh:188-447`;
the entire file was read directly, including that range. The guide and spec
had matching metadata and no recorded coverage issue. The removed provisioner
is missing from the current index and was read in full through jj at the
historical revision. No exhaustive repository or call-chain claim is made.
