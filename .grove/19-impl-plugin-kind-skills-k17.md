# plugin-kind-skills-k17

## Goal

Ship one `grove-<kind>` skill per kind over the spine, so that **a kind exists
iff a skill of that name exists**.

## Context

`docs/specs/module-decomposition.md`, decision 11, and `plugin-spine-k16`, which
this leaf completes.

**The fatness rule**, the inline half: every rule owned by that kind or its
family goes in `grove-<kind>` — its goal, its deliverable, its
human-in-the-loop mark, its review allowance, and whether it passes the done flag
when it signals. Nowhere twice.

**Where a rule belongs to a *family* rather than one kind** — the five reviews,
the two research halves — the family's text is **one file in the spine**, and
each member's skill directs a load of it **by name** in its opening imperative.
A directed load is not a selection, which is the property
`docs/research/wording-micro-test.md` found load-bearing.

## Done when

- Nineteen `grove-<kind>` skills exist, matching the kinds in `content/`'s
  routing table today — five producers each with `review-` and
  `integrate-review-` steps, `research-a`, `research-b`, `combine-research`, and
  `finish`.
- **The fatness rule is delivered in the spec's own two halves, not collapsed into
  one.** A rule owned by *that kind alone* — its goal, its deliverable, its
  human-in-the-loop mark, its review allowance, whether it passes the done flag —
  is **inline** in `grove-<kind>`. A rule owned by a *family* — the five reviews,
  the two research halves — is **one file in the spine**, and each member's skill
  directs a load of it **by name** in its opening imperative; it is not restated in
  the member. Same for the rules shared across families. **Nowhere twice**, which
  is the property the conformance runner below actually checks and the reason the
  two halves cannot both claim the family text. Each skill declares its own
  `harnesses:` eligibility.
- The conformance runner from k16 passes over the full set: every behavioural
  rule present on the composed loaded path of every kind that binds it, no rule
  with two owners, every named path existing.
- The Rust corpus suites that the runner now covers are deleted, and the commit
  message names each one and the runner assertion that replaced it. Do not delete
  one whose assertion has no home yet.
- `docs/adr/behavioural-coverage-asserts-delivery.md` **amended** — the rule
  survives, the instrument moves to the plugin's shell runner, and two of the
  four things its walk covers no longer exist in the binary.
- `docs/adr/corpus-rules-have-one-owner.md` **amended** — the filing rule
  survives unchanged; its register is the plugin's spine rather than an embedded
  `content/`, its reachability edge is the composed loaded path rather than a
  prompt module, and its all-nineteen mapping loses the set it quantified over.
- `docs/adr/restatement-declares-its-class.md` **amended** — the class
  distinction survives; the condition register relocates from the embedded corpus
  to the plugin spine.
- `docs/adr/grove-binds-without-the-plugin.md` **reworked** — its current-state
  opening is that the binary sweeps its own `content/` into every harness's skill
  directory. After this the methodology *is* a plugin, so the record's subject
  changes from *grove binds without `linkuistics`* to *what binds when a skill's
  own dependencies are absent*.
- `CHANGELOG.md` updated.

## Notes

**Lands green**, and still removes nothing from the binary — `content/` is
untouched here and dies at `delete-provisioning-k19`.

**Record the gap rather than claiming it closed.** The wording micro-test
measured **one** hop, from a prompt naming two targets. Nothing measures the
second hop, from `grove-<kind>` to the spine. What is inline is unaffected; what
is in the spine loses its guarantee; the reopen condition is **a session observed
acting without a spine rule**. Write that into the plugin's own documentation
where a future session will meet it, not only into a commit message.

**The nineteen are what the methodology declares today, not a set the machinery
holds.** Nothing in `src/` may gain a list of them, and nothing in the runner may
assert how many there are. If this leaf finds itself writing a manifest, that is
the signal it has drifted.

**Expect this leaf to be the largest authoring session in the tree.** If it
proves bigger than one session, `grove-llm leaf-decompose` is the answer and the
natural first child is the five-review family, which is where the *nowhere twice*
rule is hardest.
