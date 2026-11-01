# Coordinated Grove and Ravel release

The owner acknowledged that this is the sole live workstream through cutover.
This design selects the combined release and awaits its focused design review.
Implementation and publication remain ahead. This document reports no release.
[Evidence and task ownership](../coordinated-release-evidence.md) separate the
observed inputs from remaining work.

## Release boundary

Publish one complete Ravel 2.0.0 containing Rhai configuration and directory
delivery. Publish it first through keg-only `ravel@2`. Keep unversioned `ravel`
at 1.1.0 until a compatible Grove release is public and verified. Grove's final
integration and minor release belong to its driver-created finish session and
that session's explicit confirmation.

Ravel's release-channel contract owns publication mechanics. This document owns
ordering between products. The [delivery design](codex-skill-provisioning.md)
owns Grove's behaviour. A version predicate alone proves no compatibility:
caller assets, settings, installed layouts, confinement, results and skill
reads must work together.

Combining the changes makes Rhai publication wait for directory acceptance and
unfinished Rhai book work. Separate releases reduce that wait but add another
contract transition and release campaign. Reconsider if a native-only 2.0.0 cut
exists at execution time. Never overwrite it or present it as the combined
release. An earlier Grove compatibility release adds its own integration and
release obligation and is not part of this route.

## Public identities

| Contract | Identity and constraint |
| --- | --- |
| Released TypeScript route | Ravel 1.1.0 with request/policy schema 2. Preserve its complete installation and settings for old callers. |
| Landed, unreleased Rhai route | Script/request schema 3, worker protocol 1, selection-context schema 1, and routing and failure schemas 2. Its manifest still says 1.1.0. That does not make its changed behaviour a released 1.1.0 contract. |
| Combined release | Ravel 2.0.0, script/request schema 4. The directory design owns exact fields and rejection or explicit compatibility for schema-3 scripts, including delivery disabled. |
| Other schema families | Do not change protocol, context, routing or failure versions to match the product major. The directory design owns any justified change and encoded-frame bounds. |
| Compatible Grove | Next minor after the version on integrated main at the cut. Do not reserve 22.9.0 while main can advance. Verify native 1.1 and combined 2.0 routes independently. |

The historical TypeScript directory proposal's schema 3 is not an implementation
target. No local-only or native-only subset publishes as combined 2.0.0. If a
partial cut or public 2.0.0 appears, reconcile ownership and state first. A later
directory release needs a reviewed identity decision. Do not assume that it is a
compatible minor increment.

## Compatible tap states

| State | Published formulae | Fresh install and upgrade behaviour |
| --- | --- | --- |
| A: current | Grove 22.8, unversioned Ravel 1.1.0 | Existing native delivery and TypeScript assets. |
| B: Ravel first | Same pair, plus keg-only `ravel@2` 2.0.0 | Grove still selects 1.1.0. Opt-in 2.0 uses its explicit prefix and separate Rhai settings. Installing `ravel@2` changes no default command. |
| C: compatible Grove | New Grove, unversioned Ravel 1.1.0 and `ravel@2` 2.0.0 | Keep Grove's unversioned dependency. Native setup works by default. Directory setup explicitly selects `ravel@2`. Verify both before advancing. |
| D: promotion | Same compatible Grove. Unversioned Ravel and `ravel@2` reference identical 2.0.0 archives | Fresh installs get a compatible pair. The owner upgrade installs and verifies compatible Grove before switching Ravel or settings. Keep `ravel@2`. Promotion neither rebuilds nor cuts a version. |

Formula tests use the selected dependency's prefix. Keg-only installation does
not select Ravel for Grove. The caller design supplies explicit launch examples.
No formula state advertises an incompatible dependency as usable.

Promotion needs an upgrade rehearsal from A and C, including a Ravel-only
upgrade with outdated Grove installed. Homebrew normally updates outdated
dependents. Observe that behaviour for these formulae rather than assume it.

Pinned Grove, `HOMEBREW_NO_INSTALLED_DEPENDENTS_CHECK`, failed upgrades and deliberately kept
old archives require an explicit legacy route. Publish the remedy before
promotion: upgrade Grove first, or retain and explicitly select the complete
1.1.0 prefix and its settings. If the supported upgrade cannot preserve the
pair, stay in C and repair packaging. A warning alone is not acceptance.

## Publication ownership and order

1. Retain the acknowledged single-owner handoff. This Grove workstream alone
   executes release, upgrade and owner cutover. The other workstreams remain
   paused through completion. No session edits their task trees. When they
   resume, they must consume the actual release receipts. They must not repeat
   publication or owner conversion.
2. Keep directory development on its separate jj line through local, pinned,
   offline, lifetime, confinement and actual-harness acceptance. Inspect the
   exact Rhai book/read state. In a separate Ravel workspace, this grove
   completes every outstanding documentation or acceptance prerequisite for the
   release. It reconciles that work with the directory documentation. It does
   not wait for another workstream to resume. It does not mark that
   workstream's editorial goals done.
<!-- tc: depart synonym-rotation because "check" names the repository check suites, which are tool names, not a second verb for verify -->
3. Rollout owns Ravel's final rebase, full checks, main landing and release from
   its default colocated workspace. Implement and verify the versioned channel
   first. Reinspect manifests, tags, releases, book and tap before any cut.
4. Cut 2.0.0 once. Verify the tagged book and all three archives, including
   target smoke tests, before pushes. Publish source/tag, GitHub assets,
   versioned book, then only `ravel@2`. Verify its installed prefix, formula
   behaviour and live book. A tag alone is not completion.
5. Rollout verifies the final source-pack pins and working Grove against the
   published Ravel archives. Complete owner delivery under its separate design,
   preserving this driver's readers. Supply finish with exact receipts and any
   expressly assigned final activation steps.
6. Finish receives its one confirmation for the concrete whole sequence. It
   preserves artifacts and commits the helper-owned teardown. It integrates
   current main, then runs the integrated Grove check and its normal
   minor-release task. Verify both admitted Ravel routes. Publish Grove while
   unversioned Ravel is 1.1.0.
7. After successful Grove publication and installed verification, finish runs
   the prepared and rehearsed Ravel promotion operation. Publish the same
   archives as unversioned Ravel, verify the pair, and perform only the owner
   activation steps assigned to finish. This is an explicit finish extension,
   not an ordinary leaf after the sentinel. Incomplete promotion or activation
   means no successful finish signal.

Rollout planning grows ordinary release-tool preparation, Ravel integration,
publication and verification tasks. Rollout implements and rehearses the
promotion operation before finish. Finish is not a place to design it.

## Activation and running readers

Publication, installation and activation are separate events. Keep the full
original Ravel 1.1.0 keg, including worker/runtime resources, at its original
path. Preserve every inherited ending command. Suppress automatic cleanup for
every Homebrew mutation, including Grove's task. Do not explicitly clean up,
autoremove, reinstall or uninstall retained resources.

Keep legacy settings usable for every remaining 1.1 driver launch. Different
binary prefixes or configuration filenames do not isolate a shared settings
file. Prepare the Rhai environment in a separate HOME until no remaining old
launch depends on shared settings. The owner-delivery design accounts for
credentials, hooks and other settings. This design implies no credential copy.

Activate new launches only after the selected pair and full skill collection
pass the owner trial. Remove native discovery entries only after replacement
reads pass, retaining resources for lazy readers, including finish. A recipe
is not actual cutover: record the activated default launch path and remaining
readers. The owner-delivery design settles how these requirements are met.

A running driver retains its resolved executable. This driver keeps its 1.1.0
ending command. Restart means a new driver after the old driver and managed
group end. No leaf signals another workstream. Old-keg cleanup can occur only
after the reap of its last driver, outside that driver's finish session. An
inactive retained keg does not prevent cutover completion.

## Recovery boundaries

Use each repository's owning release procedure and observed state. Retain the
selected formula channel on every retry. Never bump again to finish a cut.

| Last completed boundary | Next behaviour |
| --- | --- |
| No cut | Repair prerequisites. Recheck ownership before preparation. |
| Local version/tag | Reconcile and verify that exact cut. Do not bump again. |
| Public source/tag | Keep its identity immutable. A source correction needs an explicit new-release decision. |
| Partial GitHub assets | Verify existing bytes. Upload only missing verified assets. |
| Assets complete | Resume the tagged book before the formula. |
| Book complete | Resume only the selected formula after verifying downloaded assets. |
| Tap commit exists | Preserve it and retry its push. Do not recopy into dirty work. |
| Installation/live-book failure | Retry that step only, retaining published identities and old binaries. |
| Grove release partial | Follow Grove recovery, without another minor bump. Keep unversioned Ravel at 1.1.0. |
| Promotion partial | Resume its formula/installation operation. Cut neither Ravel nor Grove again. |

## Acceptance and limits

Extend the agreed public-launch, release-lifecycle and real formula seams.
Controlled remotes prove channel selection and retry ordering. Installed
prefixes prove lifecycle and confined standalone behaviour. Actual Claude Code
and Codex trials prove discovery and reads. Fixtures do not prove public
publication or owner activation.

Rehearse A–D fresh installs and upgrades. Verify the old ending path after each
package mutation. Controls must fail on wrong-channel selection, mismatched
archives, silent schema-3 reuse, removed retained resources and promotion before
compatible Grove. This design introduces no new runtime API or registry.
