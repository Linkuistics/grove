# A skill states what binds when its dependencies are absent

A skill that cites another skill must state what binds when that skill is
absent. Grove's methodology is itself a set of skill packs now. Core's `grove`
plugin has the shared spine and the kind skills of core. An external pack can
ship other kind skills. The methodology also cites craft skills, which other
plugins ship. A citation names its skill bare, as the
[external pack contract](../specs/external-pack-contract.md) requires.

Grove's formula carries core and the five domain packs. Other formulae carry
the personal pack and the third-party craft skills. The formulae that a user
installs decide the sources that a session receives, and explicit Ravel
settings can replace them. Any craft skill can be absent from a session.
Every citation of one must therefore stay meaningful when its target skill is
absent.

**A directed load is a required dependency, not a citation.** A `grove-<kind>`
skill directs a load of the `grove` spine. A member of a family whose kinds all
ship in one external pack also directs a load of its family skill. The session
cannot do its work without a skill that it must load, so no fallback can state
what binds instead.

Inside one pack, the loaded skill ships with the member. It cannot be absent
while the member is present, so the member has nothing to state. Across packs,
an installation can omit the pack that holds the loaded skill. The
[external pack contract](../specs/external-pack-contract.md) then requires two
things. The pack declares the dependency to each delivery route. The loading
text sends the session to the launch prompt's rule for a missing kind skill.

Aggregate conformance proves that each such edge resolves in the supplied
packs. A skill that a member only *cites* can be absent on any route, and the
rest of this record is about that case.

**Every citation states what binds in its absence.** The question that decides
it applies to each citation and has one form. *Does the absence change what a
session writes, or only how well the session writes it?* Grove owns locally the
absence that changes **what**: grove states the rule itself, and the citation
becomes attribution. Grove defers the absence that changes **how well**, and the
deferring sentence names the grove file that holds the part that binds.

A citation that leaves this unstated is the silent dependency, whichever way
the decision went.

**The local statement is the rule, never a summary of the skill.** Grove owns
its ADR when-to-write test, its seam-selection rules and its commit boundary,
because each one changes what a session *writes*. It owns none of the reasoning
behind them. A fallback that taught what the skill teaches would make the plugin
pointless and double the surface that can drift. This record is as much about
that failure as about the silent dependency.

**One statement, reached by a pointer.** The spine's `references/grove.md` holds
what binds for every cited craft skill in one place. The trigger sentence of the
spine's `SKILL.md` sends a session that meets any citation there. A file whose
own operative rules are complete says so in its own words. No file mirrors the
gloss of the hub. A second statement of it is the duplication that
[every normative rule has one owner](corpus-rules-have-one-owner.md) exists to
prevent.

## The trade-off

A local statement of a rule that the plugin also states is a **permitted
mirror**. This repository holds that each rule has one canonical source, and a
fallback is a second source by construction. The mirror is permitted only where
absence changes what a session writes. `plugins/grove/conformance/rules.tsv`
records each mirror with its reason. That limit keeps the exception from
becoming the rule.

**Rejected: make the plugins of the craft skills a hard installation
dependency of the `grove` plugin.** A Codex bundle of the plugins makes them
available together. It does
not make the methodology's rules conditional on an installer. Each source
stays a separate Ravel source, and a grove tree must stay legible
with Grove deleted. A methodology that cannot state its own rules would violate that
walk-away property.

**Rejected: extend the mandate's rule for a missing skill to each skill that a
kind loads.** The rule rides the launch prompt because a missing kind skill
cannot state it. A kind skill that is present can state the outcome for the
skills that it loads. The too-late test admits to the prompt only a sentence
that the skill cannot repair after it loads. This sentence therefore belongs in
the kind skill.

**Rejected: defer everything, and state nothing locally.** This was the earlier
state, and it caused a measured defect, not a hypothetical one. The ADR
when-to-write test was only cited, so grove had no limit of its own. The corpus
held an OR-form paraphrase of the plugin's AND test for as long as nothing local
contradicted it. A cited test is no test.

The registry of `crates/grove/tests/plugin_fallback.rs` lists core's citations
and the decision for each. It asserts both halves over core's shipped skill set.
Each citation states what binds, and the list is complete. A citation added later
to core without a binding sentence therefore fails before it reaches a session.

The registry covers core alone. Each pack pins its own citations as rows of its
conformance manifest. The pack alone then checks that the owner states the
binding sentence and that no other file of the pack states it. DesignSkills
pins the citation of `codebase-design` in `grove-design/SKILL.md` this way. No
sweep finds an unregistered citation in a pack. Reopen that gap when a pack
adds a citation of a craft skill.
