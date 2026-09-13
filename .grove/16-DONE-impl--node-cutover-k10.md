# node-cutover-k10

## Goal

Release and install the checked binary and matching plugin, cut this tree over
to the node-file grammar, and reach the remaining migration work under a
compatible driver, through the human restart when the running driver is old.

## Context

- Routed from review `node-methodology-k20`, F3, by integration
  `node-methodology-k21`: reconcile CLI help and user-facing refusals with the
  glossary's distinction between a node file and its brief body. Inspect
  `crates/grove-llm/src/cli.rs` (`pick`, `leaf-retire`, `leaf-prune`,
  `leaf-decompose`, `root-init`) and `crates/grove-loop/src/tree_lifecycle.rs`
  (the cannot-decompose/retire/prune-a-brief diagnostics), then reconcile their
  consumers and the declared walkthrough fragments/prose in the same code
  increment. Preserve body uses of "brief" and existing API identifiers;
  this is user-facing terminology, not an API rename. Verify through the
  existing test seams and `bash scripts/check.sh` before release preparation.
- Depends on `node-documentation-k9` and any review/integration it earns.
  This is the last code/release leaf before `grove-migration-k11`.
- `docs/RELEASING.md` owns the release route: prepare/check, the default
  colocated jj workspace, version cut, archives, repository and Homebrew tap
  publication, install verification. Read the current procedure and release
  tooling; do not infer shell commands or a release version from this brief.
- The current driver is installed 20.2.0, already running in memory; changing
  PATH binaries cannot upgrade it. Cached skills also change only on refresh.
  `plan-k1` decisions 7 and 10 and the root brief authorize the migration
  workflow and require this human stop/restart boundary.
- This is the secondary jj workspace
  `~/Development/grove.dont-put-item-title-in-folder-names`; the default is
  `~/Development/grove`. Release integration materializes this tracked tree
  there too. The secondary workspace owns the actual tree conversion; after
  its final sealed cutover change, advance the default workspace onto that
  change, preserving unrelated work. Include both copies in preflight and
  installed-reader verification. Do not start a driver in the default copy.

## Done when

- Before asking for release approval, prepare a concrete reviewable release:
  finished changelog, version decision, green `bash scripts/check.sh`, release
  doctor result, exact commits/artifacts to publish, plugin-refresh route, and
  an inspected rename plan for this tree. Resolve routine failures first.
- Re-enumerate live groves under `~/Development`. Prepare temporary converted
  copies for new-binary preflight and compare selected permanent keys/outcomes
  to the untouched trees; terminal trees need no fabricated live selection.
  Expose malformed or unexpected shapes before publishing. Keep this workflow
  outside shipped code, with no product migration verb or dual-grammar reader.
- The preparation session records the concrete plan, artifact identities,
  workspace commands and any still-needed publication/runtime approval in this
  leaf's running log. Preserve existing explicit authorization. If approval
  is outstanding at the unattended handoff, snapshot with `jj status`, leave
  this handle live, and return without a signal: that stops the driving loop.
  Do not retire or seal a completed-cutover commit at preparation alone.
- The human stops the other active Grove loops, records approval of the named
  plan and confirmation of those stops in this log (or supplies an explicit
  reply that the session records). After a stopped preparation session, the
  human re-runs `grove` in this secondary workspace. If both confirmations
  arrive while preparation is still running, that session may continue with
  execution below. Restart itself is not approval. With no recorded authorization,
  the resumed session stops again without publishing or signalling. Other
  groves remain stopped until `grove-migration-k11` verifies them.
- The resumed session rechecks the plan against current files, then performs
  the approved publication, installation, plugin refresh, conversion and
  verification. Its own driver remains waiting for it throughout; do not kill
  that driver mid-session. If the human executes the plan instead, they record
  what actually completed and ensure this tree and the plugin match the newly
  installed reader before restarting; the session verifies those results and
  performs only the remaining work. Never restart a new reader on this tree
  before conversion, or the old reader after conversion.
- The release is actually published and installed, and both `grove` and
  `grove-llm` resolve to the intended build. Verify the installed plugin's
  bootstrap/format instructions against the published source. Follow the
  release procedure's integration and version/tag checks in the default
  workspace; keep unrelated work out of the release.
- This workspace's `.grove/` has `_BRIEF.md` at its root and exactly one
  `_<slug>.md` in every `NN-k<key>/` directory. Preserve node keys, leaf
  names/outcomes, bytes, headings and order; reject collisions before renames.
  Use bottom-up directory renames so deep descendants stay addressable. Keep
  a recoverable snapshot and item-by-item mapping outside shipped artifacts.
- Open this tree with the installed new binary and verify handles, brief
  chains and the remaining migration selection. Clear stale `GROVE_SIGNAL_FILE`
  for post-stop administrative probes; invoke `grove-llm` directly. Retire this
  leaf only after its release/install/rename work is done, using the new binary,
  and include the retirement and task-tree conversion in its sealed jj change.
  Ensure that change descends from the release change without losing either
  workspace's edits, then advance the default workspace to the sealed result
  and verify its actual tree too. The release tag stays on the release cut.

## Notes

**Choose the ending from the mandate's running Grove version.** An unfinished
handoff always leaves the handle live and returns without a signal. After all
work is verified, retired and sealed, a session launched by 20.2.0 (or another
driver predating this grammar release) returns without `grove-llm complete`;
the human then starts the newly installed `grove` in this workspace for
migration. A resumed session already launched by the compatible new driver
uses ordinary `grove-llm complete` as its last action. PATH replacement does
not change which case applies. After an interruption, inspect the running log,
jj diff and installed versions before resuming this same live handle; if the
tree is partially converted, finish or restore from the recorded mapping
outside the loop before restarting a reader that can open it.

The four other known trees are `APIAnyware.add-ocaml-target`, `InTheLoop`,
`Writegood` and `grove.gh-issue-12`, all under `~/Development`. Do not let their
drivers restart between install and migration. If a prepared preflight reveals
product defects, insert precise repair work ahead of this leaf before entering
the cutover window. Any release source edit still owes its book in the same
commit; a release version edit may affect a manifest reconstructed by a book.

## Decisions (running log)

1. Preparation began on 2026-09-12 under driver 20.2.0, from clean secondary
   change `mowsyyrq`, parent `9278f021`. No publication approval or confirmation
   that other loops are stopped has been supplied. This preparation must leave
   the leaf live and return without a signal.
2. Reconcile the routed F3 terminology in CLI help, lifecycle refusals and their
   existing tests/books. Preserve API identifiers and uses of brief for body
   text. This is string/comment plumbing with no library-version decision.
3. Prepare version **21.0.0**: the required node-file grammar breaks existing
   trees, so a major release communicates the compatibility boundary. The
   default workspace is clean at `f394b9dc`, parent `d91d6df1` (`main`,
   `v20.2.0`). Release doctor passes all prerequisites (Rust 1.93.1, three
   target stds, Zig 0.16.0, cargo-zigbuild and GitHub authentication).
4. Inventory found six current trees, adding `Modaliser` to the four previously
   known external groves. The default Grove workspace has no `.grove/` yet;
   release integration will create its seventh copy. Converted scratch copies
   live at `/tmp/grove-k10-preflight/`, with original bytes, SHA-256 inventories
   and item-by-item mappings. Five trees preserve their selected leaf filename
   and pass the candidate's `pick` and `brief-chain` (candidate still reports
   the source version 20.2.0; it is not installed).
5. APIAnyware preflight refuses one existing node with no `BRIEF.md`:
   `11-libapianyware-ocaml-dylib-k10/05-init-and-method-trampolines-k87/04-async-and-throws-reentry-chain-k112`.
   It contains a child node and two terminal leaves; it is not an empty
   interrupted decomposition. Do not silently fabricate a brief or weaken the
   reader. The exact-path jj history query found no recovery candidate. This
   needs explicit recovery/disposition before the cutover can be approved.
6. Plugin refresh must address the real directory `~/.codex/skills/grove`,
   preserving it in the cutover backup before replacing it with a symlink to
   the default checkout's shipped spine. `grove-impl` already links there.
   Claude's installed `grove@linkuistics` cache is `d91d6df12172`; refresh with
   `claude plugin marketplace update linkuistics` then
   `claude plugin update grove@linkuistics`, and verify installed content.
   Advancing the default checkout also updates linked kind skills, so defer
   that advance until the other loops are confirmed stopped.
7. The 21.0.0 rehearsal cut and all three release archives build successfully
   using the repository's release script in an isolated colocated checkout at
   `/tmp/grove-k10-release-rehearsal`. This avoids advancing the default checkout
   and thereby refreshing linked skills before stop confirmation. Its local-only
   tag is on `acb1c097732f12d27b80f0ba20ba58d783e8f77b`; do not push that
   unrelated rehearsal history. It differs from the preparation source only by
   the intended root workspace version, six lockfile versions and changelog
   heading. No book reconstructs the root workspace manifest. The actual release
   cut must still run in the default workspace under `docs/RELEASING.md`.
8. Recovery evidence is preserved outside the repository at
   `/Users/antony/.local/share/grove-cutovers/node-cutover-k10-20260912/`:
   `preflight.tar.gz` holds original trees, mapped scratch copies, per-file hashes,
   selection/chain results and the one-off conversion script;
   `api-refusal.json` pins the new reader's missing-file refusal;
   `rehearsal-dist/` holds inspected archives/formula and `release-build.log`
   records the successful build. These are preparation snapshots, not a licence
   to overwrite trees that continue moving. Re-enumerate and snapshot again after
   stops, comparing each entry rather than just totals.
9. Both binaries extracted from the macOS rehearsal archive report 21.0.0.
   That archive's reader preserves each passing selection: InTheLoop k50,
   Modaliser k44, Writegood k22, this tree k10 and grove.gh-issue-12 k27.
   APIAnyware's old reader selected k823 during this snapshot; the new reader
   refuses missing k112 before selection. A projected default-workspace copy
   passes, and scratch retirement of k10 selects `grove-migration-k11`.
   No terminal tree was found and no selection was fabricated.

### Prepared publication and cutover plan (approval outstanding)

**Source and version.** Publish the history from `main` at `d91d6df1` through
`9278f021`, the preparation change `mowsyyrq` (keep k10 live), then the real
`chore: release v21.0.0` cut. The actual release commit/tag does not yet exist
in the shared repository: the rehearsal proves the specified version edit and
build, not that a release has been cut or published. Before execution, snapshot
this live leaf's final preparation text and record the resulting commit SHA.

**Reviewable artifacts.** Rehearsal SHA-256 values (the final default-workspace
build must record its own values; archive timestamps can differ):

- `grove-v21.0.0-aarch64-apple-darwin.tar.xz`:
  `f1935e2a2ef896ca8f1c04f4479be2a66975f80168708bd277bc136dbe62989d`
- `grove-v21.0.0-aarch64-unknown-linux-gnu.tar.xz`:
  `49f4b3886ed068809f44f4689999268ecc906efea6dec0b7013b0120a18ad605`
- `grove-v21.0.0-x86_64-unknown-linux-gnu.tar.xz`:
  `d50941a42e2fef2ad3fa2ddefdd11d1b092a5d8eae201712ae56ab5ba324bcc4`
- Generated `grove.rb`:
  `8f151d29e008e498a3c2f6364df52be500fd36381bfb34b2283d80e558f95858`

Each archive carries `grove`, `grove-llm`, LICENSE and README. The formula names
21.0.0, all three matching release URLs and checksums. The plugin ships from the
same source release commit. Changelog's complete Unreleased entry already covers
the grammar, strict refusals and paired binary/plugin installation.

**Before execution.** Obtain approval for this named 21.0.0 plan and explicit
confirmation that all other Grove loops are stopped (including newly found
Modaliser). Resolve APIAnyware k112's missing brief; the human has been asked
whether to authorize a minimal body identifying the existing review chain or
provide an original to restore. Neither option has yet been authorized. No
publication, installation or live conversion occurs until these conditions hold.
After a preparation stop, restart the installed 20.2.0 `grove` here with this
leaf still live. That restart is not approval; the resumed session rechecks the
recorded confirmations and the plan.

**Workspace and publication sequence after approval.** Recheck both workspaces
and preserve unrelated changes. Snapshot/describe this preparation change without
retiring k10, then in `~/Development/grove` use `jj new <preparation-SHA>` to
materialize it on an empty default change. Move `main` to the preparation change;
run `cargo release major` dry-run, inspect, then the executed cut as documented.
Verify the real release change contains only root Cargo.toml, six Cargo.lock
versions and the changelog heading. Inspect imported jj history, move `main` to
that release, park default's empty working copy on it, and confirm Git HEAD is
exactly tagged v21.0.0. Run release doctor, full checks and
`scripts/release-build.sh` there, inspect artifacts and record final hashes.
Publish `jj git push -b main`, the explicit `git push origin v21.0.0` tag
exception, and the GitHub Release/archives. The Homebrew tap is itself jj-enabled
and clean: its `main` is `b2ef6f9f`, current change `susqrnpp`. Use the publish
script's constituent operations for that tap so its commit/bookmark/push go
through jj, not the script's raw Git mutation: copy the formula, describe
`grove v21.0.0`, point `main` to it, seal, then `jj git push -b main`.
Never rerun an already-successful GitHub Release creation after a later failure.

**Install and plugin refresh.** `brew update`, `brew upgrade grove`; verify
resolved paths and both versions. Refresh the Claude marketplace/plugin by the
commands in decision 6. Preserve the real Codex spine directory in the recovery
folder, replace it with the default checkout's shipped spine link, and run that
checkout's `plugins/install.sh`; inspect any analogous real-directory conflicts
and preserve them before replacement. Verify bootstrap/format bytes and the full
installed Grove skill set against the release source before restarting a harness.
The install script must run from the default checkout, never with `--force` here.

**This tree's exact renames.** Preserve a fresh original and collision-checked
mapping. Rename `.grove/BRIEF.md` to `.grove/_BRIEF.md`, rename
`.grove/08-distinguished-names-k6/BRIEF.md` to `_distinguished-names.md` in that
directory, then rename its directory to `.grove/08-k6/`. Every leaf name and body,
node key, heading and order is preserved. The preflight mapping enumerates all
moved descendant paths as well. Deeper trees use the same file-first,
bottom-up-directory order. Verified obsolete root FORMAT witnesses are removed
only with original bytes retained (observed in APIAnyware, Writegood and
`grove.gh-issue-12`); unrelated contents remain intact.

**Completion boundary.** In this secondary workspace, create an empty change on
the real release commit (after checking the preparation edits are ancestors),
perform the authorized conversion, and verify installed-reader handle resolution,
brief chains and next selection with `GROVE_SIGNAL_FILE` cleared for administrative
probes. Retire k10 using the installed new binary only after release/install/
conversion verification. Describe and seal that change with conversion and
retirement together. Advance the default workspace with `jj new <sealed-cutover>`
and verify its actual converted tree with the installed reader. The tag stays on
the release cut. Other trees remain stopped for `grove-migration-k11`, whose
inventory must include Modaliser and the default copy. This session's mandate is
20.2.0: after eventual completed cutover, return without a completion signal and
have the human start the newly installed driver here. At preparation alone,
leave k10 live, snapshot with `jj status`, and return without signalling.

10. Verification completed: `bash scripts/check.sh` exits 0, all eight principal
    checks pass, all six books reconstruct fully. The refusal assertion was
    first observed failing with the old diagnostic, then the updated existing
    brief-related lifecycle tests passed. Full check output is `check.log` in
    the recovery folder. A hash manifest of crates, plugins, scripts, docs and
    root manifests/config/glossaries remained unchanged through the final check.
    Rehearsal source comparison confirms only CHANGELOG.md, Cargo.lock and
    Cargo.toml differ from the current source outside the task tree, exactly
    the version-cut edits. The recovery tar archive was verified file by file
    against every original-tree digest. It is archived rather than copied into
    a deeper backup directory because APIAnyware's paths exceed PATH_MAX when
    that prefix is added; extract it under a short temporary path.
11. Preparation handoff: no live tree was renamed or retired, no installed
    binary/plugin was replaced, and nothing was published. Approval, other-loop
    stop confirmation and k112 brief disposition remain outstanding. Keep this
    handle live and return without `grove-llm complete`; the next session must
    revalidate these snapshots because other drivers were still active.
12. On 2026-09-13 the human replied “yes” to approval of the named 21.0.0
    cutover plan, confirmation that all other Grove loops are stopped, and
    authorization for a minimal APIAnyware k112 brief. All three confirmations
    are now recorded; do not ask again. Revalidate snapshots before execution.
13. Authorized k112 repair written as a minimal brief identifying existing
    k113/k114/k115 implementation, review and integration. Refreshed inventory
    passes all six trees: APIAnyware k870, InTheLoop k50, Modaliser k110,
    Writegood k22, this tree k10 and grove.gh-issue-12 k27. Every old/new
    selection matches, and every mapped file digest matches. New snapshots and
    mapping are in `/tmp/gk10-approved`, archived at
    `/Users/antony/.local/share/grove-cutovers/node-cutover-k10-20260913/approved-preflight.tar.gz`.
    Release doctor passes; repository source matches the previously checked
    source digest manifest. The default and tap workspaces are still empty.
14. Approved preparation is sealed as `979b37b2171068619f240a6b37ea8390458d31d4`.
    Default-workspace dry-run and executed `cargo release major` created release
    `a50b9345000e38c4157854be6c6c6aeaec8462ea`, tagged `v21.0.0`. Inspected diff
    is exactly the root workspace version, six lockfile versions and changelog
    heading. Both workspaces now descend from that release; default Git HEAD
    exactly matches the tag. Full release checks run before publication.
15. Final archives built in the default workspace with `COPYFILE_DISABLE=1`
    after inspection exposed macOS `._*` metadata in the first packaging run.
    Rebuilt archives contain exactly both binaries, LICENSE and README. Both
    macOS binaries report 21.0.0; that archive's reader passes all seven scratch
    copies, including the release-created default tree. Final SHA-256 values:
    macOS `b6b696d27bc0e24e8fb2dac1192f3aa10f2037699647ce77188957b49c21a1fd`;
    Linux ARM `b98bb50ec8c4ac06be80cde16d39875a52962074e965bc4d834dfdf1e5b9cc16`;
    Linux x86 `b1072c65299efeb04f97f4539e1252e5d597ec92f5f4cb569a8d96aff91861ad`;
    formula `519d127f74e2c7f41fb7a0a0e44e6e8d7da23135783daf140d85d47a26c3e202`.
    The formula carries each matching archive hash. Full filenames, preflight
    outputs and build log live in the 20260913 recovery folder.
16. Full checks against release a50b9345 exit 0: all eight principal checks and
    all six books pass. The source digest manifest remains unchanged before and
    after the complete run. Evidence is `release-check.log` and
    `release-checked-source-sha256.json` in the 20260913 recovery folder.
    Publication proceeds under the approval recorded in decision 12.
17. Published main and tag v21.0.0 at a50b9345, then created
    `https://github.com/Linkuistics/grove/releases/tag/v21.0.0` with all three
    archives. Published the matching Homebrew formula through jj as tap commit
    `b9fc2167`. These operations succeeded; do not recreate the GitHub Release.
18. Homebrew install succeeds; `/opt/homebrew/bin/grove` and `grove-llm`
    resolve into Cellar 21.0.0 and match the published macOS archive byte for
    byte. GitHub's three asset digests match the final local hashes. Claude's
    `grove@linkuistics` now records a50b9345; every installed Grove skill file
    matches release source. Codex and Pi's full Grove skill sets resolve into
    the default checkout and match too. Both real spine directories were
    preserved in `installed-skill-backups` before replacement; the default
    checkout's install script exits 0. Other trees remain stopped for k11.
19. This workspace's actual tree is converted. `this-tree-before.tar.gz` and
    `this-tree-mapping.json` in the recovery folder preserve every original byte
    and mapped path; comparison after conversion is exact. Installed handle
    resolution finds k10 and node k6; k19's brief chain lists `_BRIEF.md` and
    `08-k6/_distinguished-names.md`. Installed-reader scratch retirement selects
    k11. No ADR changes or parent close are needed: migration remains live.
    Seal conversion and retirement together, then advance and verify the default
    copy. This mandate is 20.2.0: return without signalling; the human starts
    installed Grove 21.0.0 here for k11. Include Modaliser and both Grove copies
    in k11's inventory, with other drivers stopped until their trees verify.
