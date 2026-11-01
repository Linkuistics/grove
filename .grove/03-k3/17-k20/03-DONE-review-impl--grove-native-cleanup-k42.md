# grove-native-cleanup-k42

**Reviews:** grove-native-cleanup-k33

## Goal

Read the optional manual cleanup procedure adversarially, after its rewrite. Find
any case in which a user who follows it literally loses a foreign, uncertain or
still-needed entry.

## Context

The artifact is the section of `docs/USAGE.md` from the anchor
`usage-earlier-codex-installations` to the anchor `usage-tree-verbs`, as the
producer's commit left it. The contract is the delivery specification's section
*Existing installations and active readers*. The ground truth for the old writer
is `crates/grove/src/provision.rs` and `crates/grove/build.rs` at `175e5c7d259^`.
The manual installer is `plugins/install.sh`.

The producer spent its one in-session reviewer before the rewrite. The rewrite
changed the ownership conditions, the reference and alias search, the installer
guidance, the recovery order and the removal order. Its own disposable exercise
is recorded on the producer leaf. That exercise ran only on macOS, with
canonical home paths.

## Done when

- The specific doubts below each have a finding or an argued dismissal.
- Other findings are reported with a concrete scenario and evidence.
- No fix is applied. Findings worth acting on get an
  `integrate-review-impl` leaf, as the review family directs.

## Notes

These doubts remain after the rewrite:

- Whether "aliases" and "retained references" are now told apart correctly.
  The Codex skill directory itself can be a link, and the Codex home can be a
  link given in `CODEX_HOME`. A link to an ancestor such as `~` is not searched.
- Whether the first link condition is checkable in practice. It asks who made a
  link at a path. The provisioner kept no record, and it replaced owner links.
- Whether a `link-XXXXXX` staging directory is classified safely. A harness
  can scan nested directories in a skill directory.
- Whether the search commands behave on Linux and on a non-canonical `HOME`.
  The exercise did not cover either.
- Whether the step 5 recovery order and the step 6 removal are complete when
  only part of the layout moved.

## Decisions (running log)

- This review inspects producer commit `de9e634b226e554fde626c628e95e5f3141db732`.
  The ownership contract is `docs/specs/codex-skill-provisioning.md`, Existing
  installations and active readers. The old writer and build inventory were
  read in full at `175e5c7d259^`; the current manual installer's relevant source
  was read directly. No cleanup command or disposable exercise is run here.
- The reference search and the `current` removal condition have actionable
  gaps. The absence of an ownership receipt is instead a conservative limit:
  without sufficient independent records, a link remains uncertain and stays.
- Reverse recovery order is correct for the subset actually moved. The final
  empty-root condition also preserves a root with retained entries. These facts
  do not repair an earlier classification that missed a dependency.

## Findings

All locations below refer to the producer commit. Each finding is medium
priority: it affects the optional destructive procedure in a specific supported
layout and should be resolved before that procedure is published.

### 1. The reference inventory excludes dependencies inside foreign entries

Location: `docs/USAGE.md:786-800`, with the top-level inventory at 764-773.

The definition excludes every reference inside `~/.agents`, and both `-path`
conditions exclude links anywhere under that directory. The inventory lists
only immediate children of each skill directory. Consequently, a retained real
directory such as `~/.agents/skills/custom` can contain a link
`resources -> ~/.agents/.grove/snapshots/bundle-OLD/plugins/...` that neither
search reports. The procedure explicitly keeps `custom`, but it can classify
the otherwise complete snapshot as unneeded. Removing that snapshot breaks
the foreign directory's resources. A reference through a provisioned skill link
has the same problem.

There is a second omission in the text search: the sole literal `.agents/`
does not match a settings or script reference to `~/.codex/skills/NAME`, an
external `CODEX_HOME/skills/NAME`, or `$CODEX_HOME/skills/NAME`. Adding search
directories as instructed changes neither this pattern nor the exclusions.

The inventory needs to account for dependencies held by retained foreign
entries, including nested entries, and text references using the supported
Codex path spellings. A clean result from the supplied searches cannot establish
the no-reference removal conditions. Preserve entries whose dependencies cannot
be established. The integration should exercise both a nested retained link
and a text reference that contains no `.agents/` literal.

### 2. Links to ancestors are omitted from the alias model

Location: `docs/USAGE.md:786-798` and 833-835.

Consider `~/home-alias -> /home/user`, where `/home/user` is the home directory.
A harness can use `~/home-alias/.agents/skills/NAME`. The two case statements
match only targets equal to, or below, the agents directory or Codex home. The
target `/home/user` matches neither. The definition also does not call this an
alias. This remains true if the search visits the link successfully. A launcher
that sets HOME to the alias can leave only `HOME=.../home-alias` in its settings;
the text search then has no `.agents/` literal to find either.

An owner can therefore miss the alias and classify NAME as unneeded, even
though the same unresolved reader would require keeping it if the alias pointed
directly to the skill directory. The model and search need to account for a
link to any ancestor through which the layout is reachable, with the same
conservative treatment of unknown readers. Merely broadening the search roots
does not change the target predicate.

### 3. A symlink used as the home search root prevents the promised traversal

Location: `docs/USAGE.md:794-806`.

With `HOME=/home/user` and `/home/user` a link to `/storage/user`, the earlier
inventory commands address children of the home and still work. However,
`find ~` starts at the symlink itself. Without a link-following option, `find`
does not traverse that starting directory link. Its resolved target is an
ancestor of the agents and Codex directories, so the supplied case statements
also print nothing for it. Existing dependency links within the physical home
are never visited. The apparent clean search contradicts the promise that it
visits every link in the home.

A linked Codex home outside the physical home has a related problem: adding
that link as another search root does not by itself traverse its contents.
Resolve intended search roots or specify the necessary root-following behavior,
and account for errors before treating the search as evidence. This is a static
finding about the command's traversal; no macOS or Linux exercise was run by
this review.

### 4. A retained text reference protects its snapshot but not `current`

Location: `docs/USAGE.md:859-860` and 877.

A settings file can name
`~/.agents/.grove/current/plugins/grove/skills/grove` directly. The supplied
grep finds that file. Suppose every skill-directory link is otherwise unneeded
and is moved. The snapshot condition correctly keeps the referenced snapshot,
but the separate condition for `current` checks only links that stay. No such
link exists in this case, so it permits moving `current`. The settings path
then stops working despite the snapshot remaining in place.

The pointer must retain every dependency that requires traversal through it,
including text references and aliases, rather than only remaining symlinks.
The integration should exercise this partial cleanup: links move, the referenced
snapshot and its required `current` pointer stay. New-session checks cannot
replace preserving a known reference used by an occasional script or reader.

### 5. An owned staged link does not establish ownership of its whole directory

Location: `docs/USAGE.md:837-839` and 882-885.

The procedure makes the whole `link-XXXXXX` directory removable when its `link`
passes the link conditions. Those conditions establish neither that the
directory contains only that link nor that additional children are owned.
For example, an interrupted provisioner can leave an evidenced owned staged
link, after which an owner puts a diagnostic file beside it. There can be no
reader or reference to the staged skill, and every stated link condition still
holds. Moving and finally deleting the whole directory deletes the foreign
diagnostic file as well. The general rule for preserving real skill directories
explicitly exempts these directories.

The old `replace_link` implementation creates a temporary directory and one
child named `link`; it supplies no basis for owning another child. Classification
of the whole container must account for every child and keep a mixed or
uncertain directory. Listing its contents in step 2 does not supply that missing
condition. The integration should also preserve a staged skill that a harness
discovers recursively: its lack of an explicit path reference alone does not
make it unneeded.

## Disposition of the mandated doubts

- Aliases and retained references: findings 1-4. The updated text handles direct
  skill-directory and Codex-home aliases in ordinary canonical layouts, but
  misses ancestor aliases and dependencies outside its search model.
- First link condition: no additional defect established. The provisioner has
  no receipt and did replace owner links, as the guide states. Records showing
  only the present target cannot satisfy this condition. If the owner lacks
  sufficiently complete independent history, the entry is uncertain and stays;
  an inability to remove it is consistent with optional cleanup.
- Staging directories: finding 5 covers foreign contents. The ordinary link
  conditions already require that the skill is unneeded in that directory and
  through its aliases. Read literally, that includes recursive harness
  discovery, so no separate exemption based solely on the `link-` name is
  justified. Findings 1-3 still undermine the dependency search.
- Linux and non-canonical HOME: finding 3 covers the traversal failure, which
  is not specific to Linux. The producer recorded no Linux exercise. No Linux
  portability pass is claimed here, including for the missing-path behavior of
  its chosen `realpath` implementation.
- Partial recovery and final removal: the reverse dependency order in step 5
  is correct when restoring only moved entries. Step 6 requires the root to
  contain only the lock and an empty snapshots directory, so it does not
  authorize removal of a root with retained snapshots, `current`, or staging
  directories. Finding 4 concerns an earlier misclassification, not reversal
  of that order. No additional defect is established in these two steps.

## Evidence and limits

This is an inspection-only review. No test, build, lint, format, installer or
cleanup command was run, and no production or test file was changed. The
producer's verification record was read as historical evidence, not reproduced.
The scenarios above are static counterexamples, not observed disposable runs.

Graph evidence used Verify tier in project
`Users-antony-Development-AgenticOrchestration-grove.gh-issue-25`, generation
`2026-11-01T18:19:53Z`. The installer symbol search returned all 14 results
with `has_more: false`. Coverage reported installer parse gaps at lines 188-447;
those lines and the harness-target declarations were read directly. The removed
provisioner and build script are absent from the current source index, so their
complete historical source at `175e5c7d259^` was read through jj. The guide and
delivery specification had matching index metadata with no recorded gap; both
were read directly for the material claims. No exhaustive repository claim is
made.
