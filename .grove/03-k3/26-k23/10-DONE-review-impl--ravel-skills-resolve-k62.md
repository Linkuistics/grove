# ravel-skills-resolve-k62

**Reviews:** ravel-skills-resolve-k54

## Goal

Read the local resolver of Ravel commit `777e609f` adversarially, against
`docs/specs/skill-discovery.md` at `822c0ed2`. Produce findings, not fixes.

## Specific doubts

- The link-chain walk in `crates/ravel/src/skills/snapshot.rs` resolves each
  component inside the canonical source root. Can a chain of relative links
  escape the root, reach a `.git` or `.jj` entry, or loop without refusal? One
  mutation of the `..` check first survived the fixtures.
- The protected-overlap check in `skills/mod.rs` compares canonical and
  lexical forms of `$HOME/.config/ravel`, the settings file and the cache
  root. Is there an alias that escapes it?
- Publication seals the object contents, renames with no replacement and then
  seals the object root. A loser compares only the manifest bytes of the
  winner. Is any state visible to a reader before the object is complete?
  Deep checks of reused objects are `ravel-skill-publication-k56`. Judge only
  what this leaf claims.
- The discovery instructions in `skills/preamble.rs` are this leaf's own
  wording. Do they carry each obligation of "The preamble" and each row of
  "Agent discovery and native installations", with no added rule?
- The strict settings reader in `strict_json.rs` replaces `serde_json` for the
  whole settings file. Does any document that the old reader accepted now
  parse differently, apart from a repeated member name?

## Done when

- Each finding names a file, a line and a concrete failing input, or the review
  states that it found none.

## Decisions (running log)

- Review boundary: Ravel commit `777e609f2fe31b9a2ceb8e11645d2f491c7a1d85`,
  parent `eac3c987`, against the discovery specification at
  `822c0ed2f67d9a446b74476fdad7d05922552b87`. Read the committed diff,
  resolver, settings reader, preamble, public fixtures, documentation and the
  producer's verification record. No test, build, lint, format or reproduction
  command ran, and no Ravel file changed.
- Graph evidence uses the Verify tier with direct-source fallback. The Grove
  index was ready. The nearest indexed Ravel project is the default checkout,
  generation `2026-11-01T07:59:38Z`; it has no resolver symbols and coverage
  reports the new resolver paths missing. It cannot answer questions about this
  feature commit. Exact `jj file show -r 777e609f` reads supply the evidence.
- Four static counterexamples warrant integration. All are medium priority:
  a cold-cache alias permits self-capture, a case alias reaches an excluded
  entry, mandatory references can duplicate one retained file, and an
  unreadable mandatory target receives the resource refusal. They do not
  depend on concurrent source edits or cache tampering.
- The current Ravel parent is `ba0f388ecb63`, after the other local producers.
  Its protected-path construction still canonicalises only complete existing
  paths. Integration must reconcile all findings with that later code. This
  review does not redo the publication, bounds or confinement reviews.
- Inserted `ravel-skills-resolve-k66` as `integrate-review-impl` immediately
  after this review, before `ravel-skill-publication-k63`. It consumes this
  review's handle and owns triage, fixes and verification. The local-delivery
  parent still has live work, so no parent closes in this session.

## Findings

All file and line locations below refer to Ravel commit `777e609f`. These are
source-derived counterexamples, not executed reproductions.

### 1. A cold cache below an aliased ancestor bypasses overlap refusal

Location: `crates/ravel/src/skills/mod.rs:220`, with the overlap comparison at
line 232 and cache creation after origin validation at line 102.

Let `H` and `P` be separate absolute directories owned by the account. Put the
valid settings under `H/.config/ravel`, make `H/.cache` a symlink to `P`, and
leave `P/ravel/skills/v1` absent. Configure one local source with path `P`,
roots `["."]` and no mandatory files. No configuration or result file is in
`P`.

Canonicalising the complete cache path fails because its final directories
do not exist. Canonicalising `H` and appending the cache suffix does not resolve
the `.cache` link. Neither retained spelling is beneath canonical source `P`,
so the origin passes. `Cache::open` then creates `P/ravel/skills/v1`, including
its private staging, inside the source. Capture walks that staging and starts
copying its own destination recursively until a filesystem limit refuses it.
The promised `skill_access_unavailable` before capture is lost, and an ordinary
first resolution can consume storage and directory depth. This needs no link
replacement or unsafe cache root; a stable owner-controlled ancestor alias is
enough. The existing overlap fixture uses a lexical cache parent and a fully
existing configuration alias, so it does not exercise this case.

### 2. A case alias of a reserved entry becomes an admitted dangling link

Location: `crates/ravel/src/skills/snapshot.rs:306`, with exclusion of the
actual directory entry at line 190.

On a case-insensitive source filesystem, create a source with a directory
whose stored name is `.git`, a regular `.git/HEAD`, and `bad -> .GIT/HEAD`.
Configure roots `["."]` and no mandatory files. The same case can use `.jj`
and `.JJ`.

Directory enumeration omits the stored `.git` entry. Link validation compares
the literal target component `.GIT` with the case-sensitive reserved strings,
then asks the source filesystem for its metadata. The filesystem reaches
`.git/HEAD`, so validation admits the link. Capture preserves `bad` but omits
its target directory, and the configured root still passes. The resolver can
therefore publish a snapshot with a dangling link, despite requiring refusal
of both dangling links and chains that reach excluded entries. This is not a
collision between two source names: one stored directory and an alternate
spelling in a link are sufficient. The existing reserved-target fixture uses
the exact spelling `.git/HEAD` only.

### 3. Different source IDs can produce duplicate mandatory references

Location: `crates/ravel/src/skills/settings.rs:152`, with unconditional
mandatory descriptor construction at `crates/ravel/src/skills/mod.rs:442`.

Configure local sources `a` and `b` with the same absolute source path `P`,
roots `["."]`, and a readable `P/SKILL.md`. Configure mandatory records
`{"source":"a","path":"SKILL.md"}` and
`{"source":"b","path":"SKILL.md"}`.

The duplicate check compares the source index as well as the relative path,
so it accepts both references. Both captures share one digest and snapshot,
and the descriptor emits two mandatory records with exactly the same absolute
file path. The preamble consequently orders the same mandatory file twice.
The Settings section of the reviewed specification says that two references
to the same file refuse. No check after settings parsing enforces that rule.
Distinct paths through an admitted symlink to one file have the same gap.
The existing duplicate fixture repeats the same source ID and path only.

### 4. An unreadable mandatory target reached through a link has the wrong code

Location: `crates/ravel/src/skills/snapshot.rs:244`, with the lexical mandatory
path set built at `crates/ravel/src/skills/mod.rs:110`.

Under an ordinary account that cannot bypass file permissions, create
`P/secret.md` with mode `000` and `P/mandatory.md -> secret.md`. Configure
one local source `P`, roots `["."]`, and the sole mandatory reference
`mandatory.md`.

The link is relative, remains inside the source and reaches a regular file.
When capture opens `secret.md`, it fails. The mandatory set holds only
`mandatory.md`, so the exact string lookup is false and capture returns
`skill_resource_invalid`. The later mandatory check never runs. The specified
refusal for a known source whose mandatory reference reaches an unreadable
file is `skill_mandatory_invalid`; its remedy must address the mandatory
reference. The existing unreadable-file fixture configures the regular
file's direct name and does not exercise an admitted alias.

## Other specific doubts

- **Link-chain boundary and cycles:** In a quiet source, the component walk
  expands each encountered relative link before consuming the remaining
  suffix. A `..` cannot pop above the canonical root, absolute targets refuse,
  missing components refuse, and the hop limit terminates a cycle. I found no
  root escape through that expansion. Finding 2 concerns filesystem aliases
  of reserved names, outside the literal component comparison. Concurrent
  entry replacement belongs to the publication review.
- **Protected configuration:** Existing configuration and settings aliases
  have their canonical targets included, and overlap is checked in both
  containment directions. Finding 1 is the counterexample for a protected
  cache path that does not exist yet.
- **Publication completeness:** `publish` seals all contents before the
  no-replace directory rename at `snapshot.rs:478`. A new reader can see all
  object contents together; I found no partially populated tree or descriptor
  in that sequence. There is a window before the outer directory is changed
  from `0700` to `0555` at line 483, including after publisher death. This is
  the already named seal-window question of `ravel-skill-publication-k63`,
  which owns its triage. Comparing only the winner's marker does not detect
  tree tampering, but deeper reuse checks are explicitly k56's work and are
  not an additional k54 finding.
- **Preamble:** Compared the five preamble obligations and all four discovery
  rows with `preamble.rs:11`. Mandatory order and supporting reads are in
  obligation 1; discovery, resource bases and lazy reads in 2; name, native,
  path and restricted mechanisms in 3 to 6; metadata in 7; ambiguity and
  duplicate copies in 8; resource failures and revision retention in 9;
  compaction and recovery in 10; and delegation in 11. No actionable omission
  or added behavioural rule was found. Instructions alone do not establish
  harness compliance or remove inherited ending authority.
- **Strict JSON compatibility:** `strict_json::parse` uses the same
  `serde_json` deserializer and end-of-input check as the former Value read.
  Its visitor handles JSON scalar, array and object values, while its map
  visitor rejects repeated decoded member names. No additional accepted-input
  regression was found by inspection. The producer's compatibility fixture
  covers one mixed document, not an exhaustive equivalence proof.

## Verification limits

The producer records macOS checks at the pinned commit, including public
fixtures and firing controls. Those are historical evidence, not checks run
by this review. This review supplies no Linux execution, no new harness
measurement, and no claim that the four static cases were reproduced.
Integration owns fixes and post-fix verification at the current feature tip.
