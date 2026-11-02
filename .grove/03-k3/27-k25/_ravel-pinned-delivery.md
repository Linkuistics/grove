# ravel-pinned-delivery-k25 — brief

## Goal

Extend the working local route to exact pinned Git sources, completing the
generic source contract of Ravel before Grove adopts directory delivery. Rhai
native compatibility is independent of this feature.

## Done when

- Public resolution and launch acquire exactly the configured commit and
  subdir. They check the object format and the filesystem, and they use
  noninteractive bounded transport. They execute no hook, filter or submodule,
  and no inherited completion authority reaches them.
- Checked exact cache hits require neither network nor Git. Missing cold pins,
  corrupt cache, object and subdir errors and timeouts keep distinct remedies.
  No nearby revision, branch or source is substituted.
- Pin changes, concurrent publication, mixed required local and Git sources and
  retained readers satisfy the same descriptor, resource and confinement
  contract.
- The full discovery acceptance fixtures of Ravel pass against a reproducible
  matched independent build. So do the applicable repository and book checks,
  the installed layout and the Git transport dependency checks. Owning
  documentation and commits record the handoff.

## Decomposition

The child planner consumes actual local-delivery implementation. Git transport
and extraction must reach existing public readers in each working increment.
Private acquisition machinery alone does not close this node.

`ravel-pinned-delivery-k26` grew these ordered children. Each one leaves the
line green with useful public behaviour at both seams.

1. `ravel-git-acquire-k70`: online exact acquisition, extraction, transport
   hygiene and the Git source record. It removes the temporary refusal.
2. `ravel-git-offline-k71`: the acquisition key record, the checked cache hit
   with no network or Git, corruption, pin changes and key-record races.
3. `ravel-git-bounds-k72`: the deadline, cancellation, child reaping, bounded
   redacted diagnostics and size bounds before extraction.
4. `ravel-git-confinement-k73`: confined reads, harness runs, lifetime and
   nested sessions with Git and mixed sources, on both backends.
5. `ravel-git-install-k74`: the formula decision, the installation
   documentation and installed checks of a miss with Git and a hit without it.
6. `ravel-pinned-handoff-k75`: the final rebase check, the acceptance row
   record, documentation, a temporary installed pair and the recorded commits.

Every child works in the workspace `Ravel.skill-directory-delivery`. It
advances the bookmark `skill-directory-delivery` by jj and never pushes it. It
records its Ravel output commit in its running log and its Grove commit
message. It runs the Ravel tasks that its change touches. It moves the book,
the examples and `CHANGELOG.md` with its behaviour. A producer that earns a
`review-impl` inserts it before `ravel-pinned-handoff-k75`.

## Pointers

Use the reviewed `docs/specs/skill-discovery.md` of Ravel, with its exact-Git,
publication, failure and acceptance sections. Its reviewed form is commit
`822c0ed2f67d` on `skill-directory-delivery`, which
ravel-rhai-skill-delivery-k50 recorded. Use the owning commits and
recorded installed-pair inputs of the preceding local node. Use the repaired
contract from ravel-rhai-skill-delivery-k34, not the historical TypeScript and
schema-3 design. Follow the repository instructions of Ravel.

ravel-local-handoff-k61 closed the local node. Its output commit is
`1d50dae5d29ee5f54e00a2259d392b407ec6509d` on the local bookmark
`skill-directory-delivery`, in the workspace `Ravel.skill-directory-delivery`.
That commit changes only the evidence record. Its parent
`6436e66fdb308e1c06ee683b6e1b7bc341fabe8b` is the build commit, with the build
identity `2ee59f33657690da4facfb7d055527cd3fd12268494dc03bb48e7c0e954bf312`.
The base is `main` `24a545236185`.

The "Local stage record" section of the
Ravel design evidence holds the build inputs, the checks and the installed-pair
smoke. Rerun that smoke with `task smoke:pair OLD=<1.1 executable> NEW=<prefix>`
on macOS.

## Settled inputs

- The identity is that of the local stage: the combined Ravel 2.0.0, script and
  request schema 4 and worker protocol 1.
- The build of this stage also reports `ravel 1.1.0` until the cut. Its
  installed-pair inputs for Grove carry the commit and build identity, as in
  the local stage.
- Item 5 of the stage handoff in the Ravel evidence document lists the work of
  this stage. A cache miss needs Git on the `PATH` of the Ravel CLI. This
  stage decides whether the Homebrew formula declares Git.
- The installed and release preparation tasks are `task smoke`,
  `task install PREFIX=<prefix>`, `task release:test`, `task release:formula`,
  `task release:archives` and `task release:smoke`. The repository and book
  tasks are those of the local stage.

## Notes

Only source acquisition extends here. This stage introduces no skill catalogue,
Grove dependency, native installer or garbage collector. Final publication
belongs to discovery-rollout-k6. Use isolated temporary installs for
acceptance.

At entry the owning pinned implementation session rebases the recorded local
commits onto current Ravel main. It checks the integrated local path and
records the exact base. Keep all work on `skill-directory-delivery`, outside
main, through acceptance. Rollout owns final rebase, checks and landing of the
complete feature. Remove the temporary Git refusal of the local subset and its
fixture when the public Git path works. Neither may survive into publication.
