# Grove caller documentation evidence

The [caller specification](specs/harness-selection-and-execution.md) and
[visual views](design/harness-selection-and-execution/README.md) describe Grove's
public use of independently installed Ravel. The executable specification,
runtime decisions and controls remain Ravel-owned. This evidence covers those
documents and their evidence locator. The k56/k57 sections retain their bounded
measurements; [complete prose acceptance](#complete-prose-acceptance) below
records k58's wider surface check and release-ready handoff for k13.

## Source and destination boundaries

The measured Grove source is the k56 documentation change on baseline
`ee1de104`, with unchanged production source and release instruments. The exact
SHA-256 pins for the edited spec, presentation README, runtime evidence locator,
all nine Mermaid sources and `diagrams.json` are in
`target/k56/document-pins.json`. The test inputs and instruments are individually
pinned in `before.json` and `after.json`; all 46,943 records agree, with no
added, removed or changed measured subject. Reporting and retirement follow
that measurement.

Nine Ravel executable/spec/transport/authority/runtime/runner/ADR destinations
were read at published release source
`bbab823803bd2f2796f8bd9570f732dce900efa2` and compared byte for byte with their
public raw URLs. `ravel-destinations.json` records each digest. Ravel main is
sealed at `3916a4a1d17fef925db7e9c730bd4c23d6df3ac8`; its publication evidence
at that revision is linked separately. Personal policy/settings and the complete
independent Ravel 0.1.0 and original Grove 22.1.0 installations retain their
measured bytes.

The historical Grove runtime evidence is pinned to the reachable published
22.1.0 source `825025bd8a644d6857475fd37ac64b511f6a37e5`. The inherited
`639b9d` citation could be read locally but returned GitHub 404, including through
authenticated contents lookup; it was not a usable published locator. The full
published evidence was fetched before replacing the local implementation claims
with the locator. Ravel's inherited historical citation has the same local-only
reference; Grove's locator uses the verified published source directly.

## Bounded checks

| Instrument | Observed result |
|---|---|
| Destination/link check | All nine external URLs in the edited payloads fetched successfully; the release-source comparison above also checks runner/ADR destinations. |
| Anchor/reference check | Every former explicit spec/evidence anchor remains; all 13 reference-navigation cases pass and agree individually with the full workspace run. |
| Lexical inventory | Every lexical token occurrence in the 13 edited payloads is enumerated. Old executable names describe explicit original-live-v22 compatibility; the runner name describes Grove's retained local runner. Paths remain stable locators. |
| Observed search controls | Current payload is clean for obsolete launch/store mechanics; the same command finds the historical contract and a deliberately mutated payload. The current public-command positive control is observed separately. |
| Mermaid 12.0.0 parser | All nine sources parse; a malformed source is rejected. Semicolons that split sequence messages were corrected to commas. |
| Fresh-context read | One adversarial source/order/ownership/link review found the historical locator defect above, classified actionable and repaired against the fetched published source. No other material finding. |
| Repository gate | `task check` passes all 18 principal checks, 1,378 workspace nextest cases, nine doctests and six final/all source books with no deferred lines. The gate's installed caller-asset/provenance tasks also pass. |

Complete logs and instruments are under `target/k56/`: `check.log`,
`references.log`, `audit.log`, `diagrams.log`, `audit.py`, `parse-diagrams.mjs`
and `measure.py`. The lexical occurrence and old-reference inventories, exact
anchor sets, observed control outputs and fetched link digests are JSON files
beside them. `runtime-outcomes.json` records all 1,419 nextest executions in the
gate, representing 1,387 unique identities. `per-case-comparison.json` compares
shared helper and focused-reference outcomes individually rather than by totals.
After this reporting document was added, all 13 reference-navigation cases
passed again (`postreport-references.log`). The measured spec/view payloads
still match their individual pins.
Workspace nextest skips 31 cases; this run does not claim that every ignored
installed/target case was executed. The earlier complete caller/delivery
acceptance records retain their own scope.

The freeze includes tracked source, task notes, book manifests/indexes/fragments,
locked Cargo dependency sources, installed caller SDK and compiler dependencies,
the parser and its locked dependencies, measurement tools, retained target/floor
inputs, complete original Grove and independent Ravel prefixes, owner config and
adjacent tracked Grove/Ravel/tap files. Generated run outputs and the live shared
legacy history store are excluded; this gate uses fixture stores, not that live
database. The measurement instrument defines the exact paths.

Graph verification used Tier 2 and Grove generation `2026-10-13T14:37:15Z`,
exact caller snippets and result/command traces. Coverage was checked for every
relied-on path. Mermaid coverage is partial, so every source was read directly;
Ravel's documentation is excluded from its fast graph, so published destination
bytes were checked directly. Coverage is best-effort, not a completeness proof.
No browser surface was available; parser acceptance establishes syntax, not
visual rendering or layout quality.

The original driver's admission/teardown/ending contract is preserved. This
increment changes documentation and task decomposition, with no production
runtime, owner-policy, product installation, publication or live-channel change.

<a id="ownership-reconciliation"></a>
## Ownership reconciliation — k57

The k57 change on sealed k56 baseline `2953777d` reconciles architecture,
glossary/context map, current guides, caller decisions/spec clauses, book prose
and structure briefs, lookup captions, plugin guidance and root/config comments.
The exact 46 payload paths and SHA-256 pins are in
`target/k57/document-pins.json`. The existing composition-guidance test pins now
distinguish Grove's binaries from its explicitly imported caller adapter. No
production runtime source or source-exact book fragment changed.

Current prose describes the public start-session/end-session boundary, one
absolute-PATH/version resolution, independent installs, caller-owned prompts,
results, provider receipts and readable assets, Grove's local runner, and its
task/Creator/teardown conventions. Historical store vocabulary remains labelled
retired; old anchors remain stable. Original live-v22 sessions alone keep their
dispatch ending/admission/Creator-run contract. The retired multi-harness skill
sweep is distinguished from current Codex snapshot provisioning.

| Instrument | Bounded result |
|---|---|
| Lexical inventory | Every token occurrence in the 46 payloads is enumerated; boundary tokens are classified as current CLI/caller contracts, local runner, preserved source/example, stable locator, generic code dispatch or named history/legacy exception. This is an inventory plus semantic passage review, not whole-repository coverage. |
| Search controls | Current obsolete-ownership clauses read clean; the same instrument finds the historical architecture and a deliberate mutation. The public-command positive also fails after deliberate deletion. All five controls were observed. |
| Citations and anchors | All explicit payload anchors remain. All 43 external citations fetched successfully; four Ravel document URLs additionally match their exact `v0.1.0` committed bytes. Local links/references pass the repository's 13 reference-navigation cases. |
| Fresh-context review | One reviewer found six actionable defects in attribution, teardown precedence, obsolete exit wording/runtime ownership, mandate summaries, binaries/assets scope and sentence completeness. Each was repaired against the existing source contract. No new design or second reviewer. |
| Final repository gate | With existing checkout-local Bun 1.4.2 first on PATH, `task check` passes all 18 checks, 1,378 workspace nextest cases, nine doctests and six final/all books with no deferred source lines. Installed caller assets/provenance and conformance negative controls also pass. |
| Individual comparison | All 35,372 named frozen inputs agree before/after. All 1,387 unique runtime identities and outcomes agree with k56; the focused 13 reference and 17 composition cases agree individually with the final gate. The gate contains 1,419 nextest executions including repeated helper cases. |

Complete logs and instruments are under `target/k57/`: `check.log` is a recorded
terminal session retaining nextest's UI; `audit.py`, `audit.log`, `measure.py`,
`before.json`, `after.json`, `freeze-comparison.json`, `lexicon.json`,
`classified-references.json`, `anchors.json`, `controls.json`,
`ravel-destinations.json`, `external-citations.json`, `runtime-outcomes.json`
and `per-case-comparison.json` retain the per-item evidence. Focused logs are
`references.log`, `composition.log` and `typecheck.log`.
After adding this reporting section, all 13 reference-navigation cases passed
again (`postreport-references.log`); the 46 measured payload pins still agree.

The first preflight was stopped to repair an additional glossary delivery
contradiction. A subsequent full run exposed host PATH Bun 1.4.3 and the two
obsolete wording-pin cases; those failures are retained in `failed-check.log`.
Both earlier measured subject sets also agree individually. Verification uses
the already existing `/Users/antony/Development/Ravel/target/tools/bun` 1.4.2,
without replacing any installation. The final freeze explicitly includes that
effective tool. All edits preceded the final measurement; reporting and
coordination/retirement follow it.

The freeze covers tracked source/task notes, book manifests/indexes/fragments,
locked Cargo dependency sources, compiler dependencies, measurement instruments,
retained target/floor inputs, complete original Grove 22.1.0 and independent
Ravel prefixes, owner configuration and adjacent tracked Grove/Ravel/tap files.
Generated run outputs and mutable shared legacy history are excluded. The
measurement script defines the exact paths. Adjacent Ravel, tap and default
Grove checkouts remain clean at `3916a4a1`, `bd5aa37b` and `89f947d5`.

Tier 2 graph evidence used generation `2026-10-13T15:02:28Z`, exact resolver,
command/result/precedence snippets and caller traces. Every relied-on path and
the relevant book/spec scopes received coverage checks. No recorded gaps were
reported except the intentionally excluded `.cargo/`, whose configuration was
read directly. Coverage remains best-effort; semantic and source checks remain
necessary. Workspace nextest skips 31 cases; this run does not claim every
ignored installed/target case or a new all-target delivery run.

k57 establishes this bounded ownership handoff. k58 still owns complete affected
surface enumeration, exception reconciliation and k12 closure/release-ready
promotion for k13. Ravel injects no instructions and assigns no result meanings;
current Grove accepts only `grove-continue`, with force-stop remaining k8 work.
No installation, publication, policy mutation, new driver, teardown or live
ending-channel change occurred. This session retains original
`harness-dispatch exit`.

<a id="complete-prose-acceptance"></a>
## Complete prose acceptance — k58

k58 accepts every grove-ravel-docs-k12 criterion after k56/k57 and closes that
node. This is documentation acceptance on sealed source baseline
`9f9bc71030ed979d6b6fc959d74dfb5c78c635d7`, plus the k58 repairs named below.
The leaf's handle-named commit seals those repairs, this evidence, the refreshed
inventory and coordination promotion together. This establishes readiness for
k13's integration/release; it is no tagged all-target release measurement.

| k12 criterion | Delivered evidence |
|---|---|
| Current guides, vocabulary and caller decisions | README, usage/release/architecture, glossary/context map, caller ADR/spec clauses and plugin guidance describe independently installed Ravel's public CLI, unchanged prompt and caller-owned ending/result/provenance conventions. Grove retains its local runner, task/Creator rules and teardown. The k56/k57 pins were compared individually: all payloads agree except the explicitly repaired provider-evidence verification footer. That footer's old pin also matches the sealed parent. |
| Walkthroughs, diagrams and transferred ownership | Active book prose, contents/index/structure summaries, all nine views and manifest captions describe the same boundary. Existing source-exact fragments and explicit anchors remain. Ravel executable/runtime specification, transport, authority, runner/worker decisions and published evidence destinations exist; eight actual cited document URLs were fetched and match their exact committed bytes. The historical runtime locator names the verified published Grove 22.1.0 source. |
| Complete affected surface, named exceptions and controls | A fresh whole-tracked-tree lexicon precedes candidate classification, including root files, dotfiles, comments, diagram/summary data and fenced source/examples. The [per-file inventory](ravel-caller-surface-inventory.tsv) records paths, meanings, exact candidate lines, tokens and source digests; binary fixtures and the inventory's own generated row are explicit. This consumes earlier bounded handoffs without treating their edited-file lists or historical counts as current scope. Exact occurrence classifications and observed controls are saved under target/k58. |
| Validation and tested handoff | The final gate passes all 18 checks, including reference/conformance validation, 1,378 workspace nextest cases, nine doctests and six final/all books with zero deferred lines. All 46,980 frozen subjects match individually. All 1,387 unique nextest identities/outcomes equal k56 and k57 individually; all nine doctest identities/outcomes equal k57 individually. k13 consumes this source, the k5/k6 delivery evidence and the preserved original-driver procedure below. |

### Repairs and semantic checks

The sweep found four stale current-tooling passages: the conformance inventory
header called the old dispatch ending current; the caller-assets README still
claimed an obsolete bundled fixture suite; the provider-evidence guide named
its removed compatibility task; and the compiler README still described the
removed package's compiler as present. They now distinguish current Ravel
endings and caller checks from the original-live-v22 contract and removed tooling.
The C process-state fixture's inherited probe motivation is explicitly
pre-extraction; its executable statements are unchanged.

Link chasing found the configure-grove reference's new-assets GitHub main URL
returned 404 before integration. It now cites the existing caller-assets README
relative to this source tree, using the same repository-citation convention as
its adjacent caller contract. The pre-report link pass checks 1,944 local destinations
and anchors across 110 discovered current Markdown subjects, including wrapped
labels; all resolve. Fenced and inline-code examples are excluded. The existing
13 reference-navigation cases also pass. This is a check of the accepted simple
Markdown link form, not arbitrary rendered HTML or every historical research URL.

Semantic checks compare summaries with the resolver, public-command adapter,
held result reader and loop decision order. Epoch invalidation gates reading;
driver interruption precedes teardown, and teardown precedes surviving-group,
status and exact-result checks. Ravel injects no instructions and assigns no
result meanings. Current Grove supports `grove-continue`; k8's force-stop
extension remains caller-owned work after k13. One fresh-context adversarial
review of bounded public-boundary excerpts against source/task recipes found no
material violations. That read is supplemental and does not claim whole-tree
semantic coverage by itself.

### Retained names, each with a reason

The inventory names each candidate path and location. The following map explains
the exception classes; target/k58/old-name-exceptions.json names every old-name
occurrence separately with its exact token, line, text and reason.

| Named surface | Retained meaning |
|---|---|
| CHANGELOG.md; dated research/evaluation/preservation records; terminal .grove tasks and their ancestor handoffs | Released or observed history. The root's current contract explicitly replaces earlier storage/history/injection requirements. Research proposals are labelled historical; k44/k46/k51 measurements retain their original bounded scope. |
| CONTEXT-MAP.md's grove-and-harness-dispatch anchor; ARCHITECTURE.md's harness-dispatch anchor; dispatch-supervises-the-harness.md and the existing harness-selection-and-execution paths | Stable inbound locators. Their destination prose now describes independent Ravel and Grove's caller responsibilities. The glossary's harness-dispatch/grove mention is explicitly a retired import to avoid. |
| AGENTS.md/CLAUDE.md; .cargo/config.toml; Taskfile authority fixture values; legacy-driver-transition.md and legacy-driver-inventory.json | Original live-v22 ending/admission/teardown preservation, pinned complete-keg entries and deliberately cleared/inert authority. These do not prescribe old commands for new Ravel launches. AGENTS remains the same symlink to CLAUDE. |
| The explicitly original-v22 passages in current guides, glossary, caller spec/ADRs, provider guide, configure-grove policy, finish/driver/retire guidance and conformance commentary | Only launches under that original contract keep harness-dispatch exit and Creator run references; new consumers refuse legacy references and open no old history store. Earlier acceptance documents record their own original session's ending. |
| Overview resolver fragments and Rust/C helpers/tests named in the inventory | Exact witnesses, incompatible-version/cwd/PATH refusals, authority scrubbing, early admission refusal, old-result baits and documentation/output-channel assertions. None launches a removed current dispatch package. Each source/fixture row names its actual assertion. |
| legacy_driver_transition.py, legacy_driver_rehearsal.py and test_legacy_driver_transition.py | Complete original installation preservation plus scratch original-protocol, ownership/collision/cleanup refusal and nested isolation controls. No real installation change was applied by k58. |
| grove-installed-smoke.sh, grove-floor-native-cases.sh, ravel-installed-smoke.sh, ravel-target-inputs.test.sh and release.test.sh | Assert old fronts/runtime payloads are absent, or deliberately insert wrong archive/ELF members to observe refusal. |
| keyed-launch names, local dispatch::ravel paths and other dispatch identifiers | Grove's retained local job runner or ordinary CLI/library dispatcher vocabulary. Ravel has its own runner and separate release; the generic names imply no shared executable package. |

The same rg --hidden -n instrument reads the frozen pre-report active surface clean,
finds the old clauses in the sealed historical surface and a deliberately wrong
scratch subject, then reads the corrected scratch subject clean. A current
start-session positive succeeds and its deliberate deletion fails. The anchor
check finds the real retained anchor and rejects its deleted scratch copy.
Nested-fence example exclusion and a malformed Mermaid rejection were observed,
not assumed. All nine current Mermaid sources parse with pinned Mermaid 12.0.0;
this establishes syntax, not browser rendering/layout quality.

### Frozen gate and release handoff

Complete terminal output is target/k58/check.log. The measured source set and
instruments are individually recorded in before.json/after.json; the unchanged
comparison is freeze-comparison.json. inventory.py, links.py, controls.py,
measure.py, parse-diagrams.mjs and their complete logs retain the instruments.
lexicon.json, occurrences.json, subject-pins.json, inherited-pins.json,
local-links.json, ravel-destinations.json, controls.json and diagrams.json hold
per-item readings. runtime-outcomes.json, per-case-comparison.json and
gate-summary.json retain every runtime identity, doctest, book and check result.
The reporting parser normalizes the older k56 identity/outcome encoding and
k57's binary/name/status encoding before comparison; totals alone are not used.

The freeze covers tracked source/task notes and the inventory, manifests/indexes/
fragments, locked Cargo/compiler/parser dependencies, the effective Bun 1.4.2,
retained target/floor inputs and tools, complete original Grove 22.1.0 and
independent Ravel installations, owner configuration and adjacent tracked
Ravel/tap/default Grove files. Generated run outputs and mutable shared legacy
history are excluded; this gate uses fixture stores. Reporting, inventory-refresh
and tree promotion/retirement follow the freeze and are not misrepresented as
unchanged measured inputs. The inventory is refreshed after retirement; frozen
occurrences/pins retain the measured names. inventory-final.py writes refresh
products separately under final-inventory. An initial refresh overwrote five
measured audit products; exact reconstructed measured source regenerated them,
and every restored byte digest equals its pre-gate record. evidence-restoration.json
records that check. The original measured inventory.py is restored too.
Post-report reference validation is recorded separately.

Tier 2 graph evidence uses full generation `2026-10-13T15:45:21Z`, with exact
resolver/command/result/precedence source and relevant caller traces. Candidate
paths and the complete repository scope received coverage checks with no
remaining pages. Every reported Mermaid range, installer lines 182–382 and QEMU
script line 208 was read directly; excluded Cargo configuration and untracked
metadata surfaces were read directly. Heuristic ambiguous resolver edges were
not used as proof of relationships. Coverage remains best-effort; the source-text
inventory, semantic checks and executable controls supply the acceptance.

Workspace nextest skips 31 cases. This gate does not claim a fresh execution of
every ignored installed/target case or a tagged archive/CPU-floor release. The
[k5 caller acceptance](ravel-caller-surface-audit.md) and
[k6 delivery acceptance](grove-delivery-contraction.md#complete-independent-delivery-acceptance)
retain their complete named ignored-case and all-target evidence and limits.
Ravel remains published/installed at 0.1.0 with source main
`3916a4a1d17fef925db7e9c730bd4c23d6df3ac8`; adjacent tap/default Grove remain at
`bd5aa37b`/`89f947d5`. Owner configuration and both complete installations retain
their measured bytes. k13 must recheck state and rebuild/verify its tagged release.

k13 must prepare/apply its fresh .jj/legacy-driver-22.1.0 plan as
[legacy-driver-transition.md](legacy-driver-transition.md) directs, compare the
complete original inventory, source release-env.sh for every release/recovery
shell, and retain the original canonical keg, backup, owned stable-PATH links
and cleanup exclusions through final finish's last signal. No install,
publication, new driver, policy mutation, teardown or live-channel change occurred
in k58. This leaf still ends with its original harness-dispatch exit contract.
