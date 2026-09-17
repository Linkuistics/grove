# Configuration forms after modular-only removal

This records the `modular-audit-k16` audit, corrected after review by
`modular-audit-k18`, of the removal delivered by
`modular-input-k11`, `modular-parser-k13` and `modular-types-k15`. The
[configuration specification](specs/modular-configuration.md) remains the
language contract; this inventory records where its examples and counterexamples
live. It is evidence for this change, not a permanent claim about future edits.

## Surface and method

The surface was repository text, including root documents, hidden `.grove/`
notes and `.cargo/`, Rust fixtures and generators, packaged KDL, scripts,
shipped plugin guidance, Markdown books, structure briefs and Mermaid sources.
VCS internals, ignored build output and external personal policy were excluded.
`jj file list` supplied a tracked-file cross-check: only the `AGENTS.md` symlink
was absent from ripgrep’s file list, and its `CLAUDE.md` target was included.
No active configuration or
previously installed example was changed.

The enumeration began with every identifier followed by a quoted value, rather
than a list of retired symbols or known kind names:

```sh
rg -n --hidden -g '!.git' -g '!.jj' -g '!target' -g '!Cargo.lock' \
  '[[:alnum:]_-]+[[:space:]]+\\?"' .
rg --files --hidden -g '*.kdl' -g '!.git' -g '!.jj' -g '!target'
```

Candidates were classified by enclosing document and use: modular declarations,
command text/parameter values, unsupported-input tests, historical evidence,
or non-configuration text (shell commands, Rust expressions, JSON, prose and
other configuration languages). A separate enumeration of runtime/parameter
slots and configuration path references reached generated documents and
unquoted structural forms. That token enumeration did not adequately check
prose: the subsequent review found stale claims in the spec, reference, source
comments and book explanations. The separate prose pass below addresses that
gap; a quoted-token control is not evidence about unquoted claims.

Controls used the same `rg -n --hidden` enumeration against a scratch hidden
directory: modular declarations were found; replacing them with an arbitrary
flat key absent from the old-symbol list produced a candidate; removing the
quoted declaration returned no candidate. A flat-form cross-tree query found
`catalog_refuses_unsupported_top_level_shapes` in the real runner tests. Its
scratch counterpart was observed dirty before conversion to a modular command
and clean after conversion. These controls establish the search's sensitivity;
the loader tests establish grammar rejection, which text search alone cannot.

The graph's generation `2026-09-17T05:07:07Z` had changed-source metadata for the
configuration files and excluded documentation and scripts. Coverage pagination
was exhausted; current file reads and text searches supplied the missing
evidence. Stale graph line locations and inferred generic-call edges were not
used to establish behavior.

## Prose enumeration and review corrections

The integration pass used the same repository file enumeration, including hidden
notes and root documents. It grouped Markdown outside fenced code into
blank-line-delimited paragraphs, Rust `//` comment runs and shell `#` comment
runs into blocks, and Mermaid text into paragraphs. Tables, headings, indexes
and summaries stayed in scope. Candidate blocks matched this case-insensitive
expression, applied to joined lines so wrapping did not hide a claim:

```text
config(?:uration)?|template|binding|\broute\b|\bprofile\b|AssignmentValue|CommandView
```

This enumerates configuration-bearing text independently of retired spellings
and quoted declaration tokens. It also admits unrelated senses of these words;
classification follows the enclosing subject, rather than treating every match
as a defect. Fenced configuration examples remain covered by the forms inventory;
production comments and exact fragments are paired when corrected. The semantic
questions were validation timing, source authority, composition, inspection
values and diagnostic shape. Their source anchors are Catalog capture,
`named::resolve`, `Templates::source`/`expand`, `AssignmentValue`, `duplicate`
and `render_diagnostics`, read from current files because graph metadata was stale.

| Claim family | Classification and disposition |
|---|---|
| Eager checking of every definition; templates read whole from one file | Stale current claims in the modular spec, grove-llm comment/book, grove-loop book and keyed-launch job explanation. Corrected to structural capture and semantic checking of active composition. The keyed-launch structure brief and grove-loop Vocabulary explanation had related timing drift and were corrected too. |
| A base namespace shared with flat entries; compatibility wording for local-only keys | Stale spec language. Flat input is rejected before wrapper traversal. The retained local-only-key behavior is stated directly as non-admission. |
| Literal-template and reset inspection assignments | Stale overview explanation. The enum and renderer expose Set and Unset; the prose now explains those operations. Literal argument words remain valid and are a different concept. |
| Local delta as the command-definition source | Stale driver comments, book explanation and operator reference. Definitions are personal-only; inspection explains local route, binding and parameter contributions. The personal-path fallback comment was reconciled with the same rule. |
| Duplicate-key report with both locations inline | Retired reference rendering. The current message says duplicate declaration; the second location is a structured related span. |
| Primary error attribution at runtime | Removed the obsolete overlay-role branch from expansion. Templates still retains the overlay path for non-admitted-key diagnostics. |
| Retained original bytes and parse tree | Accepted storage trade-off predating this removal. Resolution reads captured declarations; the retention comment no longer promises a reader of the original bytes. |
| Current composition, parameters, provenance, authority and load points | Retained; checked against the resolver and existing public load/expansion evidence. Launch obtains the resolved argv without appending arguments. |
| Older whole-file grammar and replacement claims | Historical in the preservation baseline's explicitly superseded configuration section, release history and completed task notes; rejected alternatives in the ADR remain labeled as such. |
| Other uses of template, route, profile, eager or reset | Prompt construction, VCS output templates, tree/research sequencing and process signal handling are outside configuration grammar. Lookup rows and fragment metadata identify source rather than promise behavior. |

Controls exercised prose independently of KDL tokens. A hidden scratch Markdown
file containing an unquoted eager-template claim was found by the candidate
expression; replacing it with unrelated prose made the same command return no
match. For the spec, overview explanation and both source comments, whitespace-
normalized text from review commit `9b93069b` matched the obsolete-claim probes
below, while corrected text retained configuration candidates and no longer
matched those probes:

```text
eager.{0,90}(?:validat|check)|(?:validat|check).{0,90}eager|read whole|literal template, unset|flat entries and wrapper|delta.supplied|personal or delta|delta that overrode
```

The same probes still found the historical whole-file claim in
`docs/preservation-baseline.md`, the cross-tree control. These controls establish
that enumeration and regression probes see known bad prose; they do not turn
text matching into a proof of semantic completeness. The classification and
source comparison supply the judgment. Source-exact validation separately checks
fragment bytes, ranges, ownership tables and roll-ups; it cannot validate prose.

## Forms inventory

| Form | Classification and consumers |
|---|---|
| Empty or comment-only document; empty wrapper | Supported capture with no invented routes; runner Catalog and inspection tests |
| Wrapper with personal commands, bindings and routes | Supported base-only configuration; current guides, books, fixture generators and release smoke recipe |
| Optional command parameters, defaults, shared values, route overrides and `unset` | Supported modular parameter composition; named-command and profile tests, reference and packaged examples |
| Personal profiles, includes, default selection; local selection, including empty selection | Supported ordered composition; Catalog, profile and SessionConfig tests, packaged examples |
| Local bindings/routes/value patches | Supported only with active personal target authority; local command/profile definitions remain invalid |
| Runtime slot, parameter fragment and escaped dollar text inside a command | Command contents, not top-level declarations; expansion tests preserve word boundaries and native bytes |
| Flat top-level single-string node, alone or beside a wrapper | Rejected; survivor inventory below |
| Invalid modular shapes/references/values, typed nodes, malformed KDL and unknown nested nodes | Deliberate error fixtures, not another accepted grammar |
| Opaque invalid active policy used during example installation | Preservation/no-read fixtures in `example_installation.rs`; no successful configuration load is claimed |
| Non-configuration uses of “flat”, “legacy”, “template” and quoted tokens | CLI surfaces, retired task-tree layouts, prompt templates, shell scripts and historical research; outside this grammar |

All packaged `.kdl` files are in `docs/examples/modular-configuration/`: the
personal modular example and the Codex-led, Claude-led, high-effort and local
override deltas. Their existing installation and launch tests exercise the
shipped bytes. Fixture helpers in `testing/support.rs` and runner, loop and CLI
tests construct wrappers and explicit bindings/routes, including dynamically
named kinds; they do not translate old flat input for successful loads.

## Old-form survivors

| Location | Why it remains |
|---|---|
| `crates/keyed-launch/tests/catalog.rs` | `opaque` and `local` top-level nodes assert capture rejection; a nested `opaque` node asserts invalid profile shape. The public two-loader matrix generates ordinary and grammar-word keys, alone and before/after a wrapper, in each source; it checks exact spans, roles, remedy and unchanged input. |
| `crates/keyed-launch/tests/templates.rs` | The `three` node asserts an invalid overlay fails with its own source path and unsupported-declaration error. |
| `crates/keyed-launch/tests/named_commands.rs` | Mixed `alpha` input and a flat `k` input assert load failure. They are not successful scanner examples. |
| `crates/grove-loop/tests/session_config.rs` | Flat `impl` and grammar-word `config` nodes beside a wrapper assert delta-attributed rejection and unchanged bytes. |
| `docs/CONFIGURATION.md` | A single-string node named `config` explicitly illustrates unsupported input beside a wrapper. |
| `docs/preservation-baseline.md` | The v19.3.0 flat grammar and whole-template delta are historical measurements; the configuration section now states the approved modular-only exception. |
| `CHANGELOG.md`, completed `.grove/` tasks and ancestor handoffs | Release history and records of intermediate migration states, not current executable configuration instructions. |
| This audit and removal explanations | Names of retired forms identify rejection evidence and historical material, never an additional loading path. |

The current walkthrough examples now use wrappers, commands, bindings and routes.
Their provenance explanations distinguish the personal command-definition path
returned by `source()` from all contributing origins exposed by `inspect()`.
The architecture, structure brief, shipped Grove driver guidance and validation
diagram describe the same composition and validation scopes.

## Verification boundary

`bash scripts/check.sh` is the required verification command, including the
public load/expansion suites and final source-exact validation of every book.
The task commit records its observed result. `modular-audit-k17` reviewed
rejection, authority, expansion, provenance and this audit against the completed
subsystem; its findings are classified in `modular-audit-k18`. This inventory
does not substitute for that review.
