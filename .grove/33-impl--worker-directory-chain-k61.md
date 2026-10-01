# worker-directory-chain-k61

## Goal

Close the reach the worker's own directory leaves open. The front starts the
worker in a private empty directory, but that directory's ancestors are
neither private nor empty, and Bun resolves some imports from them.

## Context

`package-entry-resolution-k52` found this while asking a different question,
and its evidence is under *Package entry resolution* in the runtime evidence.
Read that section first; it has the source citations and the two tables.

The mechanism, from the bun-v1.4.2 source. For an importer inside the compiled
executable, and for any module with no file location, the resolver takes the
directory the process started in as the importing directory
(`resolve_and_auto_install` in `src/resolver/resolver.rs`). It then walks up
from there as it would from a file. The front creates the worker's directory
with `tempfile` under its own temporary directory, the caller's TMPDIR
(`evaluate` in `crates/harness-dispatch/src/worker.rs`). So the chain is
TMPDIR and everything above it. On Linux that is usually `/tmp`, which any
local user can write. Wherever a tool points TMPDIR into a repository, the
chain is that repository.

What fires today, with every autoload switch off, through the shipped front:
a `node_modules` package in TMPDIR answers a bare import from a module the
policy imports from a `data:` URL, from a `blob:` URL, or from a virtual module
the policy registers with `Bun.plugin`. A module in a file is not affected.
A registered `harness-dispatch/…` specifier still resolves to its embedded
module from a `data:` importer, and an unregistered name under the prefix
loads the shadow. The spec's `#policy-authority` and the README's *One
exception* paragraph state this as open, naming this leaf.

It also blocks `package-json-autoloading-k62`. With package.json autoloading
on, every `package.json` in that chain is read whole on each evaluation, for
every policy. One of 4 GiB stalled a plain routed policy to its deadline, and
one with an `imports` map answered a `#` import from a `data:` module.

The repair is a choice, and each candidate costs something:

- **Start the worker in `/`.** The chain is then `/` alone, which is already
  above every importing module, so no directory gains a say. It gives up the
  empty directory. That directory is one of two independent controls for cwd
  dotenv and bunfig, and no fixture can plant a file in `/` to show the other
  still holds there. A policy's relative writes would land in `/`.
- **Keep a private directory, under a base whose ancestors are the owner's or
  root's**, such as one under HOME. `~/node_modules` and `~/package.json` then
  join the chain. HOME can be unset: the installed smoke test runs without it
  and passes `--state-dir`. A killed front leaves the directory behind where
  nothing sweeps.
- **Keep TMPDIR and check the chain** for a `node_modules` or a writable
  ancestor before starting. A check and a start are a race.

Whether the process's start directory is read once or on each resolution was
not established. `Fs::FileSystem::instance().top_level_dir` looks fixed at
start, which would rule out changing directory from inside the worker.

## Done when

- A command-seam case in `tests/hostile.rs` plants a `node_modules` package in
  TMPDIR and imports it from a `data:` module, a `blob:` module and a
  policy-registered virtual module. Through the front each stays inert. In the
  same case the fixture is seen to fire: the shipped worker started directly
  in a directory under the planted one does load it.
- The firing-configuration table in the spec gains the class, and its
  `#policy-authority` paragraph beginning "One part of that reach is open"
  states the control instead. The README's *One exception* paragraph goes.
  The ADR's sentence naming this leaf says what holds. The runtime evidence
  records what was seen.
- The spec's statement of where the worker runs matches the code, and the
  dotenv and bunfig case still shows each of its two controls holding alone,
  or the spec says which one was given up and why.
- `task check` passes, and `task release:smoke` reruns, since the front
  changes.

## Notes

The private empty directory is a stated control in the spec and in
`docs/adr/policy-evaluation-precedes-process-replacement.md`. Weakening it is
a trade-off for the human to accept. Stop and ask, with a recommendation,
rather than choosing it alone.

This is a security control that later work builds on. Decide at the end
whether it has earned a `review-impl` leaf, and cut it before retiring if so.
