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

**Reinstall in this session, and stop the loop the way every cutover does.** This
leaf changes the verb surface every later session invokes: `--kind` becomes
required and `leaf-add-pair` disappears, while the corpus starts telling sessions
to pass a kind list the *old* installed binary rejects. Run the root brief's
`### The cutover sequence` — the same one `grammar-separator-k15` runs, with no
difference at this leaf, because there is no guard here and there was none there
either. The running driver keeps its own kind set and its own verbs, so the
handoff is the unsignalled exit, not the install. The behavioural probe for step 3
is `grove-llm leaf-add --help` from the resolved `PATH`, which must show `--kind`
required and no `leaf-add-pair` beside it.

**The rule is the test.** After this leaf: *grove names a kind only where grove
writes the leaf, and the two tokens are the whole of it.*

## Decisions (running log)

**`Kind` moved into `src/task_name.rs`, and `src/leaf.rs` is deleted.** The
module's stated reason for holding `Kind` apart from the other three name
components was that the set was closed and the label doubled as a configuration
key. Opening the set leaves `Kind` as exactly what `Slug` is — a validated word
of the filename grammar — and the *canonicity* of a leaf name depends on the two
words obeying the **same** shape rule, which is a grammar fact. So one private
`refuse_token(noun, token)` states the shape once and `Kind::new` / `Slug::new`
are two nouns over it. A second validator would have been a second statement of
a rule the two must share; the leading/trailing-dash clause is the proof that
they must — a kind ending in `-` renders `impl---slug`, which splits at the
first `--` and reads back as a different kind and a different slug.

**One error type, `TokenError`, replaces `SlugError`.** Its `reason` became an
owned `String` because the Done-when requires the refusal to name the character
it refused, which a `&'static str` cannot. `TaskNameError::UnknownKind` became
`BadKind { name, kind, error }`, mirroring `BadSlug`.

**Grove spells its two tokens through two constructors and one predicate** —
`Kind::requirements()`, `Kind::finish()`, `Kind::is_finish()` — so the two
literals have one home and the three surviving recognition sites (selection
ordering, the grow verbs' refusal, teardown) ask a question rather than carry a
token.

**The `work` → `impl` alias is deleted.** It refused a well-formed token by
naming a replacement, which is grove holding an opinion about a kind's *meaning*
— exactly what this leaf removes. `work` is now an ordinary kind, and whether a
skill exists for it is the methodology's answer to give.

**`root-init` keeps its kind fixed rather than gaining a `--kind` option.** Both
the spec (decision 5) and this leaf's `## Context` describe root-init as *taking
a kind option*, so that `requirements` is "a default rather than a constant".
It does not take one, and never has — `RootInitArgs` carries only a slug. The
rule this leaf must land is unaffected either way (`requirements` is one of the
two leaves grove authors, so naming it is licensed whether it is a default or a
constant), and adding an option no `## Done when` line asks for would grow the
verb surface in the leaf that exists to shrink it. Recorded here and carried to
`spec-to-current-state-k23`, which rewrites the spec to current state and would
otherwise re-assert it.

**The no-list-of-kinds claim, enumerated rather than swept.** Every string
literal in `src/` was extracted (2,486) and filtered by the kind-token
predicate — non-empty, lowercase ASCII letters, digits, single hyphens, no
`--` — which is complete by construction, since a kind literal must be a string
of that shape. 698 candidates; 48 once test regions are excluded. Classified,
all 48: binary and namespace names (`grove`, `grove-llm`), verb names passed to
refusals (`leaf-add`, `leaf-retire`, …), lease record fields, completion
dispositions, template slot names, flag long-names (`kind` is the *flag*), the
`slug`/`session kind` nouns `refuse_token` is parameterised on, `stty sane`, and
two **slugs** — `plan` (root-init's default slug) and `finish` (the sentinel
leaf's slug, `NN-finish--finish-kN.md`). Exactly **two** are kind literals:
`requirements` and `finish`, both in `Kind`'s two constructors. No `format!` or
`concat!` in production builds a kind string either.

Three controls, each watched failing first: a planted roster in a copied tree
came back **dirty** (six tokens, including two from a `fn` rather than an array);
redacting the two licensed constants made them **disappear** from the reading, so
the instrument reads those lines rather than printing them unconditionally; and
the same instrument over `tests/` found 151. The corpus run initially read `0` —
a false clean from a `*.rs` glob over a markdown tree, which is the mis-scope
`references/execute.md` warns about; matched to the tree it finds 269, which is
where the kind set legitimately lives now.

**Cutover: re-derived, and confirmed — but for a corpus reason, not a tree
one.** The root brief requires each remaining leaf to re-run k6's matrix rather
than inherit the label. On the **tree**, no cell fails: this leaf adds, removes
and renames nothing in `.grove/`, every leaf still carries one of the nineteen,
and the installed 19.5.0 build reads the tree this leaf leaves exactly as before.
The failing cell is elsewhere. The **corpus** now tells a session to cut a
research pair as `leaf-add <parent> <stem> --kind research-a --kind research-b
--kind combine-research`, and 19.5.0 refuses a repeated `--kind`; `leaf-add-pair`
is documented nowhere any more, so a session needing a pair has no working route
against the old binary. That is the installed build meeting what this leaf leaves
and failing, so the cutover sequence runs. The latent second cell — a later
session writing a leaf whose kind the old enum does not hold, which 19.5.0 reads
as `UnknownKind` and which halts the read for the **whole tree** — is why
deferring is not an option even though no queued leaf triggers it.

Step 2's precondition is met: the new build, run read-only, agrees with 19.5.0
on `pick` for all five other live groves on this machine
(`APIAnyware.add-ocaml-target`, `grove.code-walkthrough-for-ordinal-fs-tree`,
`grove.gh-issue-12`, `grove` and `Writegood`). Nobody is stranded.
