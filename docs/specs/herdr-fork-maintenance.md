# herdr-fork-maintenance

## Problem

grove's pane-state surface depends on a two-hunk patch to herdr that upstream
does not carry, and — since offering it upstream was considered and rejected —
is not expected to. ADR *herdr-optional-ui* records that decision and the
principle the patch encodes. This spec covers the consequence: the fork is a
**permanent carry**, so rebasing it onto each new upstream release is recurring
work, and it has enough traps to lose an afternoon to on the first attempt.

Everything here is knowledge about a repository we do not control. It was
accurate when written and herdr moves fast; treat it as a map, not a contract,
and re-verify anything load-bearing.

## Solution

Two branches on `AntonyBlakey/herdr`, with different jobs, so that a rebase
never forces a rewrite of published history.

- **`authority-fix`** — branched off `upstream/master`, carrying *only* the
  patch as a single commit. Its job is to stay a clean, isolated rebase target.
  Rebasing it is the whole of the recurring work.
- **`ui-layout`** — the ship branch, from which the Homebrew formula builds. It
  carries pre-existing fork feature work *and* takes the patch by **merge**, not
  cherry-pick or rebase, so it always fast-forwards from its remote and never
  needs a force-push.

The recurring cycle is therefore: rebase `authority-fix` onto the new
`upstream/master`, verify, then merge `authority-fix` into `ui-layout` and ship.

## Decisions

### Versioning is a sequence, not a sha

The fork ships from `linkuistics/taps` as **upstream's version plus
`-linkuistics.<seq>`** — `0.7.5-linkuistics.1`. `<seq>` increments per ship and
resets when upstream's version bumps. A commit sha was rejected because shas do
not order, and Homebrew needs an ordering to recognise an upgrade.

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

herdr's `pre-commit` hook runs its lint recipe, so a commit on the fork fails
without both of these — they are not optional conveniences:

- **Zig must be 0.15.** The vendored `libghostty-vt` refuses newer Zig, and
  Homebrew's default is ahead of it. Point `ZIG` at the 0.15 keg explicitly.
- **rustup's cargo must win over Homebrew's.** herdr pins its toolchain in
  `rust-toolchain.toml`; only rustup's shim honours that pin. With Homebrew's
  cargo first on `PATH` the pin is ignored and the newer clippy invents failures
  in files the patch never touches. Put `$HOME/.cargo/bin` ahead of Homebrew's
  bin.

### Two upstream tests are flaky, independently of the patch

A parallel test run flakes on the order of a dozen tests on the *unpatched* tree
too. Run the suite single-threaded. Two tests fail even then — one in workspace
id generation, one in the pane graphics stream — and both pass in isolation;
they were verified identical on unpatched `upstream/master`, so they are
upstream's flakes and not a regression signal. The baseline that matters is the
delta: the patch should add exactly its own tests and change nothing else.

### An installed build does not take effect until the server restarts

The patch lives in code the herdr **server** runs. Installing a new build leaves
the already-running server in place, so grove's reports keep being dropped until
herdr restarts. Restarting kills every pane, which makes it a human's decision,
never an agent's. Any "the patch doesn't work" report must first establish that
the running server postdates the install.

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

**Do not use `revision` as the signal.** It tracks display-metadata token
changes only, and does not move for a state report, landed or dropped. Earlier
notes in this workstream claimed otherwise and were wrong. The observables are
`agent` and `agent_status`.

One CLI trap when driving this by hand: `herdr pane report-agent` takes its
**pane id positionally, first**. Flags placed before the positional fail with a
bare `unknown option: <value>`.

## Out of scope

- **Upstreaming.** Decided against; see ADR *herdr-optional-ui* for the reasons
  and for what would reopen it. Do not open PRs or file issues against
  `ogulcancelik/herdr`.
- **Growing the patch.** The two hunks encode one principle. Because the carry is
  permanent, every additional hunk is a rebase obligation forever, so a third
  must clear the same bar — not merely be useful.
- **Pane mis-detection.** herdr labelling a grove pane with the wrong agent is a
  separate problem with its own open question, and it is not fixed by this patch.
  See `CONTEXT.md`.
