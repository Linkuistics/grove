# worker-directory-chain-k64

**Integrates:** worker-directory-chain-k63

## Goal

Triage the committed review of `worker-directory-chain-k61`, apply the real
findings, and establish a coherent reviewed boundary before
`package-json-autoloading-k62` changes the runtime's discovery behavior.

## Context

Read `worker-directory-chain-k63` from its task commit for the findings and
their evidence, rather than treating this leaf as a prescriptive fix list.
Read the producer commit and running log, the root brief, the spec's
policy-authority and test-seam sections, the worker ADR, and the runtime
evidence's worker-directory section against the current source.

This leaf is immediately after the review so that no implementation changes
intervene in its file and line references. The review made no implementation
or test edits and ran no tests, builds, linting or formatting; its
source-derived behavior predictions need their own disposition and, when
material, measurement here.

## Done when

- Every review finding has a recorded disposition with evidence.
- Accepted findings are repaired, and code, tests and documentation state
  the same contract. Any remaining scope or human trade-off is explicit.
- Required verification passes for the applied changes. Use the Taskfile;
  changes to the worker, front or installed layout rerun the installed smoke
  checks as the root brief requires.
- The next leaf can consume the established boundary without relying on a
  refuted statement or an unmeasured behavior presented as a result.

## Notes

Own fixes and post-fix verification here. The review's handle is the handoff;
its findings are intentionally not copied into this charter.

## Decisions (running log)

**Findings read from the review's commit.** `57e34cc7` holds two findings, F1
and F2, and a table that gives a reason for no finding on each other doubt.
The working copy was empty at entry and the worker and probe builds were
current, at build `3261c425f866…`, which is k61's.

**F1 is real, and is now measured.** The review predicted it from the source
and ran nothing. Seen on macOS 26 arm64 with Bun 1.4.2, driving each build
directly as `tests/support/direct.rs` does. The policy moves into a directory
holding `.env`, `.env.local` and a bunfig preload, then starts a native
`Worker` from a file named by its absolute path.

| Build, where started, where the policy moved | VM that moved | `Worker` started there | bunfig preload |
|---|---|---|---|
| `autoload` probe, an empty directory, the planted one | Nothing | Both variables | Did not run |
| `autoload` probe, an empty directory, another empty one | Nothing | Nothing | Did not run |
| `autoload` probe, an empty directory, no move, so `/` | Nothing | Nothing | Did not run |
| Shipped, an empty directory, the planted one | Nothing | Nothing | Did not run |
| Shipped, the planted directory, the planted one | Nothing | Nothing | Did not run |
| `autoload` probe, the planted directory, no move | Both variables | Both, inherited | Ran |

A `node:worker_threads` `Worker` gave the first and fourth rows' results too.
The last row is the existing firing configuration, kept as a check that the
fixture could fire at start. So Bun loads dotenv files again for each VM it
starts, from the directory the process is then in, and never reloads bunfig.
The shipped switch keeps both out everywhere. The source agrees, read at
`bun-v1.4.2`: `clone_for_worker` (`src/dotenv/env_loader.rs`) empties
`default_files_loaded`, and `start_vm` (`src/jsc/web_worker.rs`) reapplies the
standalone flags and calls `configure_defines`, whose `run_env_loader`
(`src/bundler/transpiler.rs`) lists `top_level_dir` as it is then.

**F1 at `/`, seen in a container.** `/` can hold no fixture on the
development host, so this ran as root in a `rust:1.85` container, Linux
6.10.14 on arm64, with the linux-arm64 archive from `target/archives`, whose
worker reports the same build. A linux-arm64 `autoload` build was made for the
experiment with the pinned runtime. With `/.env`, `/.env.local` and a
`/bunfig.toml` preload planted, and a policy that starts a `Worker` without
moving:

| Build, where started | Main VM | `Worker` | Preload |
|---|---|---|---|
| Shipped, through the front | Nothing | Nothing | Did not run |
| Shipped, driven directly, an empty directory | Nothing | Nothing | Did not run |
| `autoload`, an empty directory | Nothing | Both variables, from `/` | Did not run |
| `autoload`, `/` | Both | Both | Ran |

So for a VM started after the move the private start directory controls
nothing, and the dotenv switch is the one control. Before k61 a worker stayed
in its empty directory, and a `Worker` it started read that. k61's reasoning
that no stated control was weakened is refuted for this path.

**F2 is real, and is now measured.** Same container, the shipped pair through
its front, a policy importing `chain-planted` from a `data:` module.

| Planted | TMPDIR | Result |
|---|---|---|
| Nothing | `/tmp` | Exit 3 |
| `/tmp/node_modules` | `/tmp` | Exit 3, not loaded |
| `/tmp/node_modules` and `/node_modules` | `/tmp` | Selected, the `/` package loaded |
| `/node_modules` | `/` | Selected, the `/` package loaded |
| Removed again | `/` | Exit 3 |

The second row beside the third is also the first hostile-fixture reading of
k61's control on Linux. For the half of F2 that needs the switch on, an
experimental linux-arm64 build with package.json autoloading on, reporting the
shipped identity, was put in a copy of the prefix. A `/package.json` with an
`imports` map answered a `data:` module's `#` import. The same file in `/tmp`
did not, and with the shipped worker neither did. So the documents' "any
directory above where the worker started" is false of `/`, and of a TMPDIR
that is `/`.

**F1 accepted: a real issue, repaired in the tests and the contract.** Nothing
in the shipped pair misbehaves, so no runtime code changes. What was wrong is
the claim. The tripwire read only the VM that had already loaded its
environment, so it reported as unfired a class that fires. That class is now
counted, in
`hostile::a_dotenv_where_a_policy_starts_a_worker_stays_inert_and_fires_under_the_autoload_probe`:
inert through the front under `inspect` and `run`; fired under the `autoload`
probe started in an empty directory; inert in the shipped worker started
there, so the switch is the one difference; and inert under the probe when the
policy moves into an empty directory instead, so where the process is when the
`Worker` starts is what is read. The tripwire keeps what still has no firing
configuration: dotenv for the VM that made the move, and bunfig for that VM
and a `Worker` started there. The spec's `#policy-authority`, its
firing-configuration table and its unfired-class paragraph, the ADR, the
runtime evidence, the changelog and the comments in `main.ts`, `worker.rs` and
above `SHIPPED_SWITCHES` now say that the private start directory is a second
control only for what Bun reads as the process starts.

**No repair restores the second control here.** The resolver and the dotenv
loader read the same directory, the process's. Closing the resolver's reach
needs a directory with no ancestor another user can write, and only `/` is
one; a second dotenv control needs an empty directory, and `/` is not one.
Wrapping `Worker` would be a sandbox the contract disclaims. A front-set
runtime option would be a new control with its own design, probe and review,
which is a producer chain and not this leaf's work.

**F2 accepted: a contract stated unclearly, and the contract is fixed.** The
code was right and the runtime evidence already said so. The spec, the README,
the changelog and the module comment in `worker.rs` now say "between where the
worker started and `/`", and say that `/` itself answers, as it does for a
module in a file. The spec's class name and the hostile case's comment follow.
The test keeps its name, which the brief, k61's log and k62's task cite, and
which is true of its fixture.

**The other doubts stand as the review left them.** It gave a reason for no
finding on each, and none needed a repair. One reading it asked for is now in
the words: the worker moves before it loads any policy code, since its own
embedded imports precede the move. That is fixed where this leaf edited, in
`main.ts`, `worker.rs` and the ADR, and left elsewhere.

**The control was seen to fail.** With dotenv autoloading turned on in
`SHIPPED_SWITCHES` and the worker rebuilt, the new case failed at its front
arm: the `Worker` reported both variables under `inspect`. Under the
same mutation the start-time square,
`a_cwd_dotenv_and_bunfig_preload_stay_inert_and_fire_under_the_autoload_probe`,
passed its front arms and stopped only at the stale probe's identity. So the
private directory still holds for what is read at start and holds nothing for
a later VM. `dispatch.sh` was restored to the same digest afterwards.

**No in-session reviewer was spent.** Both findings were settled by
measurement, and no claim here rests on reasoning the suite or the container
run does not show. k62 cuts its own `review-impl`, and
`dispatch-documentation-k41` cuts the documentation-acceptance review.

**k62's parked patch is unaffected.** `patch -p1 --dry-run` gives the same
result against this tree as against the parent commit: `tests/authority.rs`
fails its one hunk, as k62's task expects, and the other three files apply.

**The human accepted the single control, on 2026-10-01.** k61's task reserved
any weakening of the private start directory for the human, and k61 did not
ask, believing there was none. Asked here with the measurements above and
three ways to settle it: accept, add a second control through a new chain
ahead of k62, or revisit the move through a redesign chain. The recommendation
was to accept, because the move closed a reach open in the shipped build to
anyone who can write TMPDIR, and what it leaves to one control is a file only
the owner of `/` can write, behind a switch a command-seam test holds. The
human chose to accept. So no chain is cut, and `package-json-autoloading-k62`
runs next on this boundary.

**Verification, so far.** With every edit in place, `task check` passed all 12
principal checks at worker build `79a4f60edb07…`. `task release:smoke` then
passed on `aarch64-apple-darwin` with that build, three cases through both
fronts, and stopped before the first Linux case ran. Docker Desktop had paused
its VM while idle, and the smoke run's first `containers/create` could not
resume it: its backend log has `starting VM: starting vm: context deadline
exceeded` at 13:46 local, and the engine answered nothing after that. A
graceful `docker desktop restart` did not stop it. So the two Linux targets
are not yet smoke-tested with this build, and the leaf is not retired until
they are. The four measured source files kept their digests across the run.

**The smoke tooling broke under a Docker upgrade, and is repaired here by the
human's choice.** The human restarted Docker Desktop, which came back as
29.8.1 with a 7.0.14 kernel, where it had been 28.1.1 with 6.10.14. The rerun
passed on macOS and in the arm64 floor container at build `79a4f60edb07…`,
then failed in the helper container of the emulated route: `unshare: unshare
failed: Operation not permitted`. Measured outside the script, in the pinned
ubuntu:24.04 image: the built-in seccomp profile refuses `unshare --user`, an
unconfined AppArmor changes nothing, and either `--security-opt
seccomp=unconfined` or `--cap-add SYS_ADMIN` lets it through. A diagnostic run
of `scripts/release-smoke.sh` with the first of those on the helper container
passed all three targets, with the glibc and CPU controls firing on both Linux
targets. This is release tooling and not a review finding, and it loosens a
container's confinement, so it was put to the human with three ways to settle
it: here with the seccomp option, here with the capability, or as its own
leaf ahead of k62. The recommendation was the seccomp option here, since it
adds no capability and its own controls were seen to fire. The human chose
that. `scripts/release-smoke.sh` states the reason at the helper's `docker
run`, and `docs/RELEASING.md` and *Installed smoke* in the runtime evidence
say what was seen. Why 28.1.1 allowed the call was not established.

**Verified.** With every edit in place, `task check` passed all 12 principal
checks, and `task release:smoke` then passed on all three targets with the
committed script, under Docker 29.8.1, each worker reporting build
`79a4f60edb07…`. On both Linux targets the glibc 2.25 probe was refused, the
CPU model killed its probe while `-cpu max` ran it, and the guest's address
space stayed below its limit with a vDSO. The eight files the two runs read
as their subjects, the four above and the three smoke scripts with
`installed-smoke.sh`, kept their digests from before the first to after the
second. The earlier entry's two Linux targets are therefore no longer owed.

**No review leaf is cut for this integration.** Its changes to shipped
behavior are none: a test, comments, documents and one option on a test
container. `package-json-autoloading-k62` cuts its own `review-impl`, which
reads the hostile suite this leaf changed, and `dispatch-documentation-k41`
cuts the documentation-acceptance review of the documents. This session ran
on a direct harness and has no dispatch run.
