# session-config-review-k20

**Kind:** review-impl
**Reviews:** session-config-k19
**Producer launch:** {"producer":"session-config-k19","session":"session-config-k19","generation":"k19","harness":"codex","model":"sol-xhigh"}

## Goal

Adversarially review `session-config-k19` and record concrete findings for its integration step.

## Context

- Review the committed `session-config-k19` change against
  `docs/specs/config-driven-sessions.md` and
  `docs/adr/complete-session-configuration.md`.
- Concentrate on grammar ambiguity, exact nineteen-kind completeness,
  diagnostic aggregation and spans, argv injection, accidental shell
  evaluation, Rust 1.74 compatibility, and whether the module's interface is
  deep enough to keep those rules local.

## Done when

- Findings are recorded in this task file, each with severity, evidence,
  affected contract, and a concrete correction or explicit no-finding result.
- The review independently exercises representative valid and invalid configs
  and distinguishes implementation defects from choices already settled by the
  spec.
- No production or test code is changed.

## Findings

Reviewed artifact: commit `ornvunux` (`session-config-k19: add complete session
command configuration`) — `src/session_config.rs`, `tests/session_config.rs`,
`src/lib.rs:20`, `Cargo.toml`, `Cargo.lock` — against
`docs/specs/config-driven-sessions.md` ("Configuration file" `:63-219`, "Module
interfaces" `:750-794`) and `docs/adr/complete-session-configuration.md`. Cite as
`session-config-review-k20 E<n>`. No production or test code changed: every claim
below was reproduced through the module's own `load`/`expand` interface from an
out-of-tree probe crate that path-depends on `grove`, so the repository is
byte-identical apart from this file.

Verdict: the module is the right shape and its central invariant holds — I could
not construct a document that loads successfully while leaving a required kind
unexpandable, and I could not make a substitution value change an argument
boundary. Diagnostics are deterministic (every `HashMap` here is lookup-only;
ordering comes from `REQUIRED_KINDS` and document order). The two defects are at
the edges the module delegates to its dependencies and never re-checks:
`shell-words` strips comments, which silently deletes configured arguments (E1),
and the `KdlValue::String`-only match rejects raw strings, which are strings
(E2). Both are reachable by ordinary templates, and E1 is the only defect found
that *launches something other than what was configured*. E3–E5 are
diagnostic-contract gaps, E6–E7 are seam findings for the cutover, and E8 is
pre-existing and externalized.

### Blocking

**E1 — An unquoted `#` starts a comment, silently truncating the template.**
`src/session_config.rs:266` delegates word splitting to `shell_words::split`,
whose parser enters a `Comment` state on `#` seen in `Delimiter` state
(`shell-words-1.1.1/src/lib.rs:143`) and discards the remainder of the string.
`split` is that crate's only parsing entry point and has no comment-disabling
option, and nothing in `validate_template` re-checks that the split consumed the
template.

| template configured for `requirements` | result |
|---|---|
| `runner ${prompt} --color #ff0000 --verbose` | **loads**; argv `["runner", "MANDATE", "--color"]` |
| `runner ${prompt} #ff0000 --trailing` | **loads**; argv `["runner", "MANDATE"]` |
| `runner #ff0000 ${prompt}` | rejected — "must contain `${prompt}` exactly once" |
| `runner ${prompt} --color '#ff0000' --verbose` | loads; full argv — quoting saves it |
| `runner tag#1 ${prompt}` | loads; `tag#1` literal — mid-word `#` is fine |

Affected contract: `:182` "Configuration is all-or-nothing"; `:197` "No
diagnostic silently fills a target"; `:167-169` on argument boundaries; and the
ADR's rejection of shell execution because "shell evaluation turns quoting,
environment expansion, pipelines, and redirection into a second configuration
language". Comment stripping is exactly that second language arriving
unannounced. `:70` grants comments to the KDL *document*, not to the template
string, and `:163`'s list of un-interpreted shell features does not mention
comments — so this is unspecified behavior, not a settled choice.

*Why it matters*: the failure is position-dependent. A `#` word before
`${prompt}` fails loudly, but only because the required-once check happens to
catch it — an accidental safety net, not a rule. A `#` word after `${prompt}`
loads and launches a different, plausibly-shaped command line: in row one, a
`--color` flag stripped of its value and a lost `--verbose`. `#` is ordinary in
colours, issue references, and fragments, and a KDL author has no reason to
expect shell comment rules inside a quoted KDL string.

*Action*: reject it. In `validate_template`, before compiling, scan the raw
template for a `#` that `shell-words` would treat as comment-introducing (a `#`
reached in delimiter state) and push a diagnostic naming the kind — e.g.
`` `#` starts a comment in a command template; quote it (`'#…'`) to pass it
literally ``. Add rows one and two above as tests. If the integration step
instead decides to keep comment support, that is a grammar decision that belongs
in `config-driven-sessions.md:133-171` with a pinning test; what it must not do
is stay unstated, because the failure is silent.

**E2 — KDL raw strings are rejected as "not a string".**
`src/session_config.rs:239-248` matches only `KdlValue::String(_)`. In `kdl`
4.7.1 a raw string is a distinct variant, `KdlValue::RawString(String)`
(`kdl-4.7.1/src/value.rs:5-10`), and the crate's own accessor treats both as
strings — `as_string()` returns `Some` for `String | RawString` (`:111-117`).

Reproduced: `requirements r#"runner --x "quoted" ${prompt}"#` →
`invalid Grove configuration at …: …:1:1: session kind's sole argument must be a
string`.

Affected contract: `:187` "every node's sole argument is a string".

*Why it matters*: the rejection is loud but its explanation is false — the user
did write a string — and it lands on precisely the templates that need a raw
string most. A template carrying JSON or nested quotes (`--settings '{"a":1}'`,
which `${herdr_settings}` makes an obvious hand-written neighbour) is either
written as a raw string or double-escaped through KDL. Reaching for the natural
KDL idiom currently earns a diagnostic that denies what the user plainly did.

*Action*: replace the match with `positional[0].value().as_string()`, keeping the
existing `None` arm's diagnostic for genuine non-strings. Add a passing
raw-string case, and pin that the real non-strings still fail — bare `true`,
`null`, and numbers all correctly fail today and must keep doing so.

### Diagnostic contract gaps

**E3 — Template diagnostics never name their kind.**
`:196` requires "every invalid template with its kind and span". `at_node`
(`:366-371`) carries only a location, and none of the template messages
(`:277-318`, `:334-343`) interpolate the kind — neither `validate_template` nor
`parse_template_word` is given it. Reproduced: an invalid `review-impl` template
inside a nineteen-node document renders as
``…/config.kdl:14:1: command template must contain `${prompt}` exactly once`` —
the reader must count to line 14 to learn which kind failed, in the one
diagnostic class the spec singles out for naming its kind. Unknown-kind
(`:191`) and duplicate (`:182`) diagnostics do name theirs, so the omission is
inconsistent within one rendered list.
*Action*: thread the kind into `validate_template` / `parse_template_word` and
prefix template messages with `` `<kind>`: ``, or stamp it in `validate_node`
after the fact. Extend
`schema_and_template_failures_are_aggregated_with_source_locations` to assert the
kind appears beside the template message.

**E4 — An empty word zero reports one defect twice.**
`:277-282` pushes "word zero must be a literal non-empty executable" when
`words[0].is_empty()`; the per-word check at `:288-293` then pushes "word zero
must be a literal executable" for the same word, because an empty `Literal` also
fails its `!value.is_empty()` guard. Reproduced: `'' runner ${prompt}` yields
both lines at the same location. The neighbouring cases are correct — an empty
template yields word-zero plus missing-prompt, two genuinely different defects,
and `${herdr_settings} runner ${prompt}` yields one line.
*Action*: drop the `words[0].is_empty()` arm and keep the loop check, which
already covers both empty and substitution word zero; or keep the pre-check and
narrow the loop guard to `index == 0 && parsed.substitution_name().is_some()`.
Either way, one defect, one line.

**E5 — Concatenated substitutions are reported as an unknown *name*.**
`is_whole_substitution` (`:362-364`) is `starts_with("${") && ends_with('}')`,
which is true of `${a}${prompt}`, so `parse_template_word:334-337` classifies a
concatenation as an unknown substitution name. Reproduced:
`runner ${a}${prompt}` → ``unknown substitution `${a}${prompt}` ``, and
`runner ${prompt}${session_name}` → ``unknown substitution
`${prompt}${session_name}` `` — a message naming two substitutions that both
exist. The accurate message is the one two arms below (`:338-343`),
"substitutions must occupy a complete shell word".
*Action*: require the trailing `}` to be the only `}` in the word, so a
concatenation falls through to the embedded-substitution arm. Both templates
still fail; only the explanation changes.

### Seam findings for the cutover

**E6 — The nineteen kinds became a second source of truth, and it is the exact
one the ADR relies on.**
`REQUIRED_KINDS` (`:12-32`) restates the closed kind set that `leaf::Kind`
already owns (`src/leaf.rs:105` `Kind::ALL`, `:279-306` `Kind::label`, whose doc
comment calls itself "the single source of truth for the label direction"). The
two deliberately differ today — this is the expand half of an expand/contract,
and the target set adds `finish` and splits `research` into
`research-a`/`research-b` (`config-driven-sessions.md:103-125`, `:609-610`).
What is missing is any mechanism tying them together. The ADR's stated reason
the design binds is that "the configuration deliberately repeats all nineteen
targets so every session is complete in isolation and **adding a kind fails
visibly in every old config**". Adding a kind to `leaf::Kind` fails visibly in
no config at all unless a human also remembers `REQUIRED_KINDS`; a tree that
then emits the new kind reaches `expand`'s ``"no target for session kind"``
error (`:92-95`) — the runtime failure completeness validation exists to make
impossible. The module's completeness proof only ever guaranteed *its own*
nineteen.
*Action*: in `session-kind-migration-k27` / `lifecycle-cutover-k39`, derive
`REQUIRED_KINDS` from the extended `leaf::Kind` (a `Kind::ALL.map(Kind::label)`
equivalent) so the set has one owner, and change `expand` to take `Kind` rather
than `&str` — which turns the residual runtime error into a type-level
impossibility and makes the ADR's fail-visibly claim true by construction. Until
then, one comment in `session_config.rs` naming `leaf::Kind` as the list this
must converge on, so the duplication does not read as intentional independence.

**E7 — `expand` returns one flat argv, leaving "word zero is the program" to
every caller.**
The module validates that word zero is a literal non-empty executable
(`:277-293`) — a rule that exists *because* the program is spawned directly —
and then returns `Vec<OsString>` (`:92`), discarding the distinction. This is
within contract: k19's Done-when and `:200-206` both put executable resolution
and spawn on the caller. But it leaves the one rule the module cannot enforce —
*do not hand this to a shell* — encoded only in each call site's
`Command::new(argv[0]).args(&argv[1..])`, which is the ADR's rejected option
reachable by a single caller slip. With exactly one production caller pending
(`lifecycle-cutover-k39`), tightening is cheapest now.
*Action*: consider returning `SessionCommand { program: OsString, args:
Vec<OsString> }` (or a tuple). It cannot be misused as a shell string, and it
lets the launch-error contract at `:200-206` — "naming the selected kind and
executable" — read the executable off the type instead of re-deriving it.
Non-blocking; a judgement call the integration step may decline.

### Out of scope — externalized, do not absorb

**E8 — The declared MSRV is already unsatisfiable, independently of this change.**
`Cargo.toml:8` declares `rust-version = "1.74"`, and both k19's Done-when and
this leaf's Context treat 1.74 as a live constraint. It is not one: `Cargo.lock`
pins `clap` 4.6.1 and `clap_lex` 1.1.0, both `edition = "2024"` with
`rust-version = "1.85"`. `rustup run 1.74 cargo check --locked --all-targets`
fails before compiling any Grove code — "this version of Cargo is older than the
`2024` edition". The pin predates this change (`jj file show -r ornvunux-
Cargo.lock` carries the same `clap_lex 1.1.0`), and k19's own additions are
clean: `kdl` 4.7.1 (edition 2021), `miette` 5.10.0, `nom` 7.1.3,
`minimal-lexical`, `unicode-width`, and `shell-words` 1.1.1, declared MSRVs all
≤ 1.61.
*Finding against k19*: **none**. It neither regresses nor could preserve a
constraint that was already false.
*Action*: not `session-config-integrate-k21`'s work. Raised as its own leaf
(`msrv-claim-k74`) so the repo's MSRV claim is either corrected to its true value
or restored by pinning `clap` back. Recorded here only because "Preserve Rust
1.74 compatibility" would otherwise read as verified.

### Checked and sound

Reproduced through `SessionConfig::load` / `expand` and found correct, so the
integration step need not re-litigate them:

- **The completeness invariant holds.** `template: None` is produced only on
  paths that also push a diagnostic (`:231-251`, `:266-275`), so `load` cannot
  return `Ok` with a required kind absent from `templates`. `expand`'s `"no
  target"` arm is unreachable for a loaded config — E6 is the one way that
  changes.
- **Exactly nineteen kinds, canonical order.** `REQUIRED_KINDS` matches
  `config-driven-sessions.md:103-125` name for name and is typed `[&str; 19]`,
  so a dropped entry is a compile error. `tests/session_config.rs:8-28` restates
  the list independently, so a production typo fails the missing-file test
  rather than being mirrored by the test.
- **No shell evaluation, no boundary bleed.** `$(touch nope)`, `*`, `>`, `;`,
  and spaces inside `${worktree}` / `${repo}` / `${session_name}` / `${prompt}`
  each survive as exactly one argument. Non-UTF-8 paths survive too:
  `as_os_str().to_owned()` (`:103-104`) avoids a lossy round-trip.
- **`${prompt}` positioning and arity.** Non-final `${prompt}` works; word-zero
  `${prompt}` is rejected; zero and two occurrences are rejected; each optional
  substitution is capped at one.
- **`${herdr_settings}` splice.** Zero arguments when absent, two when present,
  at any position including last, without consuming or reinterpreting a
  neighbouring word; literal words after the splice survive.
- **Node shape.** Properties, child blocks, node and entry type annotations,
  zero or two positional arguments, and non-string values (bare `true`, `null`,
  numbers) are each rejected; a `/-`-commented node correctly counts as missing.
- **Aggregation and spans.** Missing, duplicate, unknown, malformed, and
  template diagnostics collect into one error; duplicates list every declaration
  location; missing kinds render in canonical order. Multi-byte characters
  before and on the failing line do not shift the reported column, and
  `source_location` (`:389-395`) clamps its offset.
- **Repository health on the committed tree.** `cargo fmt --check` clean,
  `cargo clippy --locked --all-targets` clean, `cargo test --locked` 626 tests
  passing. Existing CLI behavior is untouched: `src/lib.rs` gains only the module
  declaration.

## Notes

The reviewer produces findings only; `session-config-integrate-k21` owns fixes.

Review target: claude / opus-5, against a producer launched on codex / sol-xhigh
per this leaf's `**Producer launch:**` line — materially diverse in both harness
and model. Recorded as a fact, not a gate.

No in-session reviewer was materialised: a `review-*` leaf spawns none
(`grove-owns-escalated-review.md`).

E1 is the finding most likely to be waved off as pedantry about a shell corner
case. It should not be: it is the only way found to make Grove launch a command
the configuration does not describe, it needs no unusual input (`#` in a colour
or an issue reference reaches it), it produces no diagnostic, and its loud
sibling is loud only by accident. If the integration step accepts comment
stripping rather than rejecting it, that acceptance is a grammar decision and
belongs in the spec beside the shell-execution rejection in
`complete-session-configuration.md`, because it is the same trade-off — how much
shell language leaks into an argv template — seen from the other side.
