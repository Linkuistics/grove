# A skill states what binds when its dependencies are absent

A skill that cites another skill must state what binds when that skill is
absent. Grove's methodology is itself a skill set now. It is the `grove` plugin,
with a shared spine and one `grove-<kind>` skill for each kind. It cites
`linkuistics` skills from a separate plugin. Users install the two plugins
independently, through the marketplace or by hand, and an installation can
omit one. Every `linkuistics:` citation must therefore stay meaningful when its
target skill is absent.

**A skill's dependency on its own spine is a different relation, and this
record does not cover it.** A `grove-<kind>` skill directs a load of the `grove`
spine, and the two ship in one plugin. The spine cannot be absent while the
member is present, so the member has nothing to state. A skill from *another*
plugin can be absent, and that is the case below.

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
what binds for all three skills in one place. The trigger sentence of the
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

**Rejected: make `linkuistics` a hard installation dependency of the `grove`
plugin.** A Codex bundle of both plugins makes them available together. It does
not make the methodology's rules conditional on an installer. Marketplace
entries stay independent, and a grove tree must stay legible with Grove
deleted. A methodology that cannot state its own rules would violate that
walk-away property.

**Rejected: defer everything, and state nothing locally.** This was the earlier
state, and it caused a measured defect, not a hypothetical one. The ADR
when-to-write test was only cited, so grove had no limit of its own. The corpus
held an OR-form paraphrase of the plugin's AND test for as long as nothing local
contradicted it. A cited test is no test.

The registry of `crates/grove/tests/plugin_fallback.rs` lists the citations and
the decision for each. It asserts both halves over the shipped skill set. Each
citation states what binds, and the list is complete. A citation added later
without a binding sentence therefore fails before it reaches a session.
