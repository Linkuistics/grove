# API Interface Language

AIL is a proposed language for writing down the complete [binding contract](glossary.md#binding-contract)
of an existing API, once, for every consumer. It is intended for .NET, Linux
(Wayland, GTK and KDE), Rust and other typed libraries, and for Apple APIs.
Interpreted and dynamically typed languages are among the intended binding
targets. Its status is draft 0.1, a proposal under review; no parser, validator
or implementation exists.

> **Purpose:** orientation — understand AIL's purpose and choose a reading route.
> **Reader:** language designers new to [API binding](glossary.md#binding); API authors and tool implementers.
> **Before this:** nothing.
> **Status:** draft 0.1, a proposal under review; no parser, validator or implementation exists.
> **Rules:** [language](language.md), [contracts](contracts.md), [profiles](profiles.md) and [grammar](grammar.ebnf).

## Who this documentation serves

The primary reader knows language design and wants to judge AIL's notation,
concepts and authoring burden. API authors and tool implementers also need exact
rules. Useful feedback cites a rule or example, explains the difficulty, and
challenges the design where warranted.

## The problem in brief

A signature says which values cross a call, but may leave their release, validity
and [callback](glossary.md#callback) timing unstated. Human authors, analysers and
third-party tools are intended to produce AIL [packages](glossary.md#package);
[validation](glossary.md#validation) checks completeness and consistency
before binding generators, documentation renderers and LLM tooling consume the
same contract. Start with [What a signature leaves unsaid](explanation/unsaid.md).

The map shows that every producer crosses the same publication boundary and
every consumer reads the same authoritative contract. The package pins its
[profile](glossary.md#profile) and [ABI](glossary.md#abi) catalogue dependencies
by identifier, version and [digest](glossary.md#digest-pin); validation checks
completeness and consistency, not whether a binary honours the assertions.
These are proposed roles, not running tools (non-normative;
[`L2–L3`](language.md#l2), [`C8`](contracts.md#c8),
[`A1`](profiles.md#a1), [`A5`](profiles.md#a5)).

![Producers submit to publication validation; the authoritative package pins profile and ABI entries and supplies the same contract to all consumers.](figures/g1-context.svg)

[Diagram source](figures/g1-context.puml) ·
[Figure vocabulary and regeneration](figures/style/vocabulary.txt)

## Paths through the documentation

Choose a starting page by the task you want to do; the table does not require
you to read every route in order.

| If you want to… | Start at… |
|---|---|
| Understand why the contract is needed | [What a signature leaves unsaid](explanation/unsaid.md) |
| Learn to read the notation | [Synthetic buffer example](examples/buffers.ail) alongside [its conformance context](conformance.md); the tutorial *Describing a small API* is forthcoming |
| Review the design | [Conformance examples and pressure cases](conformance.md); the review guide *Reviewing AIL* is forthcoming |
| Look up a term or exact rule | [Glossary](glossary.md), then the canonical files below; the indexed *Reference* is forthcoming |

The next explanations are *How AIL states a contract*, *Packages, environments
and access paths*, and *Design choices, limits and open questions* (forthcoming).
For now, the domain page leads directly to the canonical rules.

## What is normative

The following files are normative **within this draft**:

- [Language and publication rules](language.md) (`L1`–`L6`), with the [EBNF grammar](grammar.ebnf).
- [Memory, effects and behavioural contracts](contracts.md) (`C1`–`C8`).
- [Profiles, access paths and ABI references](profiles.md) (`A1`–`A5`), including the [synthetic fixture catalogue](profiles.md#f1) (`F1`).

Rule identifiers are stable citation targets; wording can change during review.
Explanations and glossary entries are non-normative summaries, not additional
rules. The supporting material is:

- [Conformance requirements, mutations and review limits](conformance.md).
- Synthetic API examples: [buffers and output states](examples/buffers.ail),
  [callbacks and teardown](examples/callbacks.ail), and
  [objects and access alternatives](examples/objects.ail).
- [Mapping from the existing implementation](baseline.md), for maintainers.

These examples use a synthetic fixture [profile](glossary.md#profile) (a language
extension, not a preference preset), catalogue and [digest tokens](glossary.md#digest-pin);
they do not describe real operating-system APIs. No native ABI provider, binding
implementation or executable conformance suite is delivered here. The
[verification record](conformance.md#verification-record) describes historical
static checks and their limits, not an available AIL validator.

## Scope of draft 0.1

This is the first normative draft, dated 23 September 2026. The architecture is
approved; the detailed language is proposed for review. “API Interface Language”
and `.ail` are working names. AIL grew from APIAnyware's API-description work and
is not its implemented format.

In summary (non-normative; [`L2–L4`](language.md#l2) and [`A1–A2`](profiles.md#a1)):
an AIL package authoritatively describes a declared API [surface](glossary.md#surface),
its meaning, source identities and [access paths](glossary.md#access-path). Humans, analysers
and third-party producers have identical publication obligations. Incomplete
analysis, competing claims, confidence scores and unresolved facts belong
outside the package. An intentionally [opaque resource](glossary.md#opaque-resource)
is a complete contract about hidden representation, not missing analysis. A
package names versioned [ABI](glossary.md#abi) definitions supplied by tooling;
it does not define an ABI.

The core covers named declarations, scalar and aggregate types, explicit
ownership and validity, [outcomes](glossary.md#outcome),
callback schedules, [concurrency](glossary.md#concurrency), properties,
relationships and reusable [patterns](glossary.md#pattern). A small closed
predicate language makes
outcome coverage checkable ([`L4–L6`](language.md#l4), [`C1–C8`](contracts.md#c1)).
[Profiles](glossary.md#profile), which are language extensions rather than
preference presets, add source-language and access semantics through typed,
versioned clauses; there are no uninterpreted normative attributes
([`L1`](language.md#l1), [`A1`](profiles.md#a1)).

Draft 0.1 uses a finite set of exact deployment [environments](glossary.md#environment).
It does not yet
define version-range algebra, arbitrary dependent types, generic constraint
solving, bitfield layout, unions, or complete Swift/Rust/Zig/CLR/Wayland profiles.
An API needing such a feature needs a fully specified profile or a later
language version; erasing the feature cannot make the package complete. Fully
specified, profile-supported source-only access is legitimate
([`L3–L4`](language.md#l3), [`A1–A4`](profiles.md#a1);
[pressure cases](conformance.md#pressure-cases-outside-the-synthetic-profile)).

These limits do not introduce a partial-specification mode. A package may describe
a named subset of a larger API when that subset and its entire
[dependency closure](glossary.md#dependency-closure) are specified
([`L2–L3`](language.md#l2)). The forthcoming *Design choices, limits and open
questions* explains the trade-offs; the existing pressure cases above show the
boundary now.

## Design history

The [approved architecture](../../docs/design/2026-09-22-interface-language.md),
[initial language survey](../../docs/research/2026-09-22-interface-language-a.md)
and [specification-writing plan](../../docs/design/2026-09-22-interface-language-plan.md)
record the design basis. The
[documentation survey](../../docs/research/documentation-basis-a.md),
[documentation audit](../../docs/research/interface-language-documentation-audit.md)
and [documentation plan](../../docs/design/2026-09-23-interface-language-documentation-plan.md)
explain this reader-oriented presentation. Production libraries, parsers and
APIAnyware adoption belong to separate phases.

Next: [What a signature leaves unsaid](explanation/unsaid.md).
