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

## Decisions (running log)

**The start directory is not fixed at start.** The task left this open and
guessed the other way. In bun-v1.4.2, `process.chdir` calls
`VirtualMachine::set_process_cwd` (`src/jsc/VirtualMachine.rs`), which rewrites
`fs.top_level_dir`. That is the value `resolve_and_auto_install`
(`src/resolver/resolver.rs`) substitutes for an importer inside the executable
or with no file location, read at each resolution. So the directory that
matters is the one the process is in when it resolves, not the one it started
in.

**The repair is a fourth candidate: start in the private directory, then move
to `/`.** The worker calls `process.chdir("/")` before it registers or loads
anything. The front still starts it in the private directory. dotenv and
bunfig autoloading read the directory a process starts in, so the private
empty directory keeps its part in that pair of controls, and the resolver
walks `/` alone. That half is seen, not read: `run_env_loader` in
`src/bundler/transpiler.rs` loads `.env` files from `top_level_dir` when it is
called, and its caller on the compiled executable's start path was not traced.
The `autoload` probe loads the files of the directory it starts in and none of
a directory it is later moved into. None of
the task's three candidates is taken, and no stated control is weakened, so
this was not a trade-off to put to the human.

**Seen, macOS 26.6.2 arm64, Bun 1.4.2, through the checkout's front.** With
the move, a `node_modules` package in TMPDIR refused as a missing package from
a `data:` module, a `blob:` module and a policy-registered virtual module.
Without it, each loaded. With package.json autoloading turned on for the
experiment and the move in place, a TMPDIR `package.json` answered neither a
`#` import nor its own name from a `data:` module, and a sparse one of
4,298,113,024 bytes left a plain policy selecting at once. The same build
without the move answered both and timed out at a 5 s bound. So the move also
closes what `package-json-autoloading-k62` needs closed. The switch was put
back to off afterwards.

**One cost, accepted as visible.** A policy's `process.cwd()` is now `/`, so
its relative paths resolve there and a relative write fails, where it used to
land in a directory removed afterwards. The request's `cwd` was always the
caller's directory and host reads resolve against it. A policy that moves
itself moves the resolver with it; that is a policy effect. The spec and
README say so.

**The firing configuration is a fourth probe build, `unmoved`.** The task's
done-when names the shipped worker started under the planted directory. The
shipped worker now moves wherever it is started, so the configuration is the
shipped source without the move, as each other JavaScript-level control has.
The same case also starts the shipped worker in that directory and sees it
refuse, so the move is the one difference between the two arms.

**The control was seen to fail.** With the move replaced by a no-op in the
shipped source, the new case's front arm failed: `inspect` exited 0 where a
missing package is exit 3.

**The private directory was not private, and now is.** Splitting the old
directory test needed a stand-in worker to say where the front starts it,
since the real worker has left by the time a policy can look. The stand-in
reported mode `drwxr-xr-x`. `tempfile` 3.27.0 makes a directory `0o777 &
!umask` unless told otherwise (`Builder::permissions`, in the crate's own
documentation), so under a permissive umask another user could write into the
worker's start directory. The front now asks for `0o700` at creation, which
leaves no window. The stand-in test failed on the mode before the change.

**Two tripwires changed with the move.** Every worker but `unmoved` now leaves
its start directory, so the cwd tsconfig tripwire's entry moves the worker back
into the caller's cwd before importing the alias; otherwise the probe would
never be in that directory when it resolves. A new tripwire covers what the
move could have added: a `.env`, `.env.local` and bunfig preload in a
directory the worker moves into after start, under the `autoload` probe. `/`
cannot hold a fixture, so the policy makes the move into a hostile directory.
Neither fired.

**It has earned a `review-impl` leaf: `worker-directory-chain-k63`, cut ahead
of `package-json-autoloading-k62`.** The repair is none of the three the task
offered and was chosen without the human. k62 rests on it at once. Half its
argument, that dotenv and bunfig are read only where a process starts, is seen
on one host and not traced in the source. The review's body names that and
seven other doubts. With a review scheduled, no in-session reviewer was spent.
This session ran on a direct harness and has no dispatch run, so the review
carries its `**Reviews:**` line and no `**Creator:**` line.

**Done-when, by test.** The package case is
`hostile::a_package_above_the_workers_start_directory_stays_inert_and_fires_under_the_unmoved_probe`.
Where the worker starts and runs is
`worker::the_front_starts_its_worker_in_a_private_empty_directory_and_removes_it`
and
`authority::the_worker_evaluates_policy_in_the_root_directory_with_null_stdin_and_a_fresh_environment`,
which replace the one test that read both from a policy. The dotenv and bunfig
square is unchanged in
`hostile::a_cwd_dotenv_and_bunfig_preload_stay_inert_and_fire_under_the_autoload_probe`.
`bash scripts/check.sh` passed all twelve checks with every edit in place, and
`task release:smoke` passed on the three targets with worker build
`3261c425f866…`.
