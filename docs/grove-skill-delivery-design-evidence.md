# Grove skill-delivery design evidence

The [delivery specification](specs/codex-skill-provisioning.md) defines the
adoption target. This document records inspected source, design evidence and
verification limits. It does not claim implementation or acceptance of that
target. Source observations below are against Grove parent `8d45d75e`.
`grove-native-provisioning-k21` later removed the provisioner, its build script
and its tests, so those paths are named without links.

<a id="upstream-correction"></a>
## The upstream correction

The initial input was Ravel design commit
`66aaec06f5c020215b9b0e4139c2ba2e0b969644`, read locally from the owning repository.
That design enumerates skill metadata, generates catalogs, refuses duplicate
names and exposes a catalog descriptor. It is historical input, not the final
contract for this workstream after the human's corrections:

> grove and ravel are decoupled with respect to skills - grove may name a skill,
> but it is up to the user to ensure that the skill is installed either within
> each harness, or via ravel's mechanism

> Ravel doesn't resolve "a" skill - it simply injects the skill directory path(s)
> into the prompt.

Accordingly, Grove's design requires neither a catalog nor Ravel-based skill
installation. The agent finds skills in its native installation or supplied
paths. The correction delivered by `ravel-skill-directories-k15` replaces the
owning Ravel spec, ADR, glossary and visual contract in place. It supplies search
directories and direct mandatory-file references, with a retained directory-only
descriptor for recovery and delegates. Ravel acquires local/exact pinned sources;
it does not parse skill metadata, reject duplicate names or answer a skill lookup.

The producer's Ravel commit is `4c16f8e09d58c8f649a375029c8e8fa2933de2b4`.
Its `docs/specs/skill-discovery.md` defines the corrected interface and its
`docs/skill-discovery-design-evidence.md` records source evidence and limitations.

The corrected Ravel design retains Ravel 2 and policy/request schema 3 because
prepared-prompt semantics change. Grove's existing source still refuses major 2.
The adoption therefore keeps native-only 1.1 support and adds 2.x only through
explicit caller acceptance and versioned policy examples. This does not require
Grove to read a directory descriptor or run source setup.

The directory review in Grove commit `ec3b0700e29c`, integrated by
`ravel-skill-directories-k18`, identified a caller-wording obligation. The
resulting Ravel commit is `b93bb927095ff09b91e9b78dc1ad05bdacfec649`.
Grove's mandate must qualify "first" with "after any mandatory guidance". A name plus
"load", including the plugin-qualified spelling, remains a name request;
it must work through supplied directories with native copies absent. Concrete
native tools and file paths follow the explicit-route rules. The delivery spec
now carries these acceptance cases. The earlier claim that no session-interface
assumption needed correction was too broad: the CLI stays the same, but the
mandate wording changes before directory delivery is accepted.

The integrated Ravel contract keeps shared owner settings, so separate prefixes
alone cannot enable skills alongside 1.1 under the same HOME. The preparation
ADR records that trade-off. Compaction acceptance separately removes the whole
preamble and observes unprompted recovery; retaining a descriptor path is not
evidence that an agent remembers the recovery instruction.

The integrated Ravel review, followed by Grove's adoption review, is the input
to implementation planning.
The producing task records the owning-repository commit and validation. Product
code and owner configuration remain unchanged by these design sessions.

## Source and coverage

Tier 2 (Verify), project
`Users-antony-Development-AgenticOrchestration-grove.gh-issue-25`, ready full
index generation `2026-11-01T11:21:49Z`; coverage record
`2026-11-01T11:24:11Z`, matching generation and source metadata at inspection.
Targeted build/resolver searches were fully paginated. Broad exploratory searches
were narrowed before reliance; their truncated results are not an inventory.

`ensure_codex_skills` has one recorded inbound caller, `cli::execute`; its
source and the call site were read. The trace also inferred common-method edges
outside this subsystem, so no correctness claim relies on those inferred edges.
Exact snippets established the provisioning entry point, generated inventory and
version predicate. Coverage reported no recorded gap for the cited Rust/build
paths. That is a best-effort signal, not proof of completeness.

The installer has a recorded partial parse at lines 188–447. That entire range
was read directly, including harness eligibility, link reconciliation and
foreign-entry behavior. Configuration, release scripts and documentation were
searched/read as non-code text. This is a bounded consumer audit, not a claim
that every downstream reference has already been repaired.

| Inspected source | Observation and implication |
| --- | --- |
| [CLI](../crates/grove/src/cli.rs), `execute`, lines 91–106 | Provisioning follows workspace/lease acquisition and precedes Ravel resolution. Remove this call without introducing skill preflight; standalone already bypasses it. |
| Provisioner `crates/grove/src/provision.rs` (removed), lines 13–41, 98–132, 192–307 | Generated inventory drives native installation; the current pointer, retained snapshots and canonical/legacy links live under account-owned roots. Retiring the writer must not delete its output. |
| Build script `crates/grove/build.rs` (removed), lines 42–88; [binary modules](../crates/grove/src/main.rs) | Build scans all plugin files, embeds bytes and modes, derives Codex eligibility and rejects duplicate directory names. Remove this build dependency with the private module. |
| [Package manifest](../crates/grove/Cargo.toml) | Existing general dependencies also serve other code. Do not infer a dependency is removable merely because the provisioner used it. |
| [Ravel resolver](../crates/grove/src/dispatch/ravel.rs), lines 34–76, 153–204 | Stable 1.x from 1.1.0 is admitted; 2.x is refused. Extend session compatibility deliberately and keep 1.1 for native-only users. The current undirected upgrade remedy must change. |
| [Policy assets](../assets/ravel-policy/README.md) and [sample](../assets/ravel-policy/sample/policy.ts) | Shipped examples currently declare policy schema 2. New examples must identify their target SDK; context/receipt schema 1 is not a policy schema to bulk-replace. |
| [Installer](../plugins/install.sh), lines 188–447 | Explicit native installation applies harness eligibility and reconciles links. Keep it as an owner-invoked route, never invoke it as startup/upgrade cleanup. |
| [Conformance](../plugins/grove/conformance.sh), lines 305–430 | Loaded paths and owners currently use a shared skills directory and spine fallback. Cross-pack proof must exercise actual resource lookup and ownership beyond this layout. |
| [Release builder](../scripts/release-build.sh), archive layout declaration; [formula](../scripts/templates/grove.rb.tmpl), lines 28–75, 106 | Archives carry executables/caller assets, while the binary hides skills internally. Layout need not gain a skill payload. Caveats/tests currently assume Codex provisioning, Ravel 1.x and schema 2. |
| [Pinned inputs](../scripts/ravel_target_inputs.py); [installed smoke](../scripts/grove-installed-smoke.sh) | Fixtures hard-code Ravel 1.1.0 and package identities. Keep a 1.1 compatibility lane and add explicitly pinned new inputs rather than silently testing whatever PATH finds. |
| Provisioning tests `crates/grove/tests/skill_provisioning.rs` (removed); [lifecycle tests](../crates/grove/tests/lifecycle_cutover.rs) | Existing tests positively assert automatic installation and embedded guidance. Replace obsolete assertions with no-native-mutation and actual delivery-route acceptance, preserving useful lifecycle coverage. |
| [Fallback audit](../crates/grove/tests/plugin_fallback.rs) | Optional craft dependencies have independent coverage. Preserve this behavior under both delivery routes and across later pack moves. |

## Interface and alternatives

| Evidence | Caller obligation | Coupled concerns and consequence |
| --- | --- | --- |
| Current build/provisioner | Binary release must include a complete broad corpus | Corpus movement, binary compilation and shared native writes change together. Removing the adapter removes this coupling. |
| Owner-managed delivery | User supplies skills for selected harnesses; Grove names a skill | Native installation and directory injection can vary without changing task-tree logic. Setup costs remain visible. |
| Directory hints | Agent discovers and reads applicable guidance | Prompt delivery, actual reads and instruction adherence are distinct. Neither executable certifies the last two. |
| Mandatory bootstrap | Grove qualifies "first" and keeps skill names independent of tools | The injected preamble and the caller agree on read order; an absent native copy must not defeat a name-only mandate. |
| Missing mandated guidance | The mandate supplies the force-stop rule before any skill loads | A report without a defined ending can relaunch the same live leaf indefinitely. Acceptance must observe the stopped loop. |
| Confined native delivery | Prove all native loader/resource reads separately from installation | HOME does not grant skill-tree access. Existing file grants and system paths prevent a universal impossibility claim, but establish no working harness setup. |
| Native plugin namespace | The mandate names the bare skill without fixing its shipping plugin | The first external pack and subsequent moves must prove actual native names and directory loading without native copies. |
| Old native readers | Preserve links, pointers and resolved paths | Lazy/untracked readers prevent safe automatic cleanup. Keeping old content costs disk and owner administration. |
| Existing public session CLI | Verify admitted major versions and explicit policy examples | The [supervision record](adr/dispatch-supervises-the-harness.md#considered-options) owns dual-major admission, its alternative and reopen condition. |

The rejected fallback and native-writer alternatives are recorded once in
[the delivery ADR](adr/skill-delivery-is-the-users-choice.md). No new formal model is
claimed. Existing process, filesystem and conformance seams plus real harness
trials answer the material doubts.

## Documentation and delivery handoff

This is a consumer map for planning, not an executable implementation plan or
an exhaustive stale-string inventory. Each delivery increment updates the
current documentation it makes true. Historical evidence retains its original
version labels and measurements.

| Delivered behavior | Documentation and checks to move with it |
| --- | --- |
| Remove automatic native writes | README installation, `plugins/README.md`, `plugins/CONTEXT.md`, `docs/USAGE.md`, architecture delivery/module sections, glossary Skill delivery and Spine skill, formula caveats and `check:skills`. Supply the optional manual cleanup procedure in `docs/USAGE.md`, link it from plugin installation guidance and exercise ownership cases in disposable layouts. Mark the old `docs/plans/codex-skill-provisioning.md` as historical or remove it when its implementation instructions cease to be useful. |
| Caller/version adoption | Resolver diagnostics and fixtures, readable policy examples/typechecks, settings/configure guidance, caller spec/ADRs/views, `docs/specs/standalone-invocations.md`, `CONTEXT-MAP.md`, target fixtures and release instructions. Qualify "first", define missing-skill force-stop, and replace the fixed native namespace with a bare-name target before packs move. Preserve explicit old/new major combinations and separate lifecycle native acceptance from confined access proof. |
| Source-exact runtime books | Overview chapters `02-the-surface`, `03-three-steps`, `06-standalone-invocations`; grove-loop `19-the-loop` where launch/prompt boundaries appear; relevant source/concept indexes and manifests. Regenerate snippets against final code in the increment that changes it, not during this documentation-only design. Run the existing book checks. |
| Common source delivery | Installation examples for native and Ravel routes; use complete plugin boundaries while sources remain here. `plugins/linkuistics/skills/guardrail` is Claude-only, while common directory injection does not filter installer metadata. Verify that its trigger/guidance does not claim unsupported behavior in Codex. |
| First external research pack and later moves | Own and aggregate conformance, required cross-source loading, resource/provenance/licence preservation, native package references, pinned/offline/common-directory trials. Prove a moved kind loads in native Claude Code under its real plugin namespace, and through directories with native copies absent. Establish reusable tooling in the research proof before the remaining domain moves. |
| Installed rollout | Archive/formula/native and Linux-floor checks with explicit Ravel inputs, compatible SDK examples and both setup routes. Preserve legacy-driver rehearsal and original endings. Prove every intermediate tap dependency is compatible. Grove dual-major support precedes unversioned Ravel 2 activation, or Ravel 2 initially uses a separate versioned formula. Feature publication precedes Grove advertising it; shared settings wait for compatible readers. |

The adoption integration leaves Ravel and tap repositories unchanged. The
common-loading planner owns caller fixes and manual-cleanup documentation;
the research-pack and repository-move planners own native namespace trials.
The rollout planner coordinates the tap dependency transition and any Ravel
release-document repair with those repositories' owners. Read Ravel's integrated
`b93bb927` contract, not the discarded catalog design. These handoffs do not
authorize a shared installation or settings change during design.

Relevant reusable entry points already exist in [Taskfile.yml](../Taskfile.yml):
`check`, `check:skills`, `ravel:resolution`, `ravel:commands`, `ravel:policy`,
`ravel:activation`, `ravel:fixtures`, installed release checks and
`design:harness-selection`. Extend those from actual implementation needs.
The integrated result still requires `bash scripts/check.sh`; a design validation
is not that result. Native-installation and directory-injection harness trials
need separate evidence, and an unavailable delegate or compaction case stays an
explicit acceptance gap.

## Design verification

The design is subject to local-link/anchor, diagram manifest/source and
served-resource validation. Actual browser rendering, checks performed and any
remaining gaps are recorded on the producing task. Product tests, actual Ravel
2 compatibility, new source acquisition and real harness compliance remain
unmeasured by this design leaf.
