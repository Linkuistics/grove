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
