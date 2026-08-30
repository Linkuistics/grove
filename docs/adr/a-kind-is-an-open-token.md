# A kind is an open token

A leaf's **session kind** is any well-formed token — non-empty, lowercase ASCII
letters, digits and single hyphens, no `--`, not one of the grammar's own
reserved markers. Grove validates that shape and nothing else. It holds no list
of kinds, no enum of them, and no count of them.

The set of kinds that *exist* is the set of `grove-<kind>` skills the installed
methodology ships. Grove never reads that set: it renders a kind's token into a
skill name and into a configuration key, and interprets it in neither.

**Grove spells exactly two kind tokens**, and only where it writes the leaf
itself with no session to delegate to:

- `requirements`, for the first leaf `root-init` lays down before any session
  exists;
- `finish`, for the teardown sentinel the loop writes between the last ordinary
  session and the finish session.

Both are grove recognising a leaf it authored. Three sites ask about the second —
`finish` sorting last in selection, the grow verbs refusing to create one, and
teardown — and all three go through one predicate rather than carrying a token.

A kind for which no skill is installed **parses, and launches**. The failure is
reported by the session that could not load the skill, where a human is present
to read it. A kind for which no launch template resolves is refused before the
tree is mutated, naming the kind and the file that must declare it.

## The trade-off

The kind was a compiled enum of nineteen variants: a parse arm, a label arm and a
hand-maintained `ALL` roster per kind, plus the tests that held the three
together. Every kind the methodology had was therefore also a fact about the
binary, and adding one — or renaming one — meant editing Rust, rebuilding, and
publishing a release before a single session could be launched under it.

That is backwards. The methodology is what defines the kinds; grove is a
launcher. Making the machinery hold the set gave grove an opinion about what a
kind *means* that it has no standing to have, and it made the *cheapest* possible
change to the methodology — authoring one more skill — the most expensive kind of
change there is.

**What it costs is the compile-time error.** A typo'd kind used to fail at
`cargo build`; now it fails at `leaf-add`, when the launch configuration declines
to resolve a template for it. That is a worse moment in one respect — the failure
is at authoring time rather than at build time — and a better one in another: a
human is present at `leaf-add`, and the refusal can name the file that must
declare the kind, which a compiler error could not.

Two mitigations make that cost bearable, and both are load-bearing rather than
consolation:

- **The filename grammar had to move first.** With a closed set, the boundary
  between a hyphenated kind and a hyphenated slug was resolved by matching the
  middle of a name against the set — so `02-design-decomposition-k2.md` had more
  than one reading, and only the set picked one. Opening the set would have made
  that name unreadable. The `--` separator landed first, in its own leaf, and it
  is what makes an open token safe: the separator *is* the boundary, so a name
  has exactly one reading with no set consulted.
- **`--kind` became required** on `leaf-add` and `leaf-insert`. It used to
  default to `impl` — a kind literal under a friendlier name, and the one that
  produced a *wrong* leaf rather than an error, since a forgotten flag yielded a
  well-formed `impl` leaf and no complaint.

## Considered options

- **A kind manifest** — a file the methodology ships listing its kinds, which
  grove reads and validates against. Rejected: it reintroduces the coupling in a
  slower form. Grove would gain a document format, a parser, a validator and a
  set of refusals for a manifest that disagrees with the skills beside it, and
  the set would then exist in two places — the manifest and the skill
  directory — which is the drift the closed enum at least did not have. It also
  answers a question nobody asked: a session does not need grove's permission to
  name its kind, it needs a skill to load.
- **Enumeration by reading the installed skill set** — grove lists the
  `grove-*` skill directories and accepts exactly those tokens. Rejected on two
  counts. It makes the *tree grammar* depend on what happens to be installed on
  the machine, so a leaf that parses on one machine is malformed on another and
  a whole subtree disappears from `pick` because a skill was not installed —
  precisely the silent loss the `Malformed`/`Foreign` split exists to prevent.
  And grove cannot see the installed set anyway: a plugin is installed per
  machine by a marketplace the binary does not read, so any such enumeration
  would be a guess at a path.
- **Keep the enum and add kinds by release.** Rejected: it is the status quo
  whose price this record states. It also does not survive the plugin — the
  methodology already ships and installs separately from the binary, so the
  binary's roster and the installed skill set can already disagree, and the enum
  bought no protection against the one disagreement that matters.
- **Validate nothing at all** — take the kind as an opaque string with no shape
  rule. Rejected: the token is written into a filename beside a slug and read
  back out of it, so a token carrying `--`, a leading or trailing dash, or an
  uppercase letter makes a name that does not round-trip. Canonicity is a
  property of the grammar, not of the kind set, and it is the one thing the
  grammar must still say.

## Why this is hard to reverse

Closing the set again is not a matter of restoring an enum. The filename grammar
now depends on the separator rather than on set membership; the store, the
prompt, the configuration key and the CLI all carry a token rather than a
variant; and the methodology's kinds have become authorable content, so the
skills that exist on any live machine are no longer a set the binary was built
knowing. A reinstated roster would have to be a *second* source beside the
installed skills, which is the manifest alternative above under another name.
