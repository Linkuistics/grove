# pack-membership-k189

**Reviews:** pack-membership-k171

## Goal

Read `docs/specs/pack-membership.md` adversarially before the moves apply it.
The specification governs every later move under `repository-moves-k5`, so a
wrong placement or a missing proof spreads to eight moves.

## Context

The human approved the first two decisions, because they change the root
table. Review their consequences, not the choice. The other four decisions are
the producer's alone. The views are in `docs/design/external-pack-contract/`,
under the topic "Pack membership".

## Done when

Each doubt below has a finding or a stated reason that it holds. Findings go to
the review's own commit. An `integrate-review-design` leaf follows only if a
finding is worth acting on.

## Notes

The producer could not settle these doubts alone:

1. **The asset rule.** Tests 1 and 3 can both match one of Grove's evidence
   records. Check that the order gives the intended result for each row of the
   table. Check also that Grove's own walkthrough books and their decision
   records stay in Grove while the skill's evaluations move.
2. **The doubt assertions.** They move to EngineeringSkills, and Grove's suite
   then reads no text of the skill. Find a change of Grove that the pack check
   would not see in time.
3. **The trial refusal.** The specification names `design-pack-k175` as the
   first move that removes a path a case names. Check that against each case's
   sources, mandatory files, planted files and launched kinds.
4. **Sources without a kind directory.** `check:packs` runs conformance on each
   pinned `git-subdir` entry. The specification gives the change for a craft-only
   source to `testanyware-skill-move-k172`. Check that the TestAnyware entry is a
   `git-subdir` entry at that point. PersonalSkills has only a `craft` root.
   Check whether that is a boundary that the table omits.
5. **The boundary table.** Check that each trial is the cheapest run that fails
   when its boundary breaks. Check that no move adds a boundary outside the
   table.
6. **The hooks plugins.** Native users keep the full writing plugin, and Ravel
   users enable the skill-free plugin. Check that the static proof of
   `writing-craft-pack-k181` stops a hook from running twice.
7. **The Codex listing.** The specification assumes that a Codex `local` source
   names a path inside the marketplace's own repository. Check that against
   Codex's marketplace rules.

## Findings

Reviewed the design producer's commit `6ec5ad371c78a5415f01301ddb8f90860bb75376`.
The four findings below concern the acceptance contract. None changes the two
ownership decisions that the human approved.

### F1 — Check each trial dependency against its own source and revision

**Priority:** P2. **Location:** `docs/specs/pack-membership.md:285` and `:297`.

The refusal puts every dependency in the prepared Grove commit. Existing cases
already have dependencies outside that commit. `research-discovery/case.json:23`
gets `plugins/research` and `grove-research-a` from ResearchSkills at
`1c18a90945a0b2ac4086bfe6f2ba90066fcf4bcc`. That kind has already left Grove.
`research-native-namespace` gets it through the pinned marketplace plugin.
Neither dependency is a missing path in the prepared Grove build.

The stale-copy cases have another revision boundary.
`problem-stale-native-cli/case.json:40` gets its native directory from Grove
commit `9a35d526f334c3621e6cda6807a85faffeefbcda`. The current directory copy
and the historical native copy are different dependencies. The runner already
keeps them separate in `external_sources` (`scripts/harness_trial.py:2453`)
and `install_native` (`:2534`).

Applying the refusal literally would reject a valid research case even against
its recorded Grove build. It would also prevent later boundary cases from using
the external-source mechanism that this specification requires.

**Smallest useful correction:** Bind each required path to its source and the
revision or working copy that supplies it. Resolve a native plugin's kind through
that plugin's pinned checkout. Check historical native bytes against their
declared commit. State that dependency checks precede an intentional
`moveAway` or `moveRoot` fault. Require controls that admit a valid external
case and a valid historical-copy case, beside the missing-local-path refusal.
The timing before `design-pack-k175` can stay.

### F2 — Preserve the hooks of an ordinary native writing installation

**Priority:** P2. **Location:** `docs/specs/pack-membership.md:360`.

The combined-installation check and the skill-free-plugin trial can both pass
while ordinary native users lose the always-on layer. Put all hook declarations
in the skill-free plugin, put the craft skills in the ordinary writing plugin,
and give the latter no dependency on the former. Both plugins can validate.
An installation that enables both has one declaration per hook, and the residue
trial runs the hooks without a native skill. A native user who installs only the
ordinary writing plugin then receives the skill but no hooks.

That route matters: `owner-skill-delivery.md:391` says that native users keep the
full plugins, while Ravel users enable the residue plugins. The earlier full
plugin supplies its hooks automatically through
`plugins/linkuistics/hooks/hooks.json`. The proposed proof never measures its
replacement as an ordinary native installation.

**Smallest useful correction:** State how the ordinary writing plugin obtains
the one hook registration. A dependency on the hooks plugin is one option.
Require the native installation of the ordinary plugin to run the hooks, and
require enabling the residue plugin as well to leave one registration per event.
Count dependencies and default hook discovery in the static proof. Keep the
Ravel-only residue trial, which checks a different obligation.

### F3 — Cover the source whose only root is craft

**Priority:** P2. **Location:** `docs/specs/pack-membership.md:341` and `:366`.

PersonalSkills introduces a layout that the first craft-root proof does not
require. DesignSkills has both `skills/` and `craft/`; PersonalSkills has only
`craft/`, as the owner source table and `personal-pack-k183` require.

The distinction has a concrete consumer. `plugins/install.sh:82` refuses
`--pack` unless the source holds `skills/`. Its controls also require that
refusal (`plugins/install.test.sh:373`). A change that scans craft alongside
skills can pass the DesignSkills fixture while retaining this refusal. The
PersonalSkills listing of `guardrail` exercises its other plugin, and the
Ravel resolution without `guardrail` does not exercise the manual installer.
Thus every specified proof can pass while manual installation of the personal
craft fails.

**Smallest useful correction:** Name the craft-only layout as a boundary. Add
an installer fixture with `craft/` and no `skills/`, including reconciliation
after a skill is removed. It can land with the craft-root installer change in
`design-pack-k175`; PersonalSkills must consume that proof. Also require the
craft-only branch of `check:packs` to report its directory-only check. No extra
model trial is needed to expose the installer failure.

### F4 — Give the TestAnyware trial an existing thing to read

**Priority:** P3. **Location:** `docs/specs/pack-membership.md:356`.

The trial requires the agent to read a supporting resource of
`using-testanyware`. The shipped skill is one `SKILL.md`, as
`testanyware-skill-move-k172` also records. Its command-detail section sends the
agent to `testanyware llm-instructions`, `capabilities`, `schema` and `--help`.
It names no supporting file to copy into the source directory.

Keeping the current skill therefore provides no supporting file that satisfies
the trial. Adding a file solely to make that reading possible changes the skill
and tests a packaging shape that the actual move did not need.

**Smallest useful correction:** Require implicit discovery and a read of the
existing skill. If the trial also needs a resource reading, identify the tool's
existing command reference and say that it is tool output rather than a captured
file. Keep the no-VM limit. Do not invent a supporting file for this proof.

## Disposition of the seven doubts

1. **Asset rule — supported for the named placements.** Test 1 keeps the formal
   campaign records even when test 3 also describes them. It also keeps Grove's
   walkthrough books, validator, book specification and book decision records:
   those state Grove's product contract. `CONTEXT-MAP.md:244` already states
   that ownership boundary. The skill's evaluation and its acceptance record
   concern the craft skill and take test 2. The technical-communication assets
   and checks take test 2; its harness survey moves with its citers under test 3.
   Its subject also overlaps test 2, but both yield WritingSkills. The provenance
   split retains one owner per skill. No named placement needs reversal.
2. **Doubt assertions — accepted trade-off, with a concrete limit.** A Grove
   change could alter the mandate-based review ownership predicate while the
   EngineeringSkills check still reads its older `GROVE_COMMIT`. Grove's
   `check:packs` runs conformance, not the pack's moved composition check, so
   that run would not detect the disagreement. The pack would detect it only
   when its Grove pin advances. Lines 203 to 204 explicitly accept that delay;
   expand and contract is a maintainer obligation, not automatic detection.
   This review does not present the moved check as protection of current Grove.
3. **Trial refusal — timing supported; source model needs F1.** I inspected
   each case's source, mandatory, planted, native and kind declarations. No
   case depends on a TestAnyware or requirements kind path. The design move
   first removes the local `decision-records` and `cli-tool-design` dependencies.
   Engineering later removes the local impl and Bash-style dependencies;
   Personal later removes Jujutsu and authoring guidance. Research dependencies
   already come from external or native plugin checkouts. Continuation also
   names paths that it edits, which must be included when declaring its required
   paths. Intentionally absent skills are outcomes, not required dependencies.
4. **Sources without kinds — TestAnyware's pin is supported; F3 covers the gap.**
   `testanyware-skill-move-k172` explicitly requires a pinned `git-subdir`
   marketplace entry, so `check:packs` will encounter it. TestAnyware still has
   a `skills/` directory, although it has no methodology kinds or manifest.
   PersonalSkills is the first source with no `skills/` directory. These are
   distinct facts and must not be one directory-existence predicate.
5. **Boundary table — F2, F3 and F4 identify missing or unsuitable proofs.**
   The cross-pack family, combined kind/craft roots and external editorial
   family have concrete read-order obligations. Their planned trials exercise
   those obligations. Engineering repeats those shapes. The craft-only source
   needs a fixture check, and the native full-writing route needs installation
   coverage. Staging trials remain responsible for full owner activation.
6. **Hooks — one declaration is the right invariant; F2 supplies the missing route.**
   Counting the complete effective installation can exclude duplicate
   registrations. Counting declarations in each plugin independently cannot.
   The design must also prevent the zero-hook ordinary-native installation
   described in F2. No new hooks packaging exists yet to inspect at runtime.
7. **Codex listing — supported.** Official OpenAI documentation says that a
   `local` path is relative to the selected marketplace root and is for a plugin
   in that repository. It also documents `url` and `git-subdir` sources. Thus
   the WritingSkills-local listing works as proposed. The marketplace move
   can evaluate the documented remote route without a copy of the plugin.
   Source read on 2026-11-04:
   [Supported marketplace formats](https://learn.chatgpt.com/docs/enterprise/plugin-management#supported-formats).

## Verification and limits

This was an inspection-only review. No test, build, lint, format or harness
trial ran. The findings are counterexamples to the proposed acceptance contract,
not observed failures of changes that have not been implemented.

Graph evidence used Tier 2 verification for project
`Users-antony-Development-AgenticOrchestration-grove.gh-issue-25`, generation
`2026-11-04T11:11:33Z`. Relevant searches were paginated to completion, and
coverage was checked for the inspected paths and trial-case scope. The installer
has a recorded parse gap at lines 231 to 507; I read those lines directly. I
also read the four membership diagram sources directly through their reported
parse gaps. The rule manifest has no tracked freshness hash, so its relevant
rows were checked as source text. A clean coverage result is a best-effort
signal, not proof of completeness.

The design keeps one specification for the membership decisions and uses the
existing rule-placement and evidence ADRs. No additional ADR is needed for the
four acceptance corrections. The integration session owns their triage and any
changes to the design, views or downstream briefs.

## Decisions (running log)

- The approved format and hooks ownership choices stand. Four acceptance
  findings need triage before the moves consume the specification.
- The delayed check of doubt composition is the design's explicit trade-off.
  Its dependence on a later pin advance is recorded above.
- Cut an `integrate-review-design` leaf immediately after this review. Its
  charter names this review's handle and leaves every finding open to triage.
