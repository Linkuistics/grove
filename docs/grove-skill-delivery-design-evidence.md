# Grove skill-delivery design evidence

The [delivery specification](specs/codex-skill-provisioning.md) defines the
adoption target. This document records inspected source, design evidence and
verification limits. It does not claim implementation or acceptance of that
target. The source observations below are against Grove parent `8d45d75e`.
`grove-native-provisioning-k21` later removed the provisioner, its build script
and its tests, so this document names those paths without links. The
[Rhai contract mapping](#rhai-mapping) records later observations against Grove
`314d464e` and Ravel `822c0ed2`.

<a id="upstream-correction"></a>
## The upstream correction

The initial input was Ravel design commit
`66aaec06f5c020215b9b0e4139c2ba2e0b969644`, read locally from the owning repository.
That design enumerates skill metadata, generates catalogs, refuses duplicate
names and exposes a catalog descriptor. It is historical input, not the final
contract for this workstream after the human's corrections:

<!-- tc: off because these two block quotations are the human's own words, quoted verbatim -->

> grove and ravel are decoupled with respect to skills - grove may name a skill,
> but it is up to the user to ensure that the skill is installed either within
> each harness, or via ravel's mechanism

> Ravel doesn't resolve "a" skill - it simply injects the skill directory path(s)
> into the prompt.

<!-- tc: on -->

Accordingly, Grove's design requires neither a catalog nor Ravel-based skill
installation. The agent finds skills in its native installation or supplied
paths. The correction delivered by `ravel-skill-directories-k15` replaces the
owning Ravel spec, ADR, glossary and visual contract in place. It supplies search
directories and direct mandatory-file references, with a retained directory-only
descriptor for recovery and delegates. Ravel does not parse skill metadata,
reject duplicate names or answer a skill lookup.

That correction kept local and exact pinned Git sources. Ravel's later
decision record "Skill sources are local directories" narrowed them to local
directories.

The producer's Ravel commit is `4c16f8e09d58c8f649a375029c8e8fa2933de2b4`.
Its `docs/specs/skill-discovery.md` defines the corrected interface. Its
`docs/skill-discovery-design-evidence.md` records source evidence and limitations.

That TypeScript design assigned the feature to Ravel 2 and a policy/request
schema 3, because prepared-prompt semantics change. Ravel main later gave 2.0.0
and schema 3 to Rhai native execution. Those assignments therefore supply
intent only. The reviewed Rhai contract below replaces them.

The directory review in Grove commit `ec3b0700e29c`, integrated by
`ravel-skill-directories-k18`, identified a caller-wording obligation. The
resulting Ravel commit is `b93bb927095ff09b91e9b78dc1ad05bdacfec649`.
Grove's mandate must qualify "first" with "after any mandatory guidance".

A name plus "load", including the plugin-qualified spelling, remains a name
request. A name request must work through supplied directories with native
copies absent. Concrete native tools and file paths follow the explicit-route
rules. The delivery spec now carries these acceptance cases.

The earlier claim that no session-interface assumption needed correction was
too broad. The CLI stays the same, but the mandate wording changes before
directory delivery is accepted.

The integrated Ravel contract keeps shared owner settings, so separate prefixes
alone cannot enable skills alongside 1.1 under the same HOME. The preparation
ADR records that trade-off. Compaction acceptance separately removes the whole
preamble and observes unprompted recovery. A retained descriptor path is not
evidence that an agent remembers the recovery instruction.

`ravel-rhai-skill-delivery-k34` corrected the contract for Rhai main as Ravel
commit `fc87cb41a422`. `ravel-rhai-skill-delivery-k49` reviewed it, and
`ravel-rhai-skill-delivery-k50` integrated both findings. The reviewed contract
is `822c0ed2f67d9a446b74476fdad7d05922552b87` on the unpublished bookmark
`skill-directory-delivery`. That commit, this adoption and the review of this
adoption are the input to implementation planning. The historical `b93bb927`
contract supplies intent only.

The producing tasks record the owning-repository commits and validation. These
design sessions leave product code and owner configuration unchanged.

<a id="rhai-mapping"></a>
## Rhai contract mapping

The consumer handoff of Ravel's `docs/skill-discovery-design-evidence.md` at
`822c0ed2` names the facts below. `grove-rhai-skill-delivery-k35` read the
Ravel files with `jj file show` and changed none. It read Grove at `314d464e`.
The [delivery specification](specs/codex-skill-provisioning.md#supported-routes)
states the resulting routes. `grove-rhai-skill-delivery-k51` reviewed this
adoption. `grove-rhai-skill-delivery-k52` integrated its one finding as the
[runtime parity](specs/codex-skill-provisioning.md#runtime-parity) of the
delivery specification.

| Upstream fact | Grove obligation | Owner |
| --- | --- | --- |
| Directory delivery needs Ravel 2.0.0, `config.rhai` and `SCHEMA_VERSION` 4. Grove admits only stable 1.x | Admit stable 2.x from 2.0.0 beside 1.x. The admission selects no route. The 2.0 route needs Rhai caller assets | `grove-dual-ravel-k27` |
| The native route stays Ravel 1.1.0, `policy.ts` with schema 2, and settings without 2.0 keys | Keep the TypeScript assets at their installed paths for 1.x. Keep the 1.1 pinned inputs and the legacy-driver fixtures | `grove-dual-ravel-k27` |
| Ravel 1.1.0 refuses settings that hold `skills`, `configEnv`, `harnesses` or `budgets`. While a 1.1 reader remains, the directory route uses a separate `HOME` | Grove sets no HOME. That HOME also holds Grove's standalone run logs and the HOME-based configuration of the harnesses and the VCS. Acceptance runs each route in its own HOME | `grove-dual-ravel-k27`. The owner-delivery design owns the links of the owner's HOME |
| Examples put `request.prompt` into the harness arguments. They read task data, such as the handle and the receipt, from `request.originalPrompt` | The Rhai sample and examples pass `request.prompt`. Grove's assets read task identity from `taskId`, `taskFile` and `context`, as the observations below show. The example of a handle and a receipt does not match Grove | `grove-dual-ravel-k27` |
| The mandate qualifies "first". The ending command stays in the caller block, byte for byte | `grove-skill-mandate-k22` delivered the mandate. Acceptance compares `originalPrompt` and the caller block with the composed mandate | `grove-dual-ravel-k27` and `common-loading-acceptance-k29` |
| Grove needs neither `RAVEL_SKILL_DIRECTORIES` nor `skills resolve` | No Grove change. The variable grants no authority, and Ravel replaces or removes it, so Grove's scrub list and the cargo guard need no new name | None |
| Grove parses neither the routing notice nor the failure document. Both keep `schemaVersion` 2 | Grove reports Ravel's diagnostics without parsing them, as before | None |
| Until promotion, the directory route selects the keg-only `ravel@2` by its prefix | The PATH of the Grove invocation selects that prefix. Grove's formula keeps its unversioned dependency through promotion | `grove-dual-ravel-k27` and `discovery-rollout-k6` |

The fourth row is the one inexact mapping. Ravel's normative rule binds a
script that analyses the task: it reads `request.originalPrompt`. No Grove
asset analyses prompt text. An owner script that does so, such as a policy
with a deciding agent, follows that rule. No Ravel correction is necessary, because
the inexact text is an example in an evidence record.

| Observation | Source | Consequence |
| --- | --- | --- |
| `check_version` admits only output with the prefix `ravel 1.` and a nonzero minor. Its remedy says to run `brew upgrade ravel` | Grove `crates/grove/src/dispatch/ravel.rs` | After promotion, that remedy moves the default command to 2.0.0. The new remedy names both majors and their configuration files |
| Every file under `assets/ravel-policy/` is TypeScript and imports `ravel/sdk`, directly or through another asset | Grove `assets/ravel-policy/` | No asset runs under Ravel 2.0.0. Grove issue 24 asks for Rhai equivalents with the same refusals and the same receipt format. The runtime parity of the delivery specification qualifies the refusals |
| The sample and the examples pass `request.prompt` to the harness. `grove.ts` reads `request.taskFile` through `host.readText`. `provenance.ts` reads `request.kind` and `request.taskId` | Grove `assets/ravel-policy/` | The prompt-field mapping above |
| `task ravel:review` and `task ravel:provenance` need `RAVEL` and typecheck the assets against the installed SDK | Grove `Taskfile.yml` | Ravel 2.0.0 has no SDK and no inspection verb. Checks of the Rhai assets launch them through public `start-session` with fake agents |
| `CONTROL_ENV` holds ten names, and `RAVEL_SKILL_DIRECTORIES` is not one of them | Grove `crates/grove-loop/src/ravel_job.rs` | No change, as the mapping states |
| `standalone` reads `HOME` to locate personal run logs | Grove `crates/grove/src/standalone.rs` | A separate HOME moves the standalone run logs of the 2.0 route |
| The owner's `~/.config/ravel/policy.ts` imports `ravel/sdk` and Node modules, and no Grove caller asset | Owner configuration, read only | The owner's conversion to `config.rhai` does not wait for the Rhai port of Grove's assets |
| `[workspace.package]` says `version = "1.1.0"` | Ravel `Cargo.toml` at `822c0ed2` | A build of the line reports `ravel 1.1.0`, and Grove admits it as 1.x. Acceptance identifies each build by commit and build identity |
| Ravel 2.0.0 embeds no module, and a script imports a module by path. A caller that needs a constant member order, such as Grove's receipt, joins members that `json_encode` wrote | Ravel `docs/specs/configuration-evaluation.md` at `822c0ed2` | The Rhai port needs no Ravel change. That specification lists the capabilities that Grove's helpers need and names Grove issue 24 |
| The frozen comparison keeps three runtime differences. They concern an oversized choice file, U+FEFF and U+0085 as whitespace, and a jj store path that is not UTF-8. A breach of a read bound refuses even inside a catch | Ravel `docs/specs/configuration-evaluation.md` at `822c0ed2`, *Frozen comparison rules* and *Measured reads* | Grove's sample reads a choice file and a jj store path, so it meets each difference. The delivery specification accepts them for that input of the owner |
| The wrapper, the review loader and the review example test blank labels with `trim`. In Bun, `trim` empties U+FEFF and keeps U+0085. A receipt that escapes an unpaired surrogate passes the canonical comparison | Grove `assets/ravel-policy/`, and a Bun probe by `grove-rhai-skill-delivery-k52` | The Rhai helpers test the whitespace set of ECMAScript explicitly. The Rhai loader refuses the surrogate receipt, which is an accepted difference |
| Caller migration step 4 builds each read grant of a harness from `request.skills`, for example one argument for each `snapshotRoot` | Ravel `docs/specs/skill-discovery.md` at `822c0ed2` | The Rhai sample builds those grants where the sandbox of a harness needs explicit directories |
| The `ravel@2` formula is keg-only, and its `bin/ravel` is never the default PATH command | Ravel `docs/specs/release-channels.md` at `822c0ed2` | The owner selects the prefix explicitly |

## Source and coverage

The inspection used Tier 2 verification on project
`Users-antony-Development-AgenticOrchestration-grove.gh-issue-25`. That project
had a ready full index at generation `2026-11-01T11:21:49Z`. Coverage record
`2026-11-01T11:24:11Z` matched that generation and the source metadata at
inspection. The inspection fully paginated the targeted build/resolver searches.
It narrowed broad exploratory searches before it relied on them. The truncated
results of those broad searches are not an inventory.

`ensure_codex_skills` has one recorded inbound caller, `cli::execute`. The
inspection read the source of `ensure_codex_skills` and the call site. The trace
also inferred common-method edges outside this subsystem, so no correctness
claim relies on those inferred edges. Exact snippets established the
provisioning entry point, the generated inventory and the version predicate.
Coverage reported no recorded gap for the cited Rust/build paths. That result is
a best-effort signal, not proof of completeness.

The installer has a recorded partial parse at lines 188–447. The inspection read
that entire range directly, including harness eligibility, link reconciliation
and foreign-entry behaviour. The inspection also searched/read configuration,
release scripts and documentation as non-code text. This is a bounded consumer
audit. It is not a claim that every downstream reference is already corrected.

| Inspected source | Observation and implication |
| --- | --- |
| [CLI](../crates/grove/src/cli.rs), `execute`, lines 91–106 | Provisioning follows workspace/lease acquisition and precedes Ravel resolution. Remove this call without introducing skill preflight. Standalone already bypasses it. |
| Provisioner `crates/grove/src/provision.rs` (removed), lines 13–41, 98–132, 192–307 | Generated inventory drives native installation. The current pointer, retained snapshots and canonical/legacy links live under account-owned roots. Retiring the writer must not remove its output. |
| Build script `crates/grove/build.rs` (removed), lines 42–88, and [binary modules](../crates/grove/src/main.rs) | Build scans all plugin files, embeds bytes and modes, derives Codex eligibility and rejects duplicate directory names. Remove this build dependency with the private module. |
| [Package manifest](../crates/grove/Cargo.toml) | Existing general dependencies also serve other code. Do not infer that a dependency is removable merely because the provisioner used it. |
| [Ravel resolver](../crates/grove/src/dispatch/ravel.rs), lines 34–76, 153–204 | The resolver admits stable 1.x from 1.1.0 and refuses 2.x. Extend session compatibility deliberately to 2.0.0 and keep 1.1 for native-only users. The current undirected upgrade remedy must change. |
| [Policy assets](../assets/ravel-policy/README.md) and [sample](../assets/ravel-policy/sample/policy.ts) | Shipped examples currently declare policy schema 2. New examples must identify their target runtime and schema. Ravel 2.0.0 has no SDK, so its route needs Rhai assets, as the [mapping](#rhai-mapping) states. Context/receipt schema 1 is not a policy schema to bulk-replace. |
| Installer `plugins/install.sh` (removed by `manual-installer-removal-k236`), lines 188–447 | Explicit native installation applies harness eligibility and reconciles links. Keep it as an owner-invoked route. Never invoke it as startup/upgrade cleanup. |
| [Conformance](../plugins/grove/conformance.sh), lines 305–430 | Loaded paths and owners currently use a shared skills directory and spine fallback. Cross-pack proof must exercise actual resource lookup and ownership beyond this layout. |
| [Release builder](../scripts/release-build.sh), archive layout declaration, and [formula](../scripts/templates/grove.rb.tmpl), lines 28–75, 106 | Archives carry executables/caller assets, while the binary hides skills internally. Layout need not gain a skill payload. Caveats/tests currently assume Codex provisioning, Ravel 1.x and schema 2. |
| [Pinned inputs](../scripts/ravel_target_inputs.py) and [installed smoke](../scripts/grove-installed-smoke.sh) | Fixtures hard-code Ravel 1.1.0 and package identities. Keep a 1.1 compatibility lane. Add explicitly pinned new inputs rather than silently testing whatever PATH finds. |
| Provisioning tests `crates/grove/tests/skill_provisioning.rs` (removed) and [lifecycle tests](../crates/grove/tests/lifecycle_cutover.rs) | Existing tests positively assert automatic installation and embedded guidance. Replace obsolete assertions with no-native-mutation and actual delivery-route acceptance. Preserve useful lifecycle coverage. |
| [Fallback audit](../crates/grove/tests/plugin_fallback.rs) | Optional craft dependencies have independent coverage. Preserve this behaviour under both delivery routes and across later pack moves. |

## Interface and alternatives

| Evidence | Caller obligation | Coupled concerns and consequence |
| --- | --- | --- |
| Current build/provisioner | Binary release must include a complete broad corpus | Corpus movement, binary compilation and shared native writes change together. Removing the adapter removes this coupling. |
| Owner-managed delivery | The user supplies skills for selected harnesses. Grove names a skill | Native installation and directory injection can vary without changing task-tree logic. Setup costs remain visible. |
| Directory hints | Agent discovers and reads applicable guidance | Prompt delivery, actual reads and instruction adherence are distinct. Neither executable certifies the last two. |
| Mandatory bootstrap | Grove qualifies "first" and keeps skill names independent of tools | The injected preamble and the caller agree on read order. An absent native copy must not defeat a name-only mandate. |
| Missing mandated guidance | The mandate supplies the force-stop rule before any skill loads | A report without a defined ending can relaunch the same live leaf indefinitely. Acceptance must observe the stopped loop. |
| Confined native delivery | Prove all native loader/resource reads separately from installation | HOME does not grant skill-tree access. Existing file grants and system paths prevent a universal impossibility claim, but establish no working harness setup. |
| Native plugin namespace | The mandate names the bare skill and does not assert its shipping plugin | The first external pack and subsequent moves must prove actual native names and directory loading without native copies. |
| Old native readers | Preserve links, pointers and resolved paths | Lazy/untracked readers prevent safe automatic cleanup. Keeping old content costs disk and owner administration. |
| Existing public session CLI | Check admitted majors, with TypeScript assets for 1.x and Rhai assets for 2.x | The [supervision record](adr/dispatch-supervises-the-harness.md#considered-options) owns dual-major admission, its alternative and reopen condition. |

The rejected fallback and native-writer alternatives are recorded once in
[the delivery ADR](adr/homebrew-installs-skills-and-ravel-delivers-them.md). This design claims
no new formal model. Existing process, filesystem and conformance seams plus
real harness trials answer the material doubts.

## Documentation and delivery handoff

This is a consumer map for planning, not an executable implementation plan or
an exhaustive stale-string inventory. Each delivery increment updates the
current documentation that it makes true. Historical evidence retains its
original version labels and measurements.

| Delivered behaviour | Documentation and checks to move with it |
| --- | --- |
| Remove automatic native writes | README installation, `plugins/README.md`, `plugins/CONTEXT.md`, `docs/USAGE.md`, architecture delivery/module sections, glossary Skill delivery and Spine skill, formula caveats and `check:skills`. Supply the optional manual cleanup procedure in `docs/USAGE.md`. Link it from plugin installation guidance. Exercise ownership cases in disposable layouts. Mark the old `docs/plans/codex-skill-provisioning.md` as historical or remove it when its implementation instructions cease to be useful. |
| Caller/version adoption | Admission of 2.x from 2.0.0, its remedy and fixtures. Rhai caller assets, as Grove issue 24 asks, with the runtime parity of the delivery specification, and the TypeScript assets kept for 1.x. `docs/grove-provider-evidence.md`, with the definition of a blank label for both runtimes. The assets README, the configure-grove skill, the policy example in `docs/USAGE.md` and settings guidance. Caller spec/ADRs/views, `docs/specs/standalone-invocations.md`, `CONTEXT-MAP.md`, target fixtures, the asset gate in `docs/RELEASING.md` and release instructions. The glossary entries Delivered context, Owner settings, Choice file, Sample policy and Static dispatch. On 2.x, prompt analysis reads `originalPrompt`. The architecture rows for Ravel and the caller assets. `grove-skill-mandate-k22` delivered the qualified "first", the missing-skill force-stop and the bare-name target. Preserve explicit old/new major combinations. Separate lifecycle native acceptance from confined access proof. |
| Source-exact runtime books | Overview chapters `02-the-surface`, `03-three-steps` and `06-standalone-invocations`. The grove-loop chapter `19-the-loop` where launch/prompt boundaries appear. Relevant source/concept indexes and manifests. Regenerate snippets against final code in the increment that changes it, not during this documentation-only design. Run the existing book checks. |
| Common source delivery | Installation examples for native and Ravel routes. Use complete plugin boundaries while sources remain here. `plugins/linkuistics/skills/guardrail` is Claude-only, but common directory injection does not filter installer metadata. Check that its trigger/guidance does not claim unsupported behaviour in Codex. |
| First external research pack and later moves | Own and aggregate conformance, required cross-source loading, resource/provenance/licence preservation, native package references, common-directory trials. Prove a moved kind loads in native Claude Code under its real plugin namespace, and through directories with native copies absent. Establish reusable tooling in the research proof before the remaining domain moves. |
| Installed rollout | Archive/formula/native and Linux-floor checks with explicit Ravel inputs, both caller-asset sets and both setup routes. Formula caveats show both configuration formats, and the `ravel@2` prefix until promotion. Preserve legacy-driver rehearsal and original endings. Every intermediate tap state follows states A to D of the [coordinated release](specs/coordinated-release.md#compatible-tap-states). After the cut, repeat the admission and installed checks against the published `ravel 2.0.0` archives. Shared settings wait for compatible readers. |

The adoption integration leaves Ravel and tap repositories unchanged. The
common-loading planner owns caller corrections and manual-cleanup documentation.
The research-pack and repository-move planners own native namespace trials.
The rollout planner coordinates the tap dependency transition and any Ravel
release-document correction with those repositories' owners.

Read Ravel's reviewed `822c0ed2` contract. The `b93bb927` contract supplies intent only, and
the catalog design is discarded. These handoffs do not authorise a shared
installation or settings change during design.

Relevant reusable entry points already exist in [Taskfile.yml](../Taskfile.yml):
`check`, `check:skills`, `ravel:resolution`, `ravel:commands`, `ravel:policy`,
`ravel:activation`, `ravel:fixtures`, installed release checks and
`design:harness-selection`. Extend those from actual implementation needs.
The integrated result still requires `bash scripts/check.sh`. A design
validation is not that result. Native-installation and directory-injection
harness trials need separate evidence. An unavailable delegate or compaction
case stays an explicit acceptance gap.

## Design verification

The design is subject to local-link/anchor, diagram manifest/source and
served-resource validation. The producing task records actual browser
rendering, the checks performed and any remaining gaps. These design leaves do
not measure product tests, actual Ravel 2.0.0 compatibility, new source
acquisition or real harness compliance.
