# branch-review-k14

**Kind:** review

## Goal
Fresh-context adversarial read of the whole codex-pi-harness branch diff
against the spec and plan. Findings, not fixes.

## Context
- Artifact: `git diff main...HEAD` in this worktree.
- Contract: the spec (docs/superpowers/specs/2026-07-18-codex-pi-harness-
  switch-design.md) and the plan's Global Constraints section.
- Axes worth separate attention: the symlink-never-followed provisioning
  guard; env precedence and cross-harness leakage; the stamp explicitness
  rule; whether tests assert what they claim (fake-bin argv honesty).
- linkuistics:doubt-driven-development — pass artifact + contract, never the
  claim; classify findings, do not rubber-stamp.

## Done when
Findings recorded in this leaf file and committed; anything needing a fix
becomes a new leaf via `grove-llm leaf-insert 15 <slug>` (sequenced before
release), never silently patched in-session.

---

## Method

One doubt cycle, diverse-lens: four fresh-context subagents, each given
ARTIFACT (a per-axis diff slice) + CONTRACT (Global Constraints + the spec's
implementation sections) and an adversarial prompt — never the claim, never
this session's reasoning. Axes: provisioning/filesystem safety; env precedence
and routing; stamp + PID rename; test honesty. The docs/wording axis and the
build/test baseline were verified in-session.

Every finding below was re-read against the artifact text before classifying
(RECONCILE). Findings the reviewers raised that did not survive that check are
recorded under *Rejected* so the next session does not re-litigate them.

Baseline: `cargo fmt --check` clean, `cargo clippy --all-targets` clean.
`cargo test` **not** clean — see B1.

---

## Blocking — must land before release (k15)

**B1. The branch's own test suite fails on the machine the trial runs on.**
5 of 9 `tests/loop_driver.rs` tests fail, reproducibly (7 of 8 runs).
Root cause is one real failure; the other four are cascade.
- `loop_selects_model_by_kind` (tests/loop_driver.rs:304) expects
  `--model haiku` and gets `--model kimi-coding/k3`. The test sets only the
  *base* vars (`:260-262`) and never scrubs the new dimension, so the ambient
  `GROVE_REVIEW_HARNESS=pi` + `GROVE_PI_REVIEW_MODEL=kimi-coding/k3` that
  leaf 12 wrote into `~/.zshenv` reroute iteration 3. Confirmed:
  `env -u GROVE_REVIEW_HARNESS cargo test --test loop_driver` → 9 passed.
- The misroute is *invisible* rather than loud because the global
  `GROVE_HARNESS_BIN` fallback (B5) makes "pi" launch through the claude fake.
- `ENV_LOCK.lock().unwrap()` (ten sites in loop_driver.rs, plus
  tests/provision.rs:126, tests/complete.rs:149) propagates `PoisonError`
  after the first panic, so four unrelated tests report a message that says
  nothing about the cause.
- Why leaf 09 saw green: it ran *before* leaf 12 changed the environment. A
  green suite is only green against the env it ran in. The scrub is now 25
  names — that is the argument for a `clear_grove_env()` helper.

**B2. The base-var fallback defeats harness scoping — a codex profile can
reach pi.** `src/loop_driver.rs:225` falls back to `GROVE_<KIND>_MODEL`, which
is harness-agnostic, and `model_for` is called with the *post-override*
harness (`:288-293`). Trigger: codex-stamped grove, `GROVE_REVIEW_MODEL=sol-high`,
`GROVE_REVIEW_HARNESS=pi`, `GROVE_PI_REVIEW_MODEL` unset → `pi --model sol-high`.
The doc comment at `:220-221` asserts the opposite of what the code does
("a codex profile name is garbage to pi and vice versa"). The only thing
preventing it is a user-config convention (zshenv deleted the base vars),
enforced nowhere in the binary — while `src/cli.rs:9-13` still advertises the
five base vars as first-class. Compounding: `env_model` treats `""` as unset
(`:298-300`), so `GROVE_PI_REVIEW_MODEL=""` falls *through* to the base var;
there is no way to say "on pi, for review, pass no flag".

**B3. `--no-launch` permanently rebinds the grove.** `maybe_stamp`
(src/launch.rs:25) runs before the `no_launch` early return (`:47`).
`--no-launch` is documented as "Report readiness but don't exec the harness" —
a dry run that now mutates persistent cross-session state. Verified live:
`grove do --harness pi --no-launch` in a claude-only repo writes
`.grove-stamps/<name>` = `pi`.

**B4. A failed bind is permanent and has no recovery verb.** The stamp
(src/launch.rs:25) is written before `provision_all` (`:30`), which *bails* on
a foreign skills dir. So `grove do --harness pi` with a foreign
`~/.pi/agent/skills/grove` stamps `pi`, then errors — and every later plain
`grove do` re-resolves `pi` and re-fails identically. Nothing prints the stamp
path and no verb removes a stamp. Same shape: nothing checks `exec_bin` is on
PATH before stamping.

**B5. Global `GROVE_HARNESS_BIN` leaks across a reroute.**
`src/loop_driver.rs:199-201` falls back from `GROVE_HARNESS_BIN_<NAME>` to the
unscoped `GROVE_HARNESS_BIN` for *every* harness. With it set and
`GROVE_REVIEW_HARNESS=pi`, the review leaf execs the claude wrapper with pi's
flag template. Once one loop can launch two harnesses a single global bin
override is incoherent. Also the lone env seam in the file that treats `""` as
set rather than unset (contrast `env_model:298`, `harness_override:245`,
`any_harness_override_env:263`) — `GROVE_HARNESS_BIN=""` yields a broken exec.

**B6. A degraded kind peek silently cancels the reroute.**
`src/loop_driver.rs:284-287` returns the *stamped* harness whenever
`resolve_kind` degrades (missing `grove-llm`, non-zero exit, unparseable
token). In a codex grove with `GROVE_REVIEW_HARNESS=pi` a review leaf then
runs on codex — sol reviewing sol's own work, which is exactly what
"K3 reviews everywhere" exists to prevent. The three diagnostics that fire
(`:337-338`, `:346`, `:351-354`) mention only the *model*, never the harness.
`harness_override`'s own doc (`:241`) argues a silent fallback "would misroute
every review"; this path does that and is not loud.

**B7. `load_prompt` reads the stamped harness's copy, not the launching
one.** `src/loop_driver.rs:109` loads the prompt before `:114` resolves
`launch_harness`. Contract: "`load_prompt` reads from the launching harness's
copy." Content-identical today, so it masks the real failure: `provision_all`
skips a harness whose home root is absent (src/provision.rs:47-49), so with
`~/.pi` absent, pi is handed a prompt reading "use the grove skill" pointing at
a skill that was never created in pi's dir — silently, no diagnostic.

**B8. Partial extraction wedges `grove do` — a regression.**
`sync_to_stamp` writes files (`src/provision.rs:133`) then the stamp (`:134`),
non-atomically, after `remove_dir_all` + `create_dir_all`. Interrupt mid-extract
(Ctrl-C, SIGTERM, ENOSPC) and `dest` is non-empty with no stamp — byte-identical
to the "foreign directory" signature the new guard (`:99-102`) refuses. The next
`grove do` hard-bails about a directory grove created seconds earlier, advises
"move it aside" when there is nothing to preserve, and no grove session can
launch until the user manually removes it. The pre-change path had no guard and
self-healed. `tests/provision.rs:106` constructs exactly this state and asserts
the bail. Fix direction: extract to a sibling temp dir and `rename` into place.

**B9. A foreign dir under *any* harness aborts the whole sweep, including the
primary.** `provision_all` propagates with `?` (`src/provision.rs:51`) and
`do_grove` propagates at `src/launch.rs:30`; `HARNESSES` order is claude,
codex, pi. `grove do --harness pi` with a foreign `~/.codex/skills/grove`:
claude is rewritten (side effect committed), codex bails, **pi — the harness
being launched — is never provisioned**, and `grove do` refuses to run.
An unrelated harness's directory state bricks the intended one, and nothing
reports what the half-applied sweep did. The contract requires bailing on a
foreign dir; it does not say a foreign *codex* dir blocks a *pi* launch.

**B10. Unvalidated PID reaches `kill` — `GROVE_HARNESS_PID=-1` becomes
`kill -TERM -1`.** `src/complete.rs:98-101` resolves via `env_parse::<i32>`,
which accepts negatives (`:114-116`), and `:163-165` interpolates it raw into
`kill -TERM {pid}` / `kill -KILL {pid}`. `-1` signals every process the user
can signal; `-<n>` signals process group *n*. The killer is `setsid`-detached
so it survives to deliver `-KILL`. The diff added a second env source for this
value without adding a `pid > 0` guard where both converge. One-line fix:
`.filter(|p| *p > 0)`.

---

## Docs — wrong in ways that will misdirect the user

**D1. The codex profile location is documented wrongly in two places.**
`CHANGELOG.md` ("profiles you define in `~/.codex/config.toml`") and
`src/cli.rs:26` both say `config.toml`. Verified on disk: task 10 created
`~/.codex/sol-high.config.toml` and `~/.codex/sol-xhigh.config.toml`, and
`~/.codex/config.toml` contains no profile table — matching the spec, which
records that this codex build layers `$CODEX_HOME/<name>.config.toml`. A user
following either doc puts the profile where codex will not find it.

**D2. Four current-state docs still state the pre-change stamp policy**, now
false for the explicit path: `README.md:29`, `docs/grove.md:73`,
`docs/workflows/start.md:101` ("In single-harness repos the stamp is not
written"). `start.md` is the page the migration runbook's users read.

**D3. Two current-state docs still name `GROVE_CLAUDE_PID` as the mechanism:**
`docs/adr/self-driving-loop.md:15` (the ADR that *defines* the loop — the
repo's current-state record, not a changelog) and `docs/workflows/finish.md:129`
(the finish cycle under review). The plan's verification step grepped
`src/ content/ tests/` only, so `docs/` was a structural blind spot.
(`docs/research/cross-family-review-providers.md` is a dated research artifact —
leave it.)

**D4. `--harness` has no help text**, on either `StartArgs` (`src/cli.rs:77-78`)
or `RetireArgs` (`:95-96`). `--help` renders a blank description. Nothing names
the valid values (available via `known_names()`), and nothing warns that the
flag now writes a permanent binding — this is the migration mechanism's entire
user surface. `--no-launch` on the same struct has a doc comment.

**D5. `.grove-stamps/` is not gitignored.** Verified: `git status --porcelain`
reports `?? .grove-stamps/` after an explicit-harness run. Runbook step 6
mandates one explicit `grove do --harness` per grove, so this now dirties
`git status` in every migrated repo — in the *main* repo, where the user is
likely mid-work.

---

## Test honesty — green does not mean covered

**T1. `known_names_lists_every_registry_row` is vacuous** (tests/harness.rs:37):
`assert_eq!(known_names(), "claude, codex, pi")` asserts a literal against a
literal. Mutation-verified: replacing `known_names()`'s body with the hardcoded
string — precisely what the contract forbids — leaves all 14 tests green. It
cannot distinguish derived from hardcoded, the only property its name claims.

**T2. The `CRITICAL` symlink assertion is vacuous** (tests/provision.rs:97-102).
`real` was populated by the *same* embed, so in a delete-through scenario the
identical bytes are rewritten and `real/SKILL.md` exists either way.
Mutation-verified: swapping `remove_file` for `remove_dir_all` — the exact
destructive bug the comment names — leaves all 6 provision tests green.
Separately probed: on macOS/rustc 1.97 `remove_dir_all` on a symlink-to-dir
merely unlinks, so **the property is guaranteed by std, not by the guard** —
and `Cargo.toml` declares `rust-version = "1.74"`, below the release where
that hardening landed. Either raise the MSRV or record that the guard is
load-bearing. (Symlink *replacement* is genuinely covered — deleting the branch
turns `provision.rs:93` red.)

**T3. `unknown_review_harness_fails_loudly` does not test the review path**
(tests/loop_driver.rs:821). It sets `GROVE_PLANNING_HARNESS=lemur` and takes the
start-path shortcut (`src/loop_driver.rs:279-281`), which never calls
`resolve_kind`. `GROVE_REVIEW_HARNESS` with a bad value is exercised nowhere.

**T4. The reroute test structurally cannot see B7** (tests/loop_driver.rs:707):
it sets a single `GROVE_SKILL_DIR`, which short-circuits `skill_dir_for` for
every harness, so both read the same file by construction. It also scrubs no
ambient vars, unlike its sibling at `:654`.

**T5. The "must not leak" assertion at `:689` is decorative** — no code path
can consult a foreign harness's var, so it can only fail against an
implementation nobody would write. The discriminating assertion is `:695`.

**T6. Contract clauses with no coverage:** unknown harness name in a *stamp
file*; unknown name via explicit `--harness` in `resolve_for_launch`;
empty-string `GROVE_<KIND>_HARNESS`; empty-string model var; and the
`GROVE_CLAUDE_PID` **co-export** (tests read only the new name, so deleting
`GROVE_CLAUDE_PID=$$` from the wrapper turns nothing red — the half that
protects an old `grove-llm` inside a new loop). `tests/llm_cli.rs` is a
`--help` smoke test only.

**T7. Env cleanup is unguarded across the suite** — `tests/complete.rs:148`
sets and never restores `GROVE_HARNESS_PID` (and destroys the real inherited
`GROVE_CLAUDE_PID`); `tests/provision.rs:134` removes `GROVE_SKILL_DIR` without
restore while restoring `HOME` beside it; `src/provision.rs:180-197` leaves
`HOME` clobbered if its first assert fails. Latent today, cascade-prone.

**T8. `tests/provision.rs:115` has a dead disjunct** —
`err.contains("precious")` is never true (the message interpolates the dest
path), so the assertion is looser than it reads.

---

## Valid, but a decision rather than a defect

- **`maybe_stamp`'s `!explicit` branch is unreachable from `do_grove`**
  (src/harness_stamp.rs:68-75): `resolve_for_launch` already returns or bails
  in every `!explicit` case, so the surviving multi-harness write can only
  rewrite the name it just read. `tests/harness_stamp.rs:44-53` calls it
  directly with a state no production path can produce, green-lighting a no-op.
- **`retire --harness` never stamps** (src/launch.rs:60) — so the explicitness
  rule holds for one of the two verbs that take the flag, and
  `docs/workflows/start.md:101`'s "a single grove never spans harnesses
  mid-flight" is violable with no warning and no record.
- **Explicit stamp overwrites a conflicting binding silently** — no old-vs-new
  comparison. Widens the existing basename-collision hazard from multi-harness
  repos to every repo the runbook migrates.
- **The primary creates its own harness root** (src/provision.rs:47-49 +
  `create_dir_all`): `grove do --harness pi` with no pi installed materialises
  `~/.pi/agent/skills/grove`, against the contract's "absent roots → silently
  skip" with no primary exception. Arguably intended; the comment at `:48`
  ("never create") says otherwise. `tests/provision.rs:130` pre-creates `.pi`,
  so this path is untested.
- **The user's symlink farm is dismantled unlogged.** Verified live: both
  `~/.codex/skills/grove` and `~/.pi/agent/skills/grove` are symlinks to
  `~/.claude/skills/grove`. Replacement is contract-conformant and correctly
  ordered (claude is index 0), but `remove_file` discards the target path with
  no record — log the replaced link target.
- **`$CODEX_HOME` / `$CLAUDE_CONFIG_DIR` are ignored** by provisioning, while
  the spec itself names `$CODEX_HOME`. A user with a relocated codex home gets
  a skill copy codex never reads, announced as success. Also `project_dir` now
  does double duty (repo-local marker in `detect_in_repo`, home-relative
  install marker in `provision_all`) with no note, contradicting
  `harness.rs:72-74`.
- **`KIND_SUFFIXES` (src/loop_driver.rs:206) is a hardcoded array** where
  `env_suffix` is an exhaustive match — the same stale-list smell `known_names()`
  was added to remove, one level down. Low risk: the kind taxonomy is a closed
  set of five (CONTEXT.md), so a sixth is a deliberate change, not a drift.
- **Env-var name derivation is unguarded** (`harness.name.to_uppercase()`
  interpolated into var names). Safe for the current closed registry; a name
  with a hyphen or non-ASCII yields a var no POSIX shell can export, which
  fails *into* B2. Note `signal_file_path:57-69` does sanitise — the convention
  exists and the new derivation breaks it.
- **`HOME=""`** passes `home_dir()`'s unset check (src/provision.rs:71-75),
  yielding relative paths — grove would then read and delete under the *repo*.
  Narrow (daemon/`env -i` contexts) but a one-line `is_absolute()` guard.
- **Non-atomic stamp write** (src/harness_stamp.rs:71-76) and bare io error on
  an unreadable stamp (`:32`, no `.with_context`, unlike src/launch.rs:92-93).
  An empty stamp yields "contents do not match any known harness: ." — reads as
  a formatting bug.
- **No deadline marker on the PID fallback.** Three sites say "one release";
  none names a version or carries a failing test to trip on the next bump.
- **Routing env is inherited by the launched session**: a pi session running a
  rerouted review still has `GROVE_REVIEW_HARNESS=pi` in scope, so a nested
  `grove do` or a doubt-driven spawn re-resolves against it. Nothing marks
  "already the rerouted target".
- **Nothing logs the resolved `(harness, model)` per launch**, so the trial's
  central invariant is unobservable at runtime — and a typo in a var *name*
  (`GROVE_REVIEWS_HARNESS`) produces zero output for a whole month. Worth a
  one-line launch diagnostic given the grove's Done-when.
- **Validation is deferred to the first matching leaf**: `GROVE_REVIEW_HARNESS=py`
  passes startup and errors hours later. `any_harness_override_env` already
  sweeps all five suffixes, so validating all five up front is free.

---

## Rejected after re-reading the artifact

- "`args.harness.is_some()` may be true without the user passing the flag" —
  no; `StartArgs.harness` is a bare `Option<String>` with no `default_value`.
- "The `any_model_env(stamped)` gate can short-circuit past a reroute" — no;
  whenever the launch harness differs, `any_harness_override_env()` is already
  true.
- "`GROVE_<H>_<K>_MODEL` might not beat the base var / might resolve against
  the stamped harness" — both correct as implemented (`:225`, `:288-293`).
- "An unset model var might pass an empty flag" — no; `Option<&str>` guarded at
  `:180-184`.
- "`complete --done` may not distinguish stop from relaunch" — correct and
  covered; `disposition` bypasses env entirely.
- "`detect_in_repo` running on the explicit path is a bug" — harmless (three
  `is_dir` calls); readability nit only.

---

## Disposition

Four fix leaves inserted before `release-stamp-trial-k15`, in dependency order
(the suite must be trustworthy before anything else is verifiable):

1. `review-fix-tests` — B1 + T1–T8.
2. `review-fix-routing` — B2, B5, B6, B7.
3. `review-fix-provision-stamp` — B3, B4, B8, B9, B10, D5.
4. `review-fix-docs` — D1–D4.

The *decision* items above are deliberately not leaves: they need a human call
on scope, and several (MSRV, `$CODEX_HOME`, symlink-farm logging) may be
correct to accept for the trial and revisit after. Raise them in the fix leaves
where they touch the same code.

Not covered by this review: a cross-model (K3) second opinion was offered and
is unconfirmed as of this session — the doubt skill requires per-call
authorisation, and the harness reported no human input had arrived.
