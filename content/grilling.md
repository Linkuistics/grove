<!-- bundled in grove from mattpocock/skills@d574778f94cf620fcc8ce741584093bc650a61d3 (skills/productivity/grilling/SKILL.md + skills/engineering/domain-modeling/SKILL.md) — MIT licensed; see LICENSES/mattpocock-skills.LICENSE -->
<!-- intentionally fused — upstream split them; grove has no skill-to-skill invocation, so the split would be cosmetic -->

# Grilling — the planning-task interrogation procedure

<what-to-do>

Interview me relentlessly about every aspect of this plan until we reach a shared understanding. Walk down each branch of the design tree, resolving dependencies between decisions one-by-one. For each question, provide your recommended answer.

Ask the questions one at a time, waiting for feedback on each question before continuing. Asking multiple questions at once is bewildering.

If a *fact* can be found by exploring the codebase, look it up rather than asking me. The *decisions*, though, are mine — put each one to me and wait for my answer.

Do not commit decisions or grow the tree until I confirm we have reached a shared understanding.

</what-to-do>

<supporting-info>

## Domain awareness

During codebase exploration, also look for existing documentation:

### File structure

Most repos have a single context:

```
/
├── CONTEXT.md
├── docs/
│   └── adr/
│       ├── event-sourced-orders.md
│       └── postgres-for-write-model.md
└── src/
```

If a `CONTEXT-MAP.md` exists at the root, the repo has multiple contexts. The map points to where each one lives:

```
/
├── CONTEXT-MAP.md
├── docs/
│   └── adr/                          ← system-wide decisions
├── src/
│   ├── ordering/
│   │   ├── CONTEXT.md
│   │   └── docs/adr/                 ← context-specific decisions
│   └── billing/
│       ├── CONTEXT.md
│       └── docs/adr/
```

Create files lazily — only when you have something to write. If no `CONTEXT.md` exists, create one when the first term is resolved. If no `docs/adr/` exists, create it when the first ADR is needed.

## During the session

### Challenge against the glossary

When the user uses a term that conflicts with the existing language in `CONTEXT.md`, call it out immediately. "Your glossary defines 'cancellation' as X, but you seem to mean Y — which is it?"

### Sharpen fuzzy language

When the user uses vague or overloaded terms, propose a precise canonical term. "You're saying 'account' — do you mean the Customer or the User? Those are different things."

### Discuss concrete scenarios

When domain relationships are being discussed, stress-test them with specific scenarios. Invent scenarios that probe edge cases and force the user to be precise about the boundaries between concepts.

### Cross-reference with code

When the user states how something works, check whether the code agrees. If you find a contradiction, surface it: "Your code cancels entire Orders, but you just said partial cancellation is possible — which is right?"

### Update CONTEXT.md inline

When a term is resolved, update `CONTEXT.md` right there. Don't batch these up — capture them as they happen. Use the format in [CONTEXT-FORMAT.md](./CONTEXT-FORMAT.md).

`CONTEXT.md` should be totally devoid of implementation details. Do not treat `CONTEXT.md` as a spec, a scratch pad, or a repository for implementation decisions. It is a glossary and nothing else.

### Offer ADRs sparingly

Offer to create an ADR only when the `linkuistics:decision-records` when-to-write
test holds (hard to reverse · surprising without context · the result of a real
trade-off — all three). That skill owns the test, format, and template;
[ADR-FORMAT.md](./ADR-FORMAT.md) adds grove's placement conventions.

### Agree the test seams

When the increment covers code that will be tested, sketch the seams the work will
be tested through and put them to the user before the design is committed: "these
are the seams — do they match what you expected?" Prefer existing seams to new
ones, propose any new seam at the highest point you can, and drive the count down —
the ideal number is one. Record the agreement in the spec's `## Test seams`, or,
when the increment writes no spec, in the node's `BRIEF.md`
([SPEC-FORMAT.md](./SPEC-FORMAT.md)). For what a seam is and how to judge one, use
the `linkuistics:codebase-design` skill.

</supporting-info>
