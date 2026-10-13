# Grove caller documentation evidence

The [caller specification](specs/harness-selection-and-execution.md) and
[visual views](design/harness-selection-and-execution/README.md) describe Grove's
public use of independently installed Ravel. The executable specification,
runtime decisions and controls remain Ravel-owned. This evidence covers those
documents and their evidence locator; it establishes no whole-repository prose
closure. Architecture/glossary reconciliation and complete surface acceptance
remain the following increments of grove-ravel-docs-k12.

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
