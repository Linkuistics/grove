# Coordinated release evidence and ownership

The [release design](specs/coordinated-release.md) awaits its focused review.
The owner acknowledged sole execution in this workstream through completion.
No release, tap edit, owner activation, native removal or other-session signal
occurred in this design session.

## Acknowledged release owner

The owner stated: "this is the only workstream that is currently live, and will
stay that way until we are finished because of the difficulty of our cutover."
Together with the earlier instruction to do Ravel release work in this grove
and coordinate both releases, this establishes one executor. This grove's
rollout tasks own Ravel integration/publication and owner cutover; its
driver-created finish owns final Grove release and the prepared promotion.
No message, tree edit or signal was used to impose a hold on another workstream.

The design selects combined 2.0.0 through `ravel@2` first. This grove completes
all unfinished Rhai documentation and acceptance prerequisites needed to ship;
it cannot depend on the paused workstream resuming. Its old release, upgrade
and configuration tasks must consume these receipts when that workstream is
eventually resumed, rather than repeating execution. Its other goals are not
declared complete by this design.

## Observed baseline

Observed on 2026-11-02 through local source and read-only Git/GitHub queries.
Recheck moving facts before execution.

| Subject | Observation |
| --- | --- |
| Ravel main | Local and remote: `24a545236185b0910ba6117fe8f9e8b2e2c8a748`. Rhai native behavior and script/request schema 3 are landed; the manifest remains 1.1.0. |
| Historical directory design | Default workspace holds the separate design line ending at `b93bb927095ff09b91e9b78dc1ad05bdacfec649`, not main. Its TypeScript/schema-3 feature contract requires k34's repair. |
| Rhai workstream | `Ravel.gh-issue-2` is clean at empty change `069f5cf9`, above `78f8ba78` (reader checkpoint). Release k37, upgrade k38, owner-policy k39 and book/read work remain live. This session edited none of that tree. |
| Release evidence | GitHub lists only 1.1.0, 1.0.0 and 0.1.0, no drafts. Local and remote tags have no v2 cut. These checks found no 2.0 publication; they cannot prevent another operator starting one. |
| Installed pair | Homebrew reports Grove 22.8.0 and Ravel 1.1.0. The original driver uses `/opt/homebrew/Cellar/ravel/1.1.0/bin/ravel`. |
| Tap | Local/remote main: `77e034ce09038924ca8131121fb2ac06221e2127`. Grove depends on unversioned `ravel` 1.1.0. The observed remote listing contains no `ravel@2`. |
| Tap path | Maintained clone: `/Users/antony/Development/Delivery/homebrew-taps`. Supply `RAVEL_TAP_DIR` and `GROVE_TAP_DIR`; the scripts' default `~/Development/homebrew-taps` is absent. |
| Local release files | Old default-workspace archive/dist directories exist; they are not 2.0 receipts. No release lock was present. |
| Pages | API reports a built site at `https://linkuistics.github.io/Ravel/`, using `gh-pages` at `/`. Verify the release-specific book at publication; no administrative setup change is presently indicated. |

## Source evidence

Grove's graph-discovered `crates/grove/src/dispatch/ravel.rs::check_version`
admits stable 1.x from 1.1.0. Its caller is `locate_on_path`; exact source
confirms the predicate. Generation `2026-11-01T13:51:14Z` reports no recorded
gap and matching metadata for that file, the Taskfile, release guide and delivery
spec. This is bounded Verify-tier evidence. Some outbound edges resolve common
method names to unrelated symbols; no conclusion relies on those edges.

Ravel source was read at exact main through `jj file show`: `Taskfile.yml`,
`Cargo.toml`, `release.toml`, `scripts/release-prepare.sh`,
`scripts/release-publish.sh`, `scripts/release-formula.sh`, the configuration
specification and release chapter. The nearest Rhai-workspace graph generation
is `2026-10-29T08:59:41Z`; docs/scripts are excluded. Direct source reads supply
that evidence. No graph-completeness claim is made for Ravel main.

The current release task hard-codes unversioned publication: baseline checks
read `Formula/ravel.rb`, rendering emits `ravel.rb`, publication copies it to
that formula, installation upgrades `ravel`, and verification selects the
shared-prefix command. Channel selection must reach every consumer and resume
path. Renaming one file is insufficient.

Ravel's contract is `docs/specs/release-channels.md`, in the isolated
jj workspace `Ravel.coordinated-release-design` based on exact main above.
It is committed as `80e382223129cc659783b78e6e93bd6d3e391739`, on local bookmark
`coordinated-release-design`. The workspace has an empty successor. The commit
is not on main and was not pushed; k34 and rollout must reconcile it into their
owning Ravel line before implementation/publication. It changes no release
executable. Grove's design also adds a prepared promotion operation to finish's
handoff; no such task currently exists.

Homebrew's [versioned-formula convention](https://docs.brew.sh/Versions) supports
keg-only coexistence. Its [manual](https://docs.brew.sh/Manpage) describes
outdated-dependent updates and their disable switch. Actual upgrade rehearsal
is still required; this is no atomic-upgrade guarantee for these formulae.

## Task ownership and handoff

| Owner | Responsibility |
| --- | --- |
| `coordinated-ravel-release-k36` | Ordering, identity constraints, Ravel channel contract, acknowledgement and focused design review. |
| Paused Rhai workstream maintainer | On eventual resumption, reconcile k37/k38 and owner-config k39 with this grove's actual receipts. This grove edits none of its tree or marks its other goals complete. |
| `ravel-rhai-skill-delivery-k34` | Consume the reviewed release choice; settle schema-4 fields, native/schema-3 compatibility, Rhai host reads, settings and frame bounds against exact current main. |
| `grove-rhai-skill-delivery-k35` | Map reviewed upstream behavior into Grove assets and acceptance; reconcile obsolete schema/runtime claims. |
| Local/pinned and acceptance stages | Complete feature and actual-harness evidence on a separate Ravel line; no partial publication. |
| Owner-delivery k37 and final inventory k38 | Actual owner activation, settings isolation, native removal, TestAnyware, final-session guidance and retained readers. |
| Rollout planner k12 under k6 | Grow remaining Rhai release prerequisites, release-tool implementation/rehearsal, Ravel integration/publication/verification and owner cutover. Supply working promotion tooling and exact finish handoff. |
| Driver-created Grove finish | One confirmation for final integration/minor release, prepared promotion and assigned activation. Retain original ending command and teardown contract. |

The acknowledgement above settles ownership; the focused review still checks
the design. Implementation and release sessions verify current source and public
state before acting. The acknowledgement is not a completion receipt.

## Design verification

Local links and anchors in the new specifications/evidence resolve, the viewer
manifest names existing unique diagrams and groups, and whitespace checks pass
in both workspaces. Both new diagrams rendered in Safari after an initially
unavailable connection. Fresh deep links reached the intended views, and the
topic outline and current/updated markers matched the source. Inspection covered
desktop light appearance; no mobile or dark appearance was measured.

The final Grove coverage check reports generation `2026-11-01T14:10:52Z` and
matching metadata for the changed prose. Both new Mermaid files are reported
as partial over their whole ranges; their exact source was read and their
rendered result inspected. No graph interpretation of those diagrams is used.

These are design/document checks. No release-tool implementation, product test,
installed compatibility campaign or publication is claimed by this leaf.
