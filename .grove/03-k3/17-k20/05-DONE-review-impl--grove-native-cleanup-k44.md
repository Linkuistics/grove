# grove-native-cleanup-k44

**Reviews:** grove-native-cleanup-k43

## Goal

Read the optional manual cleanup procedure adversarially again, after the
integration of grove-native-cleanup-k42. Find any case in which a user who
follows it literally loses a foreign, uncertain or still-needed entry.

## Context

The artifact is the section of `docs/USAGE.md` from the anchor
`usage-earlier-codex-installations` to the anchor `usage-tree-verbs`, as the
commit of grove-native-cleanup-k43 left it. The contract is the delivery
specification's section *Existing installations and active readers*. The ground
truth for the old writer is `crates/grove/src/provision.rs` at `175e5c7d259-`.
That file did not change between its first commit `92a8bbee0037` and that
revision. The manual installer is `plugins/install.sh`.

The integration rewrote the search twice. Its one in-session reviewer read the
first rewrite and reported 13 findings. All were applied, and the second rewrite
has had no independent read. The integration's verification record holds the
exercise scenarios S1 to S13, the platforms and the remaining limits. Treat
those scenarios as the cases that the author already thought of, not as
evidence of completeness.

## Done when

- The specific doubts below each have a finding or an argued dismissal.
- Other findings are reported with a concrete scenario and evidence.
- No fix is applied. Findings worth acting on get an
  `integrate-review-impl` leaf, as the review family directs.

## Notes

These doubts remain after the second rewrite:

- Whether any search can establish "no retained reference". Two adversarial
  rounds each found new classes of missed references. Consider whether a more
  conservative rule would close the class instead. An example rule keeps
  `current` and every snapshot when any line other than an inventory link
  appears.
- Whether the manual steps are literal enough. The user follows a chain with
  `readlink` and adds each reported link to the text patterns. The user also
  judges that a transcript is not a reader.
- Whether the text search, which now follows links, is practical. A foreign
  link to a large tree makes it search that tree. Each unreadable file then
  keeps every entry. On macOS without full disk access, this can make cleanup
  impossible.
- Whether the patterns `.agents` and `.codex` without a slash give so much
  output in a real Codex home that a user stops reading it.
- Whether the rule that keeps every link in a separate Codex skill directory
  holds for a layout in which `CODEX_HOME` changed between Grove runs.
- Whether the `(unresolved)` filter on link text misses a loop or an unreadable
  chain whose text names neither `.agents`, `.codex` nor the real agents path.

## Decisions (running log)

- The reviewed artifact is `docs/USAGE.md:694-1001` at `c22acbde`, the
  grove-native-cleanup-k43 commit. The preservation contract, the historical
  provisioner and build script at `175e5c7d259-`, and the current manual installer
  were read directly. This review runs no test, build, lint, format, installer
  or cleanup command. Its counterexamples are static, not exercised scenarios.
- Five issues warrant integration: omitted intermediate dependencies, references
  inside retained cache entries, an ownership comparison that ignores foreign
  files, suppressed unresolved links, and collisions in the holding directory.
  The broad traversal and search patterns are conservative usability costs.
  Keeping all links in a separate Codex directory is also conservative, even
  when a past environment gives a link a different history.
- grove-native-cleanup-k45 is the integration step, inserted before
  grove-skill-mandate-k22 so no intervening work can move the cited text.
  The parent remains live with these two tasks; no parent closes here.

## Findings

All locations refer to `docs/USAGE.md` at `c22acbde`. Each finding is medium
priority and concerns the optional cleanup procedure. Resolve them before
publication. No fix is applied here.

### 1. A resolved link can need `current` without ending inside the cache

Location: lines 808-818 and 840-844; removal conditions at 927-928 and 949-952.

Suppose `current` points to
`$HOME/.agents/.grove/snapshots/bundle-OLD`, and a foreign link is:

```text
$HOME/guide -> $HOME/.agents/.grove/current/../../../../guide.txt
```

Here each `$HOME` denotes the actual absolute home path. The target exists and
resolves to `$HOME/guide.txt`. Opening the link still requires `current` and
its target directory: the kernel traverses them before processing `..`.
Removing either breaks the foreign link.

The search drops this link. Its final real path is neither inside the agents
or Codex skill directory nor an ancestor of a searched directory. Although its
stored text names `.agents`, that predicate runs only when `realpath` fails.
The subsequent instruction to inspect reported chains never reaches it.
A launcher that reads `"$HOME/guide"` supplies none of the text patterns either.
Even searching that launcher does not expose the dependency.

The final real path cannot determine every directory needed to traverse a
link. The inventory must preserve these intermediate dependencies or explicitly
leave their absence unproved. Showing the stored text only for links selected
by their final target does not establish the removal conditions.

### 2. Retained foreign files inside the Grove root have no reference inventory

Location: lines 790-792, 806 and 851; snapshot conditions at 927-928.

Consider a `link-AbC123` staging directory containing its staged `link` and an
owner's `restore.sh`. The staged link points through `current` to bundle-NEW.
The script reads a resource by its absolute path in bundle-OLD. The old snapshot
is otherwise complete and equal to its release, and nothing outside the Grove
root references it.

The revised staging rule correctly keeps the whole directory because it holds
an extra child. But both searches prune the entire Grove root, and the definition
of a retained reference excludes every text there. The staged link keeps
bundle-NEW; it does not keep bundle-OLD. The old snapshot therefore meets the
listed removal conditions even though the explicitly preserved script needs it.

The same omission applies to a foreign file elsewhere inside the Grove root.
Keeping its container does not preserve its resources. The inventory must
account for dependencies inside entries that stay, including these entries.

### 3. Ignoring every `__pycache__` child admits foreign snapshot contents

Location: lines 922-947, especially the comparison at 935.

Take a genuine snapshot and add an owner's diagnostic file at
`plugins/grove/skills/grove/__pycache__/notes.txt`. The snapshot still holds
only `plugins`, with only directories and regular files. The special-file
search reports nothing. `diff -r -x __pycache__` also reports no difference,
because it excludes the entire directory and every file inside it.

The guide expressly allows this difference for generated bytecode. It supplies
no provenance check for an excluded file and no constraint on its name or
contents. Thus a user following the supplied comparison can classify this
mixed snapshot as complete and owned, then delete the foreign diagnostic file.
The old build script collected every regular file; it did not reserve these
directories for generated files or record their ownership separately.

A comparison that ignores arbitrary contents cannot establish ownership of
those contents. An unexplained difference must remain uncertain under the
preservation contract, including below an excluded directory.

### 4. Unresolved links with other target spellings disappear without a warning

Location: lines 808 and 815-832; error handling at 871-875.

Suppose `$HOME/bin/skill-data` points to `/opt/private/skill-index`. That external
directory is currently unreadable. Its index may point to the old layout; the
owner cannot establish the chain while access is unavailable.

The physical link search does not enter the external target. `realpath` fails,
but its error is redirected away. The stored target text contains none of
`.agents`, `.codex` or the agents real path, so the fallback prints nothing.
The text search starts at the configuration, agents and Codex directories; it
does not follow this link in `bin`. There need be no other diagnostic.

This contradicts the assurance that both searches report each unreadable
location. The rule that preserves entries when a location is unreadable cannot
operate when the failure is hidden. The claim that the layout has no retained
dependency must remain unknown for an unresolved chain regardless of how its
first target is spelled. The same filter drops an unrelated-looking link loop;
its harmlessness has not been established by suppressing it.

### 5. A pre-existing holding directory permits overwriting foreign entries

Location: lines 972-977 and 992.

Suppose `~/grove-codex-cleanup` already exists and holds an owner's regular file
named `current`. This can also be a holding directory from an earlier cleanup.
The `mkdir` command reports that the directory exists, but these commands run
in an ordinary POSIX shell and there is no instruction to stop on that error.
The subsequent `mv .../current "$hold/"` overwrites the existing file. A prior
held skill link at `agents-skills/NAME` can likewise be replaced.

No ownership rule classifies these destination entries. Step 6 then tells the
user to delete the holding directory, including any unrelated contents that
were already there. The source classification cannot authorize overwriting or
deleting a foreign destination. The holding area must be established as fresh
and exclusively owned before it can support the claimed reversible move.

## Disposition of the mandated doubts

- **Establishing no retained reference:** findings 1, 2 and 4 give concrete
  omissions. A successful text search does not establish the claim. Keeping
  `current` and every snapshot whenever an extra line appears is a conservative
  option, but that rule alone cannot address references the search never prints.
  Integration should settle the preservation rule before extending another
  list of reference patterns.
- **Literal manual steps:** the chain instructions show no method for finding
  a link in an intermediate path component. `readlink` on an entire target path
  ending in a regular file does not enumerate those components. The stated
  fallback keeps `current` and every snapshot when the chain cannot be followed,
  so this difficulty alone does not authorize deleting them. Finding 1 concerns
  the earlier filter that prevents that fallback from being considered at all.
  A transcript is not itself a running reader. However, a saved transcript or
  log consumed by a resume tool or script still carries a retained reference
  under the preceding definition. Calling it a log does not prove that no
  future reader needs its paths. No separate unconditional deletion permission
  for that case is established by the literal text.
- **Traversal cost and unreadable targets:** `find -L` can leave the configured
  roots through foreign links, search a large tree, encounter loops and report
  permission errors. The final text discloses the traversal and preserves
  entries on unreadable locations. That can prevent all optional cleanup,
  including on macOS without full disk access, but it preserves the contract.
  No additional correctness finding follows from cost alone. Finding 4 is
  different because its error is suppressed.
- **Broad `.agents` and `.codex` patterns:** these increase the files a user
  must inspect, including ordinary skill files and histories. This is a
  usability limit, not independent permission to discard a retained reference.
  No deletion counterexample follows solely from the volume of matches.
- **Changed `CODEX_HOME`:** a separate Codex skill directory can have been the
  canonical agents directory under an earlier `HOME`. For example, a later run
  can set `CODEX_HOME` to that earlier home's `.agents`. The historical writer's
  per-run distinction does not prove who originally created every present link.
  Nevertheless, the instruction at 899-900 keeps all such links. Its historical
  explanation is too broad, but its prescribed action is conservative. No
  additional loss finding is warranted.
- **Unresolved filter:** finding 4. The target need not contain any of the
  spellings used by the filter to leave the dependency unknown.

## Evidence and limits

This is an inspection-only review. The integration's S1-S13 record was read as
historical evidence. None of those runs was repeated, and none of this review's
counterexamples was executed. No production or test file changed.

Graph evidence used Verify tier in project
`Users-antony-Development-AgenticOrchestration-grove.gh-issue-25`, generation
`2026-11-01T18:56:15Z`. The installer search returned all 14 results with
`has_more: false`. Coverage reported a parse gap at `plugins/install.sh:188-447`;
that entire range and the harness-target declarations were read directly.
The guide, delivery specification and delivery ADR had matching metadata and
no recorded gap; their material text was read directly. The removed provisioner
and build script are absent from the current index and were read from the
historical revision through jj. No exhaustive repository claim is made.
