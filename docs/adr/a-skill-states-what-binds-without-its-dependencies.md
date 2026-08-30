# A skill states what binds when its dependencies are absent

A skill that cites another skill must state what binds in that skill's absence.
Grove's methodology is itself a skill set now — the `grove` plugin, a shared
spine and one `grove-<kind>` skill per kind — and it cites `linkuistics` skills
it does not install: the two plugins are separate marketplace entries and
separate rows in `plugins/install.sh`'s symlink farm, even though both are
developed in this repository. So an installation can be one where every
`linkuistics:` citation points at a skill that is not there.

**A skill's dependency on its own spine is a different relation and is not
covered here.** A `grove-<kind>` skill directs a load of the `grove` spine, and
the two ship in one plugin: the spine cannot be absent while the member is
present, so there is nothing for the member to state. What can be absent is a
skill from *another* plugin, which is the case below.

**Every citation states what binds in its absence.** The generating question is
per citation and has one form: *does the absence change what a session writes, or
only how well it writes it?* Absence that changes **what** is owned locally —
grove states the rule itself, and the citation becomes attribution. Absence that
changes **how well** is deferred — and the deferring sentence says which
grove-local file carries the part that binds. A citation that leaves that
unstated is the silent dependency, whichever way it was decided.

**The local statement is the rule, never a précis of the skill.** Grove owns its
ADR when-to-write test, its seam-selection rules and its commit boundary because
each one changes what a session *writes*; it owns none of the reasoning behind
them. A fallback that taught what the skill teaches would make the plugin
pointless and double the drift surface, which is the failure this record is as
much about as the silent dependency.

**One statement, reached by a pointer.** The spine's `references/grove.md`
carries what binds for all three skills in one place, and the spine's `SKILL.md`
trigger sentence routes a session meeting any citation there. A file whose own operative
rules are complete says so in its own words; no file mirrors the hub's gloss,
because a second statement of it is the duplication
[every normative rule has one owner](corpus-rules-have-one-owner.md) exists to
remove.

## The trade-off

A local statement of a rule the plugin also states is a **permitted mirror** —
this repository's thesis is one canonical source per rule, and a fallback is a
second source by construction. The mirror is admitted only where absence changes
what a session writes, and each one is recorded with its reason in
`plugins/grove/conformance/rules.tsv`. That bound is what keeps the exception
from becoming the rule.

**Rejected: make `linkuistics` a hard install dependency of the `grove` plugin.**
It would remove the mirrors outright, and it is the wrong shape twice. The two
contexts ship as separate products on purpose — `docs/ARCHITECTURE.md`,
*Repository products*, describes the split — so enforcing the dependency means
one context installing another it does not own; and it violates
walk-away-ability — a grove tree must stay legible with grove deleted, which a
methodology that cannot state its own rules is not.

**Rejected: defer everything and state nothing locally.** This was the prior
state, and it produced a measurable defect rather than a hypothetical one: with
the ADR when-to-write test only cited, grove had no bar of its own, and the
corpus carried an OR-form paraphrase of the plugin's AND test for as long as
nothing local contradicted it. A cited test is no test.

The enumeration of citations and their per-row decisions is
`tests/plugin_fallback.rs`'s own registry, which asserts both halves over the
shipped skill set — that each citation states what binds, and that the
enumeration is exhaustive, so a citation added later without a binding sentence
fails before it reaches a session.
