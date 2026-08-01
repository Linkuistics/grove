# Maintaining the Herdr fork

## Problem

Grove's pane-state surface depends on a two-hunk patch to Herdr that upstream
does not carry, and — since offering it upstream was considered and rejected —
is not expected to. [Grove's architecture](../docs/ARCHITECTURE.md#herdr-optional-ui)
records that decision and the principle the patch encodes. This runbook covers
the consequence: the fork is a
**permanent carry**, so rebasing it onto each new upstream release is recurring
work, and it has enough traps to lose an afternoon to on the first attempt.

Everything here is knowledge about a repository we do not control. herdr moves
fast; treat it as a map, not a contract, and re-verify anything load-bearing.
Last re-verified **2026-07-27** against `upstream/master` at `dc2506ea`, with
`authority-fix` at `b1484e37` and `ui-layout` at `d17e0f42` — except the
flaky-test section, which was not re-run (see the note there).

## Solution

Two branches on `AntonyBlakey/herdr`, with different jobs, so that a rebase
never forces a rewrite of published history.

- **`authority-fix`** — branched off `upstream/master`, carrying *only* the
  patch as a single commit. Its job is to stay a clean, isolated rebase target.
  Rebasing it is the whole of the recurring work.
- **`ui-layout`** — the ship branch, from which the Homebrew formula builds. It
  carries pre-existing fork feature work *and* takes the patch by **merge**, not
  cherry-pick or rebase, so it always fast-forwards from its remote and never
  needs a force-push. That feature work is two `feat:` commits — the `ui.layout`
  API method (drawn cell-rects for sidebar and tab-bar entries) and the CLI verb
  reaching it — carried for **Modaliser**, which paints chips over herdr's own
  chrome. Upstream has no `ui.layout` in any form, so it is a permanent carry on
  the same terms as the patch: the branch is never retired, and its end
  condition is upstream shipping an equivalent geometry API.

The recurring cycle is therefore: rebase `authority-fix` onto the new
`upstream/master`, verify, then merge `authority-fix` into `ui-layout` and ship.

The two invariants worth checking before believing the layout is intact are
`git rev-list --count ui-layout..authority-fix` (must be 0 — the ship branch
contains the fix) and the tap formula's pinned `revision` matching `ui-layout`
HEAD. Both held at the last re-verification.

**How far behind the rebase is** is not the same question as **how much churn
the patch's seam has taken**. Upstream commits that never touch
`src/terminal/state.rs` cost the rebase nothing however many there are, so the
useful measure is
`git diff --stat $(git merge-base authority-fix upstream/master) upstream/master
-- src/terminal/state.rs`. An earlier +1281/−812 on that file is what made a
previous rebase expensive; at the last check it was empty across five upstream
commits.

## Decisions

### Versioning is a sequence, not a sha

The fork ships from `linkuistics/taps` as **upstream's version plus
`-linkuistics.<seq>`** — `0.7.5-linkuistics.1`. `<seq>` increments per ship and
resets when upstream's version bumps. A commit sha was rejected because shas do
not order, and Homebrew needs an ordering to recognise an upgrade.

The formula is `Formula/linkuistics-herdr.rb`, and it pins `ui-layout` by
explicit `revision:` — so shipping is *two* edits, the `version` and the
`revision`, and a rebase that updates only one of them ships the wrong tree under
the right name. It is also where the `ZIG` override below is applied for anyone
installing rather than building by hand.

Two consequences that have already bitten:

- **A suffix rename can sort as a downgrade.** The one-time move from
  `-uilayout.<sha>` to `-linkuistics.<seq>` sorted *backwards* under Homebrew's
  own `Version` comparison, because `linkuistics` < `uilayout` alphabetically.
  That transition needed `brew reinstall`; ordinary `<seq>` bumps upgrade
  normally. Any future suffix rename must be checked the same way.
- **`herdr --version` cannot tell you which build is installed.** It prints bare
  upstream (`0.7.5`) either way, because the suffix is Homebrew's, not Cargo's.
  Use `readlink -f "$(which herdr)"` for the Cellar path, or
  `brew list --versions`.

### The build environment has two required overrides

herdr wires `core.hooksPath` to its in-repo `.githooks/`, whose `pre-commit` runs
`just lint`, so a commit on the fork fails without both of these — they are not
optional conveniences:

- **Zig must be 0.15.** The vendored `libghostty-vt` refuses newer Zig, and
  Homebrew's default is ahead of it. Point `ZIG` at the 0.15 keg explicitly.
- **rustup's cargo must win over Homebrew's.** herdr pins its toolchain in
  `rust-toolchain.toml`; only rustup's shim honours that pin. With Homebrew's
  cargo first on `PATH` the pin is ignored and the newer clippy invents failures
  in files the patch never touches. Put `$HOME/.cargo/bin` ahead of Homebrew's
  bin.

Both are still live traps, not historical ones: Homebrew's default `zig` is
ahead of 0.15 on this machine, and `command -v cargo` resolves to Homebrew's.
Check them each time rather than assuming a previous session's shell survived —
the failure mode is a wall of clippy errors in untouched files, which reads as
"the rebase broke something" and is not.

**This section is about building the fork by hand, and nothing else.** The same
Homebrew-vs-rustup trap bites grove's *own* release, where it is handled by
construction rather than by memory: `scripts/release-common.sh` puts rustup's
shim directory at the front of `PATH` and `release-doctor.sh` verifies the
outcome against the resolved toolchain. Do not borrow the manual `export` above
into grove's release path — it is already pinned — and do not expect grove's pin
to help here, because a herdr build is not run through grove's scripts.

### Two upstream tests are flaky, independently of the patch

A parallel test run flakes on the order of a dozen tests on the *unpatched* tree
too. Run the suite single-threaded. Two tests fail even then — one in workspace
id generation, one in the pane graphics stream — and both pass in isolation;
they were verified identical on unpatched `upstream/master`, so they are
upstream's flakes and not a regression signal. The baseline that matters is the
delta: the patch should add exactly its own tests and change nothing else.

*Not re-verified at the 2026-07-27 pass.* Which particular tests flake is the
least durable knowledge in this spec and the most expensive to re-establish (a
cold build of the vendored `libghostty-vt`), and it gates nothing — the delta,
not the absolute pass list, is the signal. Re-establish it as part of a rebase,
where the unpatched baseline has to be built anyway.

### An installed build does not take effect until the server restarts

The patch lives in code the herdr **server** runs. Installing a new build leaves
the already-running server in place, so grove's reports keep being dropped until
herdr restarts. Any "the patch doesn't work" report must first establish that the
running server postdates the install.

Establish it without restarting anything: compare the start time of the
`herdr server` process (`ps -eo pid,lstart,args | grep 'herdr server'`) against
the patch commit's own date — if the server started before the commit existed, it
provably does not carry it, and no amount of testing against that server means
anything. The Cellar install time (`stat`, or the `time` field in
`INSTALL_RECEIPT.json`) is the same check one step later in the chain.

This trap is not hypothetical; it was live at the 2026-07-27 re-verification,
stacked with a second one — the *shipped* `grove` had no reporter compiled in at
all. Two independent silences produce the same symptom as a broken patch, so
check both binaries before suspecting the code.

### Restarting need not kill panes — prefer the live handoff

"Restart" names two operations with very different costs, and only one destroys
work. Conflating them is what makes a stale server look like an unavoidable
price, so the distinction is load-bearing rather than pedantic:

- **`herdr server stop`**, and any plain restart after it, kills every pane.
- **`herdr server live-handoff --import-exe <path>`** replaces the server
  process while **preserving pane processes**: it binds a handoff socket,
  captures each pane's file descriptors, pauses the readers, spawns
  `<path> server --handoff-import`, and passes the fds across. It has rollback
  logic if the import server fails to come up.

The handoff is a **first-class CLI subcommand**, listed in `herdr server`'s own
help — not a raw socket call. Do **not** reach for `herdr update --handoff`
instead: that fetches *upstream* herdr and would clobber the fork.

Demonstrated 2026-07-27: `herdr server live-handoff --import-exe
/opt/homebrew/bin/herdr` swapped a 24 Jul server (PID 3825) for a patched one
(PID 77248) with every pane surviving — including the pane that issued the swap.

Two bounded costs, neither fatal. Connected TUI clients are disconnected with
"live update in progress; reconnect after handoff completes", so panes live but
the UI must reattach. And a handoff carries at most `MAX_FDS_PER_HANDOFF` (64)
panes, above which it refuses cleanly rather than half-migrating.

It stays the human's call, because it interrupts their UI. But "restarting kills
every pane" is not a property of restarting.

## Verifying a rebase

The acceptance test for the carry, run against a scratch pane whose session
identity is owned by a harness integration (which is the situation the patch
exists for):

1. Report a state as grove — the pane's `agent` becomes `grove` and
   `agent_status` becomes the reported state.
2. Report a second, different state — it lands too. This is the half that fails
   on a stock server, where the first report may land and every later one is
   dropped, freezing the pane.
3. Release grove's authority — the pane returns to screen detection.
4. Throughout, the pane's `agent_session` must stay exactly as the harness left
   it. This is the second hunk's whole purpose: grove never disturbs session
   resume.

Step 3's observable is `agent: null` with `agent_status: "unknown"`, **not** an
immediate return to the previously detected agent. Release hands the pane back to
screen detection, but detection re-runs on its next sweep rather than
synchronously, so a checker that asserts "back to the detected agent" right after
the release will fail against a working patch.

Whether it *ever* returns depends on there still being a harness to detect, and
the two cases are worth separating because they look like the same bug. On a
`grove do` pane whose harness is **still running**, release is followed by
`agent: null` for roughly **14 seconds** and then a re-acquisition as the launched
harness — measured 2026-07-28, and it re-acquires as `claude` rather than `codex`
because grove sets the `HERDR_AGENT` hint (*herdr-optional-ui*). After
`complete --done` the harness has already exited, the hint exits with it, and the
pane stays at `agent: null` indefinitely. So "released and never came back" is
correct at the end of a grove and a symptom mid-loop; a checker must say which it
is testing, and must wait out at least one sweep either way.

Watching a real `grove do` pane rather than driving the CLI by hand, the
`idle` that ADR *herdr-optional-ui* places before the release on `complete
--done` is **not externally observable**: the report and the release land inside
the same socket exchange, and polling at 30 ms still sees `working` go straight
to released. That is not a defect — released *is* the intended resting state —
but a watcher must not wait for an `idle` that will never be sampled.

For the same reason, **`herdr pane get`'s `revision` cannot be used to count
reports**: it tracks pane lifecycle, not agent state, and does not move when a
state report lands (measured across three consecutive state changes). So a
report that re-asserts the state the pane already holds is indistinguishable
from no report at all. A checker that needs to prove a *silent* step — the
driver's relaunch row, say — must either arrange a different pre-state to watch
persist, or assert on the one transition nothing else produces: the `agent=null`
that only a release yields.

A watcher on a real **claude** `grove do` pane also sees reports the driver did
not make. Turn hooks report from inside the session — `working` at every prompt
submit, `working` or `blocked` at every turn end, `blocked` while a dialog waits
on a human, and `working` at **every tool call**
(*herdr-turn-boundary-hooks*) — so the report *count* is not a fixture and is not
even bounded by the number of turns, and a `blocked` mid-session is normal rather
than a sign the loop parked. Assert on the pane's resting state at each step,
never on how many reports arrived.

**Do not use `revision` as the signal.** It does not move for a state report,
landed or dropped. Earlier notes in this workstream claimed otherwise and were
wrong. The observables are `agent` and `agent_status` (read them with
`herdr pane current` or `herdr pane get`).

Nor is `revision` merely "display-metadata tokens", as this spec previously said:
it bumps on a metadata-token patch, on metadata-token *expiry*, and on a change
to the pane's **stripped terminal title**. The last one matters here, because a
`grove do` pane's title is grove's own — so a watcher would see `revision` move
for reasons that have nothing to do with either metadata or state, and read that
as confirmation.

Two CLI traps when driving this by hand:

- `herdr pane report-agent` takes its **pane id positionally, first**. Flags
  placed before the positional fail with a bare `unknown option: <value>` — and
  `--help` prints a usage line showing the positional *last*, which is what makes
  this worth writing down. Same for `release-agent`.
- The CLI **exits 0 on a protocol error**, returning the failure in the JSON body
  (`{"error":{"code":"pane_not_found",…}}`). Any scripted check must parse the
  body; an exit-status check passes on every failure.

A safe way to test argument handling without mutating a pane: aim the command at
a pane id that does not exist. A parse failure and a `pane_not_found` are
distinguishable, and neither touches real state.

### Measuring detection on a grove pane

Anything that checks what herdr *detects* — as opposed to what grove reports —
has to get grove's own authority out of the way first, and there are two ways
that go wrong.

**Prefer a `grove retire` pane over `release-agent` on a `grove do` one.** The
retire path makes no herdr call at all (it is a plain `Command::status`, and the
process shape is identical to `grove do` — unidentifiable leader, live harness,
MCP helpers), so its pane is a free window onto raw detection with the
`HERDR_AGENT` hint live: no release to issue, and no race against grove's own
next report. On a `grove do` pane you must release first, then wait out a
detection sweep (~14 s, measured 2026-07-28), and grove's turn hooks will
self-heal the pane back to `agent: grove` at the next tool call.

**A synthetic rig built from system binaries proves nothing.** macOS discloses
no environment at all for SIP-protected platform binaries through
`kern_procargs2`, which is exactly what herdr reads: `/bin/sleep` and
`caffeinate` show zero tokens where `claude`, `codex` and `grove` show ~100. So
a hand-rolled reproduction that hints a system binary silently exercises the
group-scoring fallback instead of the hint path, and reads as "the hint does not
work". Copying a signed system binary to dodge it does not help either — the
copy fails its code signature and is killed on Apple Silicon. **Compile a
throwaway.** This is harmless in production, where no harness is a platform
binary; it only bites while building a rig.

## Out of scope

- **Upstreaming.** Decided against, for **both** patches — the fork is ours to
  maintain, not a staging area for contributions. See
  [Grove's architecture](../docs/ARCHITECTURE.md#herdr-optional-ui)
  for the reasoning on the authority patch and for what would reopen it. Do not
  open PRs or file issues against `ogulcancelik/herdr`. The tap formula's
  `caveats` say the same thing, because `brew info` is where a user meets this
  decision first.
- **Growing the patch.** The two hunks encode one principle. Because the carry is
  permanent, every additional hunk is a rebase obligation forever, so a third
  must clear the same bar — not merely be useful.
- **Pane mis-detection.** herdr labelling a grove pane with the wrong agent is a
  separate problem, and it was never fixed by this patch. It is **fixed**, by
  grove setting herdr's own documented `HERDR_AGENT=<harness>` hint on the
  harness child — no fork hunk, and it works on stock herdr too. That is why
  *Verifying a rebase* above can assume a released pane re-acquires as the
  launched harness rather than as `codex`. See `CONTEXT.md` and ADR
  *herdr-optional-ui*.
