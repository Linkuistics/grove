# to-an-argv-k112

## Goal

Draft chapter 5 of the `keyed-launch` book — *From a template to an argv*,
`05-to-an-argv.md`, slice `whole-word-or-nothing` — owning `src/templates.rs`
lines 146–275 (130) and 661–670 (10), and `src/argv.rs` whole (48): 188 lines.

## Context

- Draft stage, child 5 of 10 of `keyed-launch-k107`. Responsibilities are the
  structure brief's *5 · From a template to an argv*.
- `source`, and why `None` for a key the primary does not declare whatever the
  overlay says; `require` as the obligation a consumer discharges *before* it
  commits to a key, stated once so the refusal's wording has one owner;
  **`expand`'s single remaining obligation and why it is stated over the
  *vocabulary* rather than over this template's own words** — so a consumer cannot
  have a call that works for one key and fails for its neighbour purely because
  the two templates mention different optional slots; `match_values` as that
  check; `unresolved` and its two wordings, **which is where the
  untracked-delta rule is actually kept**; `keys` as the conformance kit's one
  window and nothing more; and `Argv` — no constructor, no shell, `words()`.
- **The seam is proved here.** `Argv::new` is `pub(crate)` and `Templates::expand`
  is its only caller, so *nothing reaches a spawn that a template did not author*
  is a fact about the types. Chapter 1 stated it; this chapter shows the two lines
  that make it true. Verify the `pub(crate)` and the caller count against the
  source rather than against chapter 1's claim.
- Prose obligation 3 is **supply the argument** — the last of the three chapters
  it points at. From chapter 6 onward the instruction reverses.
- Required example anchor: `four-words` — `expand("impl", …)` through to
  `["claude", "--model", "opus", "<the prompt>"]`, the prompt one argument however
  many spaces it holds.

## Done when

- `book-check --repo . --book docs/walkthroughs/keyed-launch --through
  whole-word-or-nothing --check all` is valid: 958 resolved lines, 1,115 deferred,
  `final=false`.
- `scripts/check.sh` is red on `book-check` alone, and this file says so.

## Notes

This chapter closes `src/templates.rs`: after it, all eight blocks of that root
are resolved. It owns the 146–275 that sits between chapter 3's two blocks, and
the ownership ledger is where a reader sees that.

## Decisions (running log)

**1 · The worked example loads the primary alone, setting chapter 3's overlay
aside and saying so.** The brief's required anchor is `four-words` with the
observable end `["claude", "--model", "opus", "<the prompt>"]` — the *primary*'s
`opus` template. Chapter 3 left the carried example in a state where `impl`
resolves from the overlay as `claude --model opus-fast ${prompt}`, so continuing
from that state would have contradicted the brief's stated end. The section takes
one sentence to say the overlay is set aside and why it changes nothing about
expansion — which file a template came from is `source`'s question, and this
chapter answers it two sections later. Rejected: carrying the overlay state and
producing `opus-fast`, which would have made the brief's `#four-words` end wrong
in the one chapter that owns it; and silently reverting, which leaves a reader who
remembers chapter 3 unable to tell whether the book lost track of its own example.

**2 · The mandate string is fixed here, and it is fixed to hold both hazards.**
Chapter 1 wrote the `prompt` value as `<the mandate>` because nothing before this
point depended on what was inside it. Chapter 5's whole claim is about what is
inside it, so the value becomes `Fix the $(date) helper in scripts/check.sh` —
five spaces and a `$(…)`, which is exactly the pair
`a_slot_value_is_one_argument_whatever_it_contains` uses. That makes the page's
worked example and the pinning test the same shape rather than two shapes a
reader has to reconcile. The remaining three slot values (`session_name`,
`worktree`, `repo`) are fixed here too, because expansion's obligation is over the
vocabulary and the example cannot be made without all four.

**3 · Twelve literal fragments over three blocks, and `templates-keys` left
unrefined.** `resolution-and-expansion` is six, at function boundaries, because
here the function *is* the unit: each of the six answers one row of the chapter's
question table and the page walks that table in source order.  `argv` is five,
cut so that `pub(crate) fn new` sits alone in a five-line fragment — the seam is
the chapter's proof obligation and burying the constructor in a fragment with the
two accessors would have made the load-bearing token one line of ten.
`templates-keys` is ten lines and one function and stays a single literal
top-level block: refining it would produce a composite whose only child is
itself. Rejected: splitting `expand` into a signature fragment and a body
fragment, which would have separated the doc comment stating the obligation from
the `match_values` call that discharges it.

**4 · No figure drawn beyond the four tables the prose contract requires, and the
`Argv::new` call count stated as a `text` figure.** The four relations the reader
would otherwise reassemble from sentences are drawn: the position-by-position
expansion mapping, the two-column comparison of the obligation over the
vocabulary against the rejected obligation over the template, the three
`match_values` refusals against *when* each is found, and the two `unresolved`
wordings against what each tells the reader. The call-count figure is three lines
of `text` rather than a sentence because it is the chapter's proof and a reader
should be able to see the count without parsing a clause. Everything else is left
to `art`.

**5 · The `#[must_use]` observation replaced a false claim about workspace
lints.** The first draft attributed the attribute's force to the lints
`Cargo.toml` inherits, which is wrong — `[lints] workspace = true` does not put
`#[must_use]` on anything. The claim is now the checkable one: twelve functions in
the crate carry it, `source` and `keys` among them, and the two that return a
`Result` do not, because the compiler already refuses a discarded `Result`.

**6 · The leaf's one in-session reviewer was spent on technical truth, and it
found eleven things.** The claim put to it was that every assertion the chapter
makes about `crates/keyed-launch` holds against the source — line numbers, caller
counts, test bodies, exact refusal strings, reachability and check ordering — and
the reviewer was given the page and that contract with the conclusion stripped.
It confirmed all fourteen fragments byte-exact, the unreachability argument for
`unresolved`'s default branch, the check ordering in `expand`, the three
name-comparison sites, and every quoted refusal string including the
`slot`/`slots` pluralisation. **All eleven findings were valid and actionable;
none was noise, a stated trade-off, or a contract stated unclearly**, and each was
re-verified against the source before it was applied. Six were mine to fix in
place: the `.words()` caller count (seven in `grove-loop`'s tests, not six, and
the enumeration had silently dropped this crate's own helper); a false universal
that all twelve `#[must_use]` functions are queries, when `take_interrupt` clears
a latch as it reads; a sentence saying the program/args split happens at load,
when `expand` performs it at 194–198 and load supplies only the guarantee; a
bogus reason for `declared_slots` being a method, since Rust privacy is
module-scoped and the file's own free functions read `SlotSpec` directly; a
mis-attribution of the `keyed_launch::run` call to `session_config.rs` when it is
`loop_driver.rs:422`; and three line citations that named block boundaries while
reading as function boundaries. Two were phrasing: a collision with chapter 3's
*takes its second step here*, and an unprovable *no later chapter returns to it*
over pages that do not exist. No second reviewer was materialised and no
`review-*` leaf was cut: every fix is a local factual correction the source
settles on re-reading, which the execute procedure's escalation rule explicitly
does not count as tree-sized work.

**7 · One finding was chapter 1's, and it is fixed there rather than deferred.**
`01-orientation.md`'s five-call trace wrote step 2 as `expand("impl", [Slot {
name: "prompt", … }])` — a single value, against the four-slot vocabulary the
same page tabulates twenty lines above. Under `match_values` that call is a
refusal, so chapter 1's figure asserts a launch that cannot happen, and chapter 5
is the page that proves it: the rule condemning it is the rule this chapter owns.
Leaving it would have shipped a book whose first chapter contradicts its fifth,
and no later stage's charter reaches it — `copy-edit`, `art` and `proof` do not
own technical truth. The fix is confined to that one figure: the call now offers
one value per declared name, and a two-line note under it points at this chapter
for the reason. No fragment, ownership range or ledger row is touched, and the
scoped validator is green over the whole prefix afterwards. Rejected: a
`## Handed forward` entry, which would have named no owning stage; and correcting
it only in chapter 5's prose, which leaves the false figure standing on the page
a reader meets first.

## Result

`book-check --repo . --book docs/walkthroughs/keyed-launch --through
whole-word-or-nothing --check all` is **valid: 9 files, 958 resolved lines, 1115
deferred lines, `final=false`** — the figures this leaf's *Done when* names. Three
blocks moved from `deferred` to `resolved`: `resolution-and-expansion` (130),
`templates-keys` (10) and `argv` (48). `src/templates.rs` is now closed — all
eight of its blocks are resolved — and `src/argv.rs` is the fourth root the book
reconstructs whole.

`bash scripts/check.sh` exits 1: **FAILED — 1 of 8, and the one is `book-check`**,
as every child of this node but the last is expected to leave it. Under `--final`
the keyed-launch book reports the five unwritten pages (`M101`), the two
navigation and contents entries that name them (`M103` — one of which is this
page's own missing `Next` to `06-the-channel.md`), the fourteen source blocks
chapters 6–9 own (`F003`), and the seven ownership and early-use rows still
`pending` for those chapters (`F009`). Every one of those findings names a later
chapter; none names a defect in `whole-word-or-nothing`. The other seven checks —
`cargo fmt`, `shellcheck`, `cargo clippy`, plugin install, conformance, the
conformance suite and the test suite, including
`every_repository_markdown_reference_resolves` and the corpus-inventory tests —
pass. The measurement was taken after every edit was finished, including the
review fixes and the chapter 1 correction, and the other four books
(`ordinal-fs-tree`, `overview`, `grove-llm`, `jj-workspace`) still validate
`final=true`.

The early-use row this chapter owns — `Argv`, `Slot`, first used at
`01-orientation.md#the-cast` — is now `explained`, and it does not appear in the
`--final` `F009` list, which is the check that it was accepted rather than merely
edited.
