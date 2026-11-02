# Coordinated Grove and Ravel release

The owner acknowledged that this is the sole live workstream through cutover.
This design selects the combined release. Its focused design review and that
review's integration are complete. Implementation and publication remain ahead.
This document reports no release.
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
release. Ravel's decision record
`a-caller-transition-publishes-a-versioned-formula-first` states why `ravel@2`
publishes first. It also records the rejected earlier Grove compatibility
release.

## Public identities

| Contract | Identity and constraint |
| --- | --- |
| Released TypeScript route | Ravel 1.1.0 with request/policy schema 2. Preserve its complete installation and settings for old callers. |
| Landed, unreleased Rhai route | Script/request schema 3, worker protocol 1, selection-context schema 1, and routing and failure schemas 2. Its manifest still says 1.1.0. That does not make its changed behaviour a released 1.1.0 contract. |
| Combined release | Ravel 2.0.0, script/request schema 4. The reviewed directory design at Ravel `822c0ed2` adds `originalPrompt` and `skills`. A schema-3 script refuses as `unsupported_version`, also with delivery disabled. |
| Other schema families | Do not change protocol, context, routing or failure versions to match the product major. The reviewed directory design keeps worker protocol 1, selection-context schema 1 and routing and failure schemas 2. It sets the evaluate-frame bound. |
| Compatible Grove | Next minor after the version on integrated main at the cut. Do not reserve 22.9.0 while main can advance. Verify three routes independently: native 1.1, native 2.0.0 with delivery disabled, and 2.0.0 directory delivery. The [delivery design](codex-skill-provisioning.md#supported-routes) states each route. |

The historical TypeScript directory proposal's schema 3 is not an implementation
target. No local-only or native-only subset publishes as combined 2.0.0. If a
partial cut or public 2.0.0 appears, reconcile ownership and state first. A later
directory release needs a reviewed identity decision. Do not assume that it is a
compatible minor increment.

Schema 4 exists because a prepared prompt changes what `request.prompt` means
to a script. The directory design applied that test to schema 3, including the
case with delivery disabled, where the prompt does not change. Schema 3
refuses, because a settings edit must not change what an unchanged script
receives. The new number has a cost: every `SCHEMA_VERSION = 3` example,
fixture and book page on Ravel main moves to 4. The owner's `config.rhai`
conversion target moves with them.

## Compatible tap states

| State | Published formulae | Fresh install and upgrade behaviour |
| --- | --- | --- |
| A: current | Grove 22.8, unversioned Ravel 1.1.0 | Existing native delivery and TypeScript assets. |
| B: Ravel first | Same pair, plus keg-only `ravel@2` 2.0.0 | Grove still selects 1.1.0. Opt-in 2.0 uses its explicit prefix and separate Rhai settings. Installing `ravel@2` changes no default command. |
| C: compatible Grove | New Grove, unversioned Ravel 1.1.0 and `ravel@2` 2.0.0 | Keep Grove's unversioned dependency. Native setup works by default. Until promotion, directory setup explicitly selects `ravel@2`. Verify both before advancing. |
| D: promotion | Same compatible Grove. Unversioned Ravel and `ravel@2` reference identical 2.0.0 archives, and `ravel@2` is deprecated | Fresh installs get a compatible pair and the `config.rhai` sample. An upgrade moves the default command to 2.0.0, which refuses a HOME that holds only `policy.ts`. The owner writes and verifies `config.rhai` and compatible Grove before switching Ravel or settings. `ravel@2` stays installable at the promoted version and receives no later release. Promotion neither rebuilds nor cuts a version. |

Formula tests use the selected dependency's prefix. Keg-only installation does
not select Ravel for Grove. The caller design supplies explicit launch examples.
No formula state advertises an incompatible dependency as usable. After
promotion, unversioned `ravel` is the only publication channel for later cuts.

Promotion is also a configuration-format change for every default-channel user.
Ravel 2.0.0 reads only `config.rhai`, and it refuses a lone `policy.ts` with
`config_format_retired` and exit 3. Ravel's decision record
`configuration-script-uses-rhai` records that clean cutover. Before promotion,
publish the conversion remedy: the Ravel 2.0.0 changelog, book chapter 4, and compatible
Grove's caveats and samples for both formats. The two files coexist under one
HOME: a 1.1 launch reads `policy.ts`, and 2.0 reads `config.rhai`.

Promotion needs an upgrade rehearsal from A and C, including a Ravel-only
upgrade with outdated Grove installed. Homebrew normally updates outdated
dependents. Observe that behaviour for these formulae rather than assume it.
Each upgrade rehearsal includes a HOME that holds only `policy.ts`. It must show
the documented refusal and remedy, then a working launch after conversion.

Pinned Grove, `HOMEBREW_NO_INSTALLED_DEPENDENTS_CHECK`, failed upgrades and
deliberately kept old archives require an explicit legacy route. Publish the remedy before
promotion: upgrade Grove first, or retain and explicitly select the complete
1.1.0 prefix and its settings. A warning alone is not acceptance.

If the rehearsed upgrade cannot preserve the pair, stay in C and repair
packaging. Rollout makes that decision before `finish-commit`. Inside finish,
`.grove/` no longer exists, so only the recovery table's resume rows apply. A
failure there ends without a successful finish signal.

## Publication ownership and order

1. Retain the acknowledged single-owner handoff. This Grove workstream alone
   executes release, upgrade and owner cutover. The other workstreams remain
   paused through completion. No session edits their task trees. When they
   resume, they must consume the actual release receipts. They must not repeat
   publication or owner conversion.
2. Keep directory development on its separate jj line through local, lifetime,
   confinement and actual-harness acceptance. Inspect the
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
5. Rollout verifies the final
   [source-pack pins](owner-skill-delivery.md#pack-checkouts) and working Grove against the
   published Ravel archives. The `grove-core` pin can be final only after
   Grove's release. For that source, rollout verifies the accepted candidate.
   It shows that the final line has the same tree ID in the source directory.
   A committed document records that ID. Finish's owner switch compares
   the ID at the released commit before any change. With the published
   archives, rollout also records each source's accepted `resourceDigest`.
   The switch compares each real checkout with its digest before any change.
   Rollout admits
   the archives into Grove's pinned independent Ravel inputs by SHA-256,
   beside the retained 1.1.0 pins, before Grove's release. Complete owner delivery under its separate design,
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
   activation steps assigned to finish.
   [Finish promotes after the Grove release](../adr/finish-promotes-after-the-grove-release.md)
   records this finish extension. Incomplete promotion or activation means no
   successful finish signal.

Rollout planning grows ordinary release-tool preparation, Ravel integration,
publication and verification tasks. Rollout implements and rehearses the
promotion operation before finish. Finish is not a place to design it.

## Activation and running readers

Publication, installation and activation are separate events. Keep the full
original Ravel 1.1.0 keg, including worker/runtime resources, at its original
path. Preserve every inherited ending command. Suppress automatic cleanup for
every Homebrew mutation, including Grove's task. Do not explicitly clean up,
autoremove, reinstall or uninstall retained resources.

Suppression covers only the mutations that set it. Any other install, upgrade
or reinstall can run Homebrew's periodic full cleanup, which removes old
versions. After the owner's unversioned Ravel upgrade, the 1.1.0 keg is such an
old version. From that upgrade until the old driver's reap, add `ravel` to
`HOMEBREW_NO_CLEANUP_FORMULAE` and keep the existing entries. Set it where every
Homebrew invocation on the machine reads it, such as Homebrew's user `brew.env`.

Keep legacy settings usable for every remaining 1.1 driver launch. Different
binary prefixes or configuration filenames do not isolate a shared settings
file. Prepare the Rhai environment in a separate HOME until no remaining old
launch depends on shared settings. The
[owner-delivery design](owner-skill-delivery.md) accounts for credentials,
hooks and other settings. This design implies no credential copy.

Activate new launches only after the selected pair and full skill collection
pass the owner trial. Remove native discovery entries only after replacement
reads pass, retaining resources for lazy readers, including finish. A recipe
is not actual cutover: record the activated default launch path and remaining
readers. The owner's `config.rhai` must exist and pass before any owner default
launch reaches 2.0. Keep `policy.ts` while any 1.1 launch remains. The
[owner-delivery design](owner-skill-delivery.md#the-switch-inside-finish)
settles how these requirements are met.

A running driver retains its resolved executable. This driver keeps its 1.1.0
ending command. Restart means a new driver after the old driver and managed
group end. No leaf signals another workstream. Old-keg cleanup can occur only
after the reap of its last driver, outside that driver's finish session. An
inactive retained keg does not prevent cutover completion.

After Grove's release, finish records teardown through the newly installed
`grove-llm` under the 22.8.0 driver. The verb needs session-epoch admission
through the launch directory. The compatible Grove release therefore keeps the
22.8.0 launch-directory and admission contract unchanged. A release that
changes it must preserve the old helper, as the
[signal-contract cutover](../RELEASING.md#signal-contract-cutover) requires.
Rollout then delivers that preservation step.

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
| Promotion complete, owner switch native, partial, active or drifted, or complete before finish's signal | Keep every backup and any pending-activation record. Run the prepared switch again from its first step. Its third step finds the state from the layout and the latest complete backup. [Resuming the switch](owner-skill-delivery.md#resuming-the-switch) states each continuation. A changed switch plan runs only after its rehearsal, as [The rehearsed plan](owner-skill-delivery.md#the-rehearsed-plan) states. Cut and promote no Ravel again, and tear down nothing again. A Grove patch release follows only the next row. |
| Promotion complete, a core trial or a probe of a Grove kind skill failed on the released Grove | Repair `main` and publish a Grove patch release through its owning procedure. Then run the switch again from its first step. Its fourth step reads that release. Cut and promote no Ravel again. |

## Acceptance and limits

Extend the agreed public-launch, release-lifecycle and real formula seams.
Controlled remotes prove channel selection and retry ordering. Installed
prefixes prove lifecycle and confined standalone behaviour. Actual Claude Code
and Codex trials prove discovery and reads. Fixtures do not prove public
publication or owner activation.

Rehearse A–D fresh installs and upgrades. Verify the old ending path after each
package mutation. Controls must fail on wrong-channel selection, a
`versioned-2` cut after promotion, mismatched archives and silent schema-3
reuse. They must also fail on removed retained resources and promotion before
compatible Grove. This design introduces no new runtime API or registry.
