# Coordinated release evidence and ownership

The [release design](specs/coordinated-release.md) received its focused review,
and this grove integrated the findings. The owner acknowledged sole execution in
this workstream through completion.
This design session made no release, tap edit, owner activation or native
removal, and it sent no signal to another session.

## Acknowledged release owner

The owner stated:

<!-- tc: off because this block is the owner's exact quotation -->
> this is the only workstream that is currently live, and will stay that way
> until we are finished because of the difficulty of our cutover.
<!-- tc: on -->

The earlier instruction was to do Ravel release work in this grove and
coordinate both releases. Together, the two statements establish one executor.
This grove's rollout tasks own Ravel integration/publication and owner cutover.
Its driver-created finish owns final Grove release and the prepared promotion.
No message, tree edit or signal imposed a hold on another workstream.

The design selects combined 2.0.0 through `ravel@2` first. This grove completes
all unfinished Rhai documentation and acceptance prerequisites for the release.
It cannot depend on the paused workstream resuming. When that workstream
resumes, its old release, upgrade and configuration tasks must consume these
receipts rather than repeat execution. This design declares none of its other
goals complete.

## Observed baseline

Observed on 2026-11-02 through local source and read-only Git/GitHub queries.
Verify moving facts again before execution.

| Subject | Observation |
| --- | --- |
| Ravel main | Local and remote: `24a545236185b0910ba6117fe8f9e8b2e2c8a748`. Main holds the Rhai native behaviour and script/request schema 3. The manifest remains 1.1.0. |
| Historical directory design | The default workspace holds the separate design line ending at `b93bb927095ff09b91e9b78dc1ad05bdacfec649`, not main. Its TypeScript/schema-3 feature contract requires k34's repair. |
| Rhai workstream | `Ravel.gh-issue-2` is clean at empty change `069f5cf9`, above `78f8ba78` (reader checkpoint). Release k37, upgrade k38, owner-policy k39 and book/read work remain live. This session edited none of that tree. |
| Release evidence | GitHub lists only 1.1.0, 1.0.0 and 0.1.0, with no drafts. Local and remote tags have no v2 cut. These queries found no 2.0 publication. They cannot prevent a later cut by another operator. |
| Installed pair | Homebrew reports Grove 22.8.0 and Ravel 1.1.0. The original driver uses `/opt/homebrew/Cellar/ravel/1.1.0/bin/ravel`. |
| Tap | Local/remote main: `77e034ce09038924ca8131121fb2ac06221e2127`. Grove depends on unversioned `ravel` 1.1.0. The observed remote listing contains no `ravel@2`. |
| Tap path | Maintained clone: `/Users/antony/Development/Delivery/homebrew-taps`. Supply `RAVEL_TAP_DIR` and `GROVE_TAP_DIR`. The scripts' default `~/Development/homebrew-taps` is absent. |
| Local release files | Old default-workspace archive/dist directories exist. They are not 2.0 receipts. No release lock was present. |
| Pages | The API reports a built site at `https://linkuistics.github.io/Ravel/`, using `gh-pages` at `/`. Verify the release-specific book at publication. No administrative setup change is presently indicated. |

## Source evidence

Grove's graph-discovered `crates/grove/src/dispatch/ravel.rs::check_version`
admits stable 1.x from 1.1.0. Its caller is `locate_on_path`. The exact source
supports the predicate. Generation `2026-11-01T13:51:14Z` reports no recorded
gap and matching metadata for that file, the Taskfile, release guide and
delivery spec. This evidence has the bounded Verify tier.

Some outbound edges
resolve common method names to unrelated symbols. No conclusion relies on
those edges.

The design session read Ravel source at exact main through `jj file show`:
`Taskfile.yml`, `Cargo.toml`, `release.toml`, `scripts/release-prepare.sh`,
`scripts/release-publish.sh`, `scripts/release-formula.sh`, the configuration
specification and the release chapter. The nearest Rhai-workspace graph
generation is `2026-10-29T08:59:41Z`, and it excludes docs and scripts. Direct
source reads supply that evidence. This document makes no graph-completeness
claim for Ravel main.

The current release task hard-codes unversioned publication. The baseline step
reads `Formula/ravel.rb`, and rendering emits `ravel.rb`. Publication copies it
to that formula, installation upgrades `ravel`, and verification selects the
shared-prefix command. Channel selection must reach every consumer and resume
path. Renaming one file is insufficient.

Ravel's contract is `docs/specs/release-channels.md`, in the isolated jj
workspace `Ravel.coordinated-release-design` based on exact main above. Its
decision record is `docs/adr/a-caller-transition-publishes-a-versioned-formula-first.md`.
The reviewed commit was `80e382223129cc659783b78e6e93bd6d3e391739`. Above it,
`c94741960df5a62f569efae1d1cec3c05f0d54ee` converts the specification to
controlled English.

The integrated commit is `2a77ffbda3b2510f0997c740c0642b18afaeece7`, on local
bookmark `coordinated-release-design`. The workspace has an empty successor.
The commit is not on main, and nobody pushed it. k34 and rollout must reconcile it into
their owning Ravel line before implementation/publication. It changes no
release executable.

Grove's design also adds a prepared promotion operation to
finish's handoff. No such task exists now.

Homebrew's [versioned-formula convention](https://docs.brew.sh/Versions) supports
keg-only coexistence. Its [manual](https://docs.brew.sh/Manpage) describes
outdated-dependent updates and their disable switch. Actual upgrade rehearsal
is still required. This is no atomic-upgrade guarantee for these formulae.

## Review integration evidence

The integration verified each review finding against source on 2026-11-02.

| Subject | Observation |
| --- | --- |
| Configuration format | Ravel main's `crates/ravel/src/authority.rs` refuses a lone `policy.ts` with `config_format_retired` and exit 3. Its remedy names book chapter 4. The `Unreleased` changelog does not yet name the format change. |
| Owner and caller configuration | The owner's `~/.config/ravel` holds only `policy.ts`. The tap's Grove formula tells users to copy a `policy.ts` sample, and its test requires `ravel 1.`. |
| Teardown admission | `cmd_record_teardown` in `crates/grove-llm/src/cli.rs` requires session-epoch admission when `GROVE_LAUNCH_DIR` is set. |
| Release smoke pins | `scripts/release-smoke.sh` and `scripts/grove-installed-smoke.sh` hard-code Ravel 1.1.0 archive names and version. `docs/ravel-target-inputs.md` pins only 1.1.0. |
| Homebrew cleanup | The installed Homebrew 7.0.11 manual documents periodic full cleanup from install, upgrade and reinstall. It also documents `HOMEBREW_NO_CLEANUP_FORMULAE`, user `brew.env` files and deprecation that warns but proceeds. |
| Paused release leaf | `Ravel.gh-issue-2` leaf `release-ravel-2-0-0-k37` runs `task release:major`. `scripts/release-prepare.sh` bumps whenever the baseline is complete and main has changes. |
| Schema reason | `docs/grove-skill-delivery-design-evidence.md` gives the reason for a new script/request schema: prepared-prompt semantics change. |

## Task ownership and handoff

| Owner | Responsibility |
| --- | --- |
| `coordinated-ravel-release-k36` | Ordering, identity constraints, Ravel channel contract, acknowledgement and focused design review. |
| `coordinated-ravel-release-k39` and `-k40` | The focused review and its integration: two decision records, the post-promotion channel rule and the configuration-format remedy. |
| Paused Rhai workstream maintainer | On eventual resumption, reconcile k37/k38 and owner-config k39 with this grove's actual receipts. This grove edits none of its tree and marks none of its other goals complete. |
| Rollout, for the paused Rhai workstream | Supply brief-update text that carries the 2.0.0 release, promotion and owner-conversion receipts. It directs that workstream to prune or retarget k37, k38 and k39. A resumed k37 would otherwise cut 3.0.0. |
| `ravel-rhai-skill-delivery-k34` | Consume the reviewed release choice. Settle schema-4 fields, native/schema-3 compatibility, Rhai host reads, settings and frame bounds against exact current main. |
| `grove-rhai-skill-delivery-k35` | Map reviewed upstream behaviour into Grove assets and acceptance. Reconcile obsolete schema/runtime claims. |
| Local and acceptance stages | Complete feature and actual-harness evidence on a separate Ravel line, with no partial publication. |
| Owner-delivery k37 and final inventory k38 | Actual owner activation, settings isolation, native removal, TestAnyware, final-session guidance and retained readers. The owner's `config.rhai` conversion precedes any owner default launch on 2.0. |
| `grove-dual-ravel-k27` and rollout | k27 makes Grove's release and installed smoke admit a second independent Ravel beside 1.1.0. Rollout pins the published 2.0.0 archives by SHA-256 from the publication receipt before Grove's release. |
| Rollout planner k12 under k6 | Grow remaining Rhai release prerequisites, release-tool implementation/rehearsal, Ravel integration/publication/verification and owner cutover. Supply working promotion tooling and the exact finish handoff. |
| Rollout, for promotion readiness | The 2.0.0 changelog names the configuration-format change. Upgrade rehearsals include a HOME with only `policy.ts`. Machine-wide `HOMEBREW_NO_CLEANUP_FORMULAE` protection is in place before finish. The compatible Grove keeps the 22.8.0 launch-directory contract, or rollout delivers helper preservation. |
| Driver-created Grove finish | One confirmation for final integration/minor release, prepared promotion and assigned activation. Retain the original ending command and teardown contract. |

The acknowledgement above settles ownership. The focused review examined the
design, and its integration repaired the findings. Implementation and release sessions verify current source and
public state before they act. The acknowledgement is not a completion receipt.

## Design verification

Local links and anchors in the new specifications/evidence resolve. The viewer
manifest names existing unique diagrams and groups. `git diff --check HEAD`
passes in both workspaces. Both new diagrams rendered in Safari after an initially
unavailable connection. Fresh deep links reached the intended views, and the
topic outline and current/updated markers matched the source.

The inspection covered desktop light appearance. Nobody measured mobile or
dark appearance.

The final Grove `check_index_coverage` result reports generation `2026-11-01T14:10:52Z` and
matching metadata for the changed prose. It reports both new Mermaid files as
partial over their whole ranges. The design session read their exact source
and inspected their rendered result. No conclusion uses a graph interpretation
of those diagrams.

These results cover the design and its documents only. This leaf claims no release-tool
implementation, product test, installed compatibility campaign or publication.

## Ravel 2.0.0 publication receipt

ravel-two-publication-k313 cut Ravel 2.0.0 once, on 2026-11-12, and
published it through the keg-only `ravel@2` only. This is tap state B.
Unversioned `ravel` stays at 1.1.0. Grove 22.8.0 still selects it.

### The run

Before the cut, a fresh read found no `v2.0.0` tag on either side, no 2.0.0
GitHub release and no `ravel@2.rb` in the tap. Ravel `main` and its remote
were `12df84bebf4a`. The tap was clean at `12b53f8c449d`.
`task release:doctor` met every prerequisite.

The run used Ravel's default colocated workspace. It removed
`RAVEL_EXIT_FILE` and `GROVE_LAUNCH_DIR` from the environment:

```sh
env -u RAVEL_EXIT_FILE -u GROVE_LAUNCH_DIR PATH="/opt/homebrew/bin:$PATH" \
  RAVEL_TAP_DIR=/Users/antony/Development/Delivery/homebrew-taps \
  CHANNEL=versioned-2 HOMEBREW_NO_INSTALL_CLEANUP=1 task release:major
```

Every step through the installation passed. `task check` ran 713 tests, which
passed, and skipped 48. The task stopped at the evidence step with
"permission denied" and exit 127. Git records `scripts/release-evidence.sh`
with mode `100644`, and the Taskfile runs it with no `bash` prefix.

The recovery row "Release evidence failed" of book chapter 11 applied. The
resume ran `bash scripts/release-evidence.sh --channel versioned-2` at the
release commit, then `scripts/release-book.sh live v2.0.0`. Both passed. No second
version cut happened. ravel-evidence-step-invocation-k332 repairs the
Taskfile line on Ravel `main`.

### Published identities

| Subject | Identity |
| --- | --- |
| Channel | `versioned-2`, formula `ravel@2`, class `RavelAT2`, keg-only |
| Tag | `v2.0.0`, annotated object `d2f74e6d634369538a191dd1a7347b2c0563d392` |
| Tag commit | `ed6cedbf9a14cd088bea13fcfb8b86300cb6aaca`, also Ravel `main` on origin |
| GitHub release | Non-draft, published `2026-11-12T10:00:02Z`, three assets |
| Book commit | `eb704fbf0700d92b1b9a6a1d338782d6e1a061d0` on `gh-pages`, which serves `v2.0.0/` beside `v1.1.0/` |
| Tap commit | `f7f9b9b0bc4d6353dcd8cd41fb47deabafc14c09`, "ravel@2 v2.0.0". It adds only `Formula/ravel@2.rb`. `Formula/ravel.rb` stays at 1.1.0 |

Each asset has one SHA-256. Three measurements gave it: the archive in
`target/dist` that the smoke test ran, an anonymous download of the public
asset, and the `digest` that GitHub reports. The rendered `ravel@2.rb`
carries the same values.

| Asset | SHA-256 |
| --- | --- |
| `ravel-v2.0.0-aarch64-apple-darwin.tar.xz` | `5957840ab33f4331b794539dbd455d801f71c6c6485c781dc290cfb48490231d` |
| `ravel-v2.0.0-aarch64-unknown-linux-gnu.tar.xz` | `a4750ea439450e88621be411183e7bf30d2616f9a545b64a30445df3a9f02bee` |
| `ravel-v2.0.0-x86_64-unknown-linux-gnu.tar.xz` | `906e589f72161ed5dce5db683e6af10963c1952bdf177c6b343d1918928c3dd1` |

### Installed result

- `brew test ravel@2` passed inside the release task and again on its own.
- The keg is `/opt/homebrew/Cellar/ravel@2/2.0.0`, and the prefix
  `/opt/homebrew/opt/ravel@2` resolves to it. Its `bin/ravel --version`
  reports `ravel 2.0.0`.
- Homebrew records `ravel@2` as keg-only, with no linked keg.
- The installation record `share/ravel/installation.json` names
  `/opt/homebrew/share/ravel/skills`.
- `ravel` on `PATH` is `/opt/homebrew/bin/ravel`, and it reports 1.1.0.
- The served book at `https://linkuistics.github.io/Ravel/v2.0.0/` shows
  `ravel 2.0.0` in chapter 3. Chapter 11 has the section "Package a skill
  source for the default directory" and the section "Promote a versioned
  release".

The formula test prints `stop attempt … signal 9: Operation not permitted`
for each fixture session. The test still passes. The lines are diagnostics
of the stop path that ravel-stop-eperm-k302 repaired. They are not a
failure.

### The original driver's keg

An inventory before the cut recorded the path and SHA-256 of each file and
link in `/opt/homebrew/Cellar/ravel/1.1.0`. It also recorded the links
`/opt/homebrew/opt/ravel` and `/opt/homebrew/bin/ravel`. The same inventory
after the installation is identical. The keg holds 14 files. Its `bin/ravel`
reports `ravel 1.1.0`. Outside a session, `end-session grove-continue`
refuses with exit 1 and no result.

### Where the records are

Ravel's release evidence is `target/release-evidence/v2.0.0.json` in Ravel's
default workspace. Promotion reads it. The logs of the run, the resume and
the separate formula test are beside it. That directory is untracked.
`docs/ravel-v2.0.0-release-evidence.json` in this repository is a byte copy
of the JSON file, with SHA-256
`02a120ea4ac083db0ff6f3026749c88d68114052a671dc24cd1771a9def5ad5e`. If the
original is lost, give the copy to `task release:promote` as `EVIDENCE`.

## Homebrew cleanup protection

homebrew-cleanup-protection-k316 set the machine-wide protection for `ravel`
on 2026-11-12, before finish. The section "Activation and running readers"
of the [release design](specs/coordinated-release.md) requires it. The
owner's manual records why the protection exists and when the owner may
remove it. That record is the section "Homebrew cleanup protection" of the
Obsidian note "Maintenance and Recovery".

### The setting

`~/.homebrew/brew.env` holds comment lines and one setting,
`HOMEBREW_NO_CLEANUP_FORMULAE=ravel`. Before the change, no `brew.env`
existed in `/etc/homebrew`, `/opt/homebrew/etc/homebrew` or `~/.homebrew`.
No shell profile, mise configuration or environment set the variable, so
there were no existing entries to keep.
After a `mise bootstrap packages upgrade` released mise's history lock,
`mise dot track ~/.homebrew/brew.env` added a `[dotfiles]` entry with
`mode = "track"` to `~/.config/mise/config.toml`. mise saved checkpoint 137
for the file.

The `bin/brew` of Homebrew 7.0.13 reads three files at the start of each
command. The order is `/etc/homebrew/brew.env`, then
`$HOMEBREW_PREFIX/etc/homebrew/brew.env`, then the user file. The user file
is `$XDG_CONFIG_HOME/homebrew/brew.env` when that variable is set, else
`~/.homebrew/brew.env`. No shell profile sets `XDG_CONFIG_HOME`. The
launcher exports each line literally, so a value in the file replaces a
value from the shell. A `brew` command under another HOME does not read the
user file.

In `Library/Homebrew/cleanup.rb`, `clean!` rejects each formula that
`skip_clean_formula?` names before it cleans a keg. The periodic cleanup,
`periodic_clean!`, calls `clean!`. The install cleanup and `brew cleanup
ravel` use the same test. The test compares exact formula names and
aliases, so `ravel` does not protect `ravel@2`.

### The read-only query

After the change, `brew config` printed
`HOMEBREW_NO_CLEANUP_FORMULAE: ravel`.

### The control

`task owner:cleanup-protection:controls` runs
`scripts/homebrew-cleanup-control.sh`. The script reads the real prefix
only. It makes APFS clones of `Library/Homebrew`, the `linkuistics` tap and
the `ravel` 1.1.0 keg in a scratch prefix. A second clone of the keg
becomes an old version, 0.1.0. Each run uses `env -i` with a scratch HOME,
cache, logs and temp. Each run then makes the periodic cleanup due and
calls `Homebrew::Cleanup.periodic_clean!`.

Two guards come before each cleanup. The scratch `brew --cellar` must be
the scratch Cellar. Homebrew must also find the 0.1.0 keg eligible for
cleanup. The scratch HOME trusts the `ravel` formula for that second guard.
Without the trust, Homebrew 7 loads no formula of the tap and omits `ravel`
from the installed formulae without an error. The first run of the control
had no trust, and its run without the file kept the old keg too.

On 2026-11-12, the run with a copy of the owner's file saw the setting, and
the periodic cleanup kept `ravel` 0.1.0. The run without a file saw no
setting, and the same cleanup removed the 0.1.0 keg, 14 files of 64 MB. A
mutant file that named `ravel@2,zstd` made the control fail, because the
cleanup removed the old keg. After each run, the metadata of every entry
in `/opt/homebrew/Cellar/ravel` was unchanged. The real
`~/.homebrew/trust.json` and `/opt/homebrew/.git/config` also kept their
modification times from before the session.
