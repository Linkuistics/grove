# grove is a self-extension core plus its methodology, bundled in one skill

grove is two things and nothing else:

1. **A self-extension core** — the `.grove/` task tree as data, the `pick` walk, the
   grow verbs (`root-init` / `leaf-add` / `leaf-insert` / `leaf-decompose` /
   `leaf-retire` / `brief-chain` / `resolve`), the two task kinds, and the minimal
   loop skeleton.
2. **Its distinctive methodology** — grilling, the `driving.md` habits, the loop
   discipline, and the CONTEXT / ADR / SPEC format guides.

Both are **bundled in a single global skill**. Distribution is one
`brew install grove` binary that embeds the methodology `content/` and provisions the
global skill on launch. grove owns no runtime machinery: worktree lifecycle,
fresh-context looping, and the human-as-scheduler crank are supplied by the
self-driving loop (see *self-driving-loop*); generic engineering skills (TDD, review,
debugging) are deferred to third-party collections grove never had superior versions
of.

## Why: less machinery, not less wisdom

The guiding directive is "less in grove," and the key distinction is *what* to make
less of. The methodology is **markdown** — nearly free to carry, proven in practice,
and superior in many ways to third-party collections. The machinery is **code** —
expensive, brittle, and largely a re-implementation of what a harness or substrate
already provides. So grove keeps the wisdom (it is cheap and distinctive) and sheds
the machinery (it is costly and duplicative). Keeping the methodology is also *less
work* than re-homing it — nothing methodological has to move.

Concretely, grove is **not** a terminal UI, a fleet dashboard, an inbox / cross-repo
observation subsystem, or an install-materialise-with-drift-tracking layer. Those
were code re-implementing substrate concerns; they are gone, and their absence is the
point. What remains is the tree verbs, the loop driver, the completion signal, and the
methodology prose.

## Distribution collapses to one binary + one skill

Because the binary embeds the methodology and re-extracts it to the global skill on
launch, there is no per-worktree materialised copy to drift and no multi-artifact
version model. One binary, one global skill it provisions, always matching.

## Consequences

- The surviving code is small: the tree verbs, the loop driver, and the signal verb.
  The surviving *value* is the methodology, intact.
- The decision is reversible in principle — the shed machinery lives in git history —
  but expensive to reverse in practice; it clears the ADR bar as a durable trade-off.
- It pairs with *self-driving-loop*, which supplies the runtime the shed machinery
  used to provide.
