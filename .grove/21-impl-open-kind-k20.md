# open-kind-k20

## Goal

Open `Kind`. Grove names a kind only where grove **writes** the leaf — two
tokens, `requirements` and `finish` — and holds no enumeration of kinds anywhere
in the machinery.

## Context

`docs/specs/module-decomposition.md`, decision 5, and its requirement *grove
names only the kinds it writes* with three scenarios.

**The licensing rule**, which is decision 9's loop contract restated: the loop
mutates the tree only where **no session exists to delegate to** — root
scaffolding before the first session, and the finish sentinel between the last
ordinary session and the finish session. Those two writes mint the only two
leaves grove itself authors, and they are the only two kinds it may name.

That resolves the three surviving matched places, all of which are grove
recognising the leaf it wrote itself rather than interpreting the methodology:
`finish` sorting last in selection, `finish` being refused to the grow verbs, and
`requirements` being root-init's default (a **default**, because `root-init`
takes a kind option). The other two matched places — kind→reference-file and
kind→session-ending-file — died at `prompt-names-the-kind-k18`, and the
completeness check derived from `Kind::ALL` died at `keyed-launch-templates-k10`.

**Two further sites carry a kind literal, and neither is a `match`** — which is
why counting `match` arms has undercounted this surface twice. Both are removed
rather than resolved:

- **The research-pair verb.** Its three kinds are a constant in the machinery —
  a fourth token beside the two grove may name. The **atomicity is real and
  kept**: three separate appends are three snapshots and three chances to stop
  half way, and a live prefix of a pair is indistinguishable from a deliberately
  hand-cut partial one. So the verb is **generalised, not deleted** — the
  ordinary add takes an ordered list of kinds and appends them as one unit, at
  consecutive ordinals with consecutive keys. `leaf-add <parent> <stem> --kind
  research-a --kind research-b --kind combine-research` is then the pair, spelled
  by the methodology that owns those three tokens; a one-kind list is the
  ordinary add. **Twelve verbs, not thirteen.** Deleting the verb and letting the
  skill call `leaf-add` three times is **rejected** — it reintroduces the
  live-prefix hazard the verb's own contract exists to exclude.
- **`--kind` defaulting to `impl` on add and insert.** A default is a literal
  under a friendlier name, and it is the one kind literal that would silently
  produce a **wrong** leaf rather than an error. `--kind` becomes **required** on
  both. Root scaffolding keeps its default, because `requirements` is one of the
  two leaves grove authors.

## Done when

- `Kind::new` validates the token's **shape** — non-empty, lowercase ASCII
  letters, digits and single hyphens, no separator, not a reserved word — and
  nothing else. `Kind::ALL`, the nineteen-arm parse and `TaskNameError`'s
  unknown-kind variant are gone; what replaces the last is a **shape refusal
  naming the character it refused**.
- A leaf carrying a kind for which no skill is installed **parses**, the launch
  **proceeds**, and the failure is reported by the session that could not load
  the skill.
- A leaf of kind K whose template does not resolve is refused **before the tree
  is mutated**, naming K and the file that should declare it.
- `leaf-add` takes an ordered list of kinds and lands them as one unit or not at
  all; `leaf-add-pair` is gone; `--kind` is required on `leaf-add` and
  `leaf-insert`.
- **No list of kinds appears in the machinery.** Verify by enumerating every
  string literal that is or could be a kind across the whole of `src/` and
  classifying each — not by sweeping a pattern list, which is complete only as
  far as the list. `references/execute.md` carries the method and its controls,
  and this surface has already defeated two counts.
- `docs/adr/a-kind-is-an-open-token.md` is **added**. The compiled enum going
  open is hard to reverse (it forces the filename grammar), surprising, and
  carries two rejected alternatives — a kind manifest, and enumeration by reading
  the installed skill set. What it costs is a typo'd kind failing at `leaf-add`
  rather than at compile time.
- **The plugin's own text is updated in the same commit**: the spine's
  decomposition procedure describes `leaf-add-pair` today and must describe the
  kind list instead; every skill that tells a session to invoke `leaf-add` or
  `leaf-insert` must pass `--kind`. The corpus is the plugin's now, so this is
  the only copy.
- The binaries are **rebuilt and reinstalled in this session** — see below.
- `cargo test` and `cargo clippy --all-targets` clean; `CHANGELOG.md` updated.

## Notes

**Lands green**, and it is the **contract** stage of the kind work.

**Reinstall in this session, and do not expect the loop to stop for you.** This
leaf changes the verb surface every later session invokes: `--kind` becomes
required and `leaf-add-pair` disappears, while the corpus starts telling sessions
to pass a kind list the *old* installed binary rejects. Follow
`grammar-separator-k15`'s install sequence. Unlike that leaf, the build-pairing
guard that stopped the loop between iterations was retired at
`prompt-names-the-kind-k18`, so the running driver will keep going on its own
code — which is fine for the driver and wrong for the sessions it launches.
Verify `grove-llm leaf-add --help` from the resolved `PATH` before committing.

**The rule is the test.** After this leaf: *grove names a kind only where grove
writes the leaf, and the two tokens are the whole of it.*
