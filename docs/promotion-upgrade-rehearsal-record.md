# Promotion upgrade rehearsal record

`promotion-upgrade-rehearsal-k318` rehearses fresh installs and upgrades
through the tap states of
[Compatible tap states](specs/coordinated-release.md#compatible-tap-states)
with real Homebrew. Its result decides whether finish promotes Ravel 2.0.0 or
the grove stays in state C. This record holds each scenario's commands and
results, and the decision once the controls have run. It records no
publication and no change to the owner's installation.

## Instrument

`task promotion:rehearsal WORK=<empty directory> TAP=<tap working copy>` runs
`scripts/promotion-rehearsal.sh`. The script's header states its rules. In
summary:

- The scratch tap is a bare clone of the real tap's Git history,
  `WORK/origin.git`. Its `main` names the tap state of a scenario. State A is
  tap commit `12b53f8c449d`, the commit before `ravel@2`. State B is tap
  commit `f7f9b9b0bc4d`, which adds `ravel@2` 2.0.0. State C is one commit
  on state B that the script makes. It replaces only `Formula/grove.rb` with
  the candidate formula.
- The candidate Grove is the working result of this repository under a
  candidate label. The script copies the files that a commit of the working
  copy would hold into `WORK/candidate-src`. It sets the label in the copy's
  `Cargo.toml` and `Cargo.lock`, and runs `release-build.sh --unreleased`
  there for the three release targets. It renders the copy's
  `scripts/templates/grove.rb.tmpl` with those archives through `file://`
  URLs. It also renders the release form with the same checksums, and fails
  unless the two differ in the three `url` lines only. It commits the
  candidate formula to a clone of the scratch tap and pushes it to the
  scratch `origin`. The real working copy is only read.
- Each scenario has its own private Homebrew. This is a `--shared` clone of
  the owner's Homebrew repository at its HEAD, with its own prefix, HOME, logs
  and temporary directory. All scenarios share one download cache.
- Each private Homebrew taps `WORK/origin.git` as `linkuistics/taps`, because
  the formulae name their dependencies there. A tap state moves through a
  fetch and fast-forward of the tapped clone. That is the tap step of
  `brew update`. The script does not run `brew update`, because that command
  would also move the private Homebrew away from the owner's version.
- The scratch HOME holds a copy of the owner's `~/.homebrew/brew.env`. Every
  package mutation sets `HOMEBREW_NO_INSTALL_CLEANUP=1`, as the owner's
  release tasks do. The rehearsal therefore measures the owner's path.
- `brew test` turns on Homebrew's developer mode. The script turns it off
  after each test round, so each package mutation runs in Homebrew's default
  mode. The owner's Homebrew has developer mode on. That mode changes only
  the commit that `brew update` moves to.
- After each package mutation, the 1.1.0 front of the scratch prefix and the
  real front `/opt/homebrew/Cellar/ravel/1.1.0/bin/ravel` must report
  `ravel 1.1.0`. Each must also refuse `end-session grove-continue` with no
  output. Each call runs under `env -i`. Ravel 1.1.0 finds its session
  through `RAVEL_EXIT_FILE`, so no inherited session authority reaches it.
- Before and after the run, the script digests the protected subjects. These
  are the real prefix's `grove`, `ravel` and `ravel@2` kegs and links, and
  the list of the real Cellar. They are also the real tap working copy, the
  installed real tap and the tap remote's `main`. Then come this
  repository's `Cargo.toml` and `Cargo.lock`. The last are the owner's
  `~/.homebrew` and `~/.config/ravel`. The run fails if any digest changes.

A usable pair at state A or B means four facts. The Homebrew tests of `ravel`
and `grove` pass, and `bin/ravel` links the 1.1.0 keg. With `ravel@2`
installed, its test also passes. `ravel@2` must also report 2.0.0 through
`opt/ravel@2` and have no link in the shared `bin` or `share`. `bin/grove`
must report the version of the tap state.

At state C, a usable pair also has two working setups. One fixture HOME holds
`policy.ts` and `config.rhai` and no `settings.json`. Each file routes the
`impl` kind to a fixture agent. The agent records the Ravel that evaluated the
route, the delivered prompt and the inherited `RAVEL_SKILL_DIRECTORIES`. It
then records a teardown and ends the session, so the Grove loop completes.

- **Native setup** runs `grove` with the prefix's `bin` first on `PATH`. The
  default `ravel` is 1.1.0, which reads `policy.ts`. The agent must receive
  no `RAVEL_SKILL_DIRECTORIES` and a prompt that Ravel did not wrap.
- **Directory setup** runs `grove` with `$(brew --prefix ravel@2)/bin` first
  on `PATH`, as Grove's caveats direct. Ravel 2.0.0 reads `config.rhai`. The
  agent must receive `RAVEL_SKILL_DIRECTORIES` and a prepared prompt that
  names its descriptor. `ravel skills resolve --json` through `ravel@2`, in
  the same HOME, must list the six sources of the candidate keg. Each
  `originPath` must be that source's private copy in the candidate keg.

## States A and B

scratch-homebrew-rehearsal-k333 ran the instrument on 2026-11-12. The owner's
Homebrew was `7.0.13-83-ga6fea6b`, commit `a6fea6bd59bb`. The measured run
used `scripts/promotion-rehearsal.sh` with SHA-256 `be39c5fc22d5…` and
`Taskfile.yml` with SHA-256 `6aa390742eee…`. The digests of both files were
the same before and after the run. The run log stayed in the session's scratch
directory. This section copies the readings that matter.

### Instrument controls

Each check failed on a subject that is known to be wrong:

| Control | Result |
| --- | --- |
| A front that reports `ravel 1.1.0` and accepts `end-session` | `end-session did not refuse (exit 0)` |
| A front that reports `ravel 2.0.0` | `reports 'ravel 2.0.0'` |
| A keg file whose mode changes | The keg manifest changed |

### Tap trust

Homebrew 7.0.13 does not trust a tap when `brew tap` adds it. A fully
qualified name trusts only the named formula, and Homebrew writes that
trust to the HOME's `~/.homebrew/trust.json`. A dependency from an untrusted
tap refuses to load. In fresh-a, a new user's first install refused and left
no keg:

```text
$ brew install linkuistics/taps/grove
==> Trusted formula linkuistics/taps/grove
Error: Refusing to load formula linkuistics/taps/ravel from untrusted tap linkuistics/taps.
Run `brew trust --formula linkuistics/taps/ravel` or `brew trust linkuistics/taps` to trust it.
```

The documented installation therefore fails for every new user at every tap
state. The owner does not see the refusal, because the owner's trust store
already trusts `linkuistics/taps`. Each scenario then ran
`brew trust --tap linkuistics/taps`, as the message directs. Homebrew stored
that trust under the tap's remote URL, in lower case.
homebrew-tap-trust-docs-k337 owns the documentation correction.

### Results

| Scenario | Commands after tapping and trust | Result |
| --- | --- | --- |
| fresh-a | `brew install linkuistics/taps/grove` | Grove 22.8.0 and Ravel 1.1.0. The tests of `ravel` and `grove` pass. `bin/ravel` links `Cellar/ravel/1.1.0`. |
| fresh-b | `brew install --skip-link linkuistics/taps/ravel@2`, then `brew install linkuistics/taps/grove` | Homebrew printed `Skipping 'link' on request` and made no `bin/ravel`. Grove then installed Ravel 1.1.0 as its dependency. The tests of `ravel`, `grove` and `ravel@2` pass. `bin/ravel` links 1.1.0. `opt/ravel@2` links `Cellar/ravel@2/2.0.0` and reports 2.0.0, with no shared link. |
| upgrade-a-b | At A, `brew install linkuistics/taps/grove`. Then the tap moves to B. Then `brew outdated --verbose`, `brew upgrade` and `brew install linkuistics/taps/ravel@2` | At A, the same pair as fresh-a. After the move to B, `brew outdated` lists nothing and `brew upgrade` changes nothing. `ravel@2` installs without `--skip-link`, and Homebrew reports that it `was installed but not linked because linkuistics/taps/ravel is already linked`. At B, the same pair as fresh-b. |

After each package mutation, both 1.1.0 fronts reported `ravel 1.1.0`. These
are the front of the scenario's prefix and the real front. Each refused
`end-session grove-continue` with exit 1 and no output:
`ravel: end-session: no enclosing Ravel session (RAVEL_EXIT_FILE is unset or
empty)`. In upgrade-a-b, the manifest of the scratch 1.1.0 keg at B equals the
manifest at A, apart from `INSTALL_RECEIPT.json`.

The protected subjects had the same digest before and after the run:
`986ec69872d2…`. The run changed nothing in the real prefix, the real tap,
its remote or the owner's HOME.

### Observations for later states

- With `ravel` 1.1.0 linked, Homebrew prints a caveat for `ravel@2`. It says
  `Run brew link ravel@2 to switch the active version to this keg.` At states
  B and C, that command would move the default `ravel` away from 1.1.0, which
  Grove 22.8.0 and the owner's running driver read. The documented route
  selects `ravel@2` through its own prefix. Pages that describe `ravel@2`
  must not repeat Homebrew's suggestion. cross-repo-docs-reconciliation-k319
  can check that none does.
- Ravel 1.1.0 and 2.0.0 print
  `stop attempt … Operation not permitted (os error 1)` at the end of a
  session, and the session still succeeds. State C shows the line also in
  launches outside Homebrew's sandbox. ravel-stop-eperm-k302 measured the
  cause. On Darwin, a process group whose members exited but are not yet
  reaped refuses a group signal with `EPERM`. Ravel 2.0 keeps that
  refusal as a report beside the reap, so the line is not a failure.
- Each private prefix prints Homebrew's warning that the prefix is not
  `/opt/homebrew`, a Tier 3 configuration. The Grove and Ravel formulae have
  no bottles, so the warning changes nothing that these formulae install.

## State C

candidate-state-rehearsal-k334 ran the instrument on 2026-11-12. The owner's
Homebrew was `7.0.13-83-ga6fea6b`, commit `a6fea6bd59bb`. The measured run
used `scripts/promotion-rehearsal.sh` with SHA-256 `d3ed875840ab…` and
`Taskfile.yml` with SHA-256 `6aa390742eee…`. The digests of both files were
the same before and after the run.

After the run, the comment and the
description of the `promotion:rehearsal` task changed to name the release
toolchain. One comment of the script also changed, because
`removed_surface.rs` reads a `GROVE_` word in it as an unclassified
variable. No command changed.

The same run repeated fresh-a, fresh-b and upgrade-a-b with the results of
the section above. The run log stayed in the session's scratch directory.

An earlier run stopped in upgrade-a-c at `brew list --versions`. That run's
formula listing was correct. The cause is in Homebrew, and the
"Observations" below describe it. The instrument then changed to
`brew list --formula --versions`, and the measured run is the complete run
after that change.

### Candidate

The candidate label is `22.9.0-rc.1`. k335 keeps it. The script computes it
as the next minor after the workspace version, 22.8.0, with the pre-release
suffix `-rc.1`. The reasons are:

- Homebrew must see an upgrade, so the label must sort above 22.8.0.
- [Public identities](specs/coordinated-release.md#public-identities) forbid a reservation of
  22.9.0 while `main` can advance. A pre-release label is no release number.
- The owner's Homebrew 7.0.13 sorts `22.9.0-rc.1` above 22.8.0 and below
  22.9.0, so a later 22.9.0 still supersedes it. `brew ruby` with
  `Version#<=>` gave this order. In the same check, `22.9.0-candidate.1`
  sorted above 22.9.0, because Homebrew reads `candidate` as a plain string
  token, not a pre-release token.
- The formula test asserts that `grove --version` equals the formula
  version, so the binaries carry the label. The copy's `Cargo.toml`
  workspace version and the six workspace members of its `Cargo.lock`
  changed. Nothing else changed.

The archive inputs were those of `main` at `dc1b9c8015f`. The working copy
changed only the rehearsal script and the grove's task file, and the archive
holds neither. The packs were at the pins of `plugins/packs.json`. The
archives of the measured run are:

| Archive | SHA-256 |
| --- | --- |
| `grove-v22.9.0-rc.1-aarch64-apple-darwin.tar.xz` | `c1ab230e086f…` |
| `grove-v22.9.0-rc.1-aarch64-unknown-linux-gnu.tar.xz` | `16ae87881a21…` |
| `grove-v22.9.0-rc.1-x86_64-unknown-linux-gnu.tar.xz` | `251226e5bf5e…` |

A rebuild gives other digests, because the archives do not build
reproducibly byte for byte. The earlier run's archives had other digests.

The candidate formula differed from the release rendering only in its three
`url` lines, each a `file://` URL into `WORK/candidate`. The script pushed it
to the scratch `origin` as state C, commit `8cf2c34eb17f`. That commit's
parent is state B, and it changes only `Formula/grove.rb`.

### Controls of the setup checks

After the run, the checks of `setups` ran against readings that are known to
be wrong. The readings came from upgrade-a-c. Each check failed:

| Subject | Check | Result |
| --- | --- | --- |
| Readings of the native launch | Directory setup | Fails: the route ran `opt/ravel/bin/ravel` |
| Readings of the directory launch | Native setup | Fails: the route ran `opt/ravel@2/bin/ravel` |
| Native readings with the `ravel@2` path written into `ravel.txt` | Directory setup | Fails: no descriptor |
| The same, with the directory launch's descriptor added | Directory setup | Fails: no prepared prompt |
| `resolve.json` against a decoy directory with the six source names | Source resolution | Fails: `originPath` names the candidate keg, not the decoy |

### Results

| Scenario | Commands after tapping and trust | Result |
| --- | --- | --- |
| fresh-c | `brew install linkuistics/taps/grove`, then `brew install linkuistics/taps/ravel@2` | Grove 22.9.0-rc.1 installed with Ravel 1.1.0 as its dependency. `ravel@2` installed keg-only and unlinked, beside the linked 1.1.0. The tests of `ravel`, `grove` and `ravel@2` pass. The Grove test launched through both Ravel builds. `bin/grove` reports 22.9.0-rc.1, and `bin/ravel` links 1.1.0. Native setup and directory setup work. |
| upgrade-a-c | At A, `brew install linkuistics/taps/grove`. Then the tap moves to C. Then `brew outdated --verbose`, `brew upgrade` and `brew install linkuistics/taps/ravel@2` | At A, the pair of fresh-a. At C, `brew outdated` lists only Grove. `brew upgrade` upgrades only Grove. The 22.8.0 keg stays beside 22.9.0-rc.1, because install cleanup is off. After the `ravel@2` installation, the pair and both setups are those of fresh-c. The Ravel 1.1.0 keg is unchanged apart from `INSTALL_RECEIPT.json`. |
| upgrade-b-c | At B, `brew install linkuistics/taps/grove`, then `brew install linkuistics/taps/ravel@2`. Then the tap moves to C. Then `brew outdated --verbose` and `brew upgrade` | At B, the pair of upgrade-a-b at B. At C, `brew outdated` and `brew upgrade` behave as in upgrade-a-c. The pair and both setups are those of fresh-c. The Ravel 1.1.0 keg and the `ravel@2` 2.0.0 keg are unchanged apart from `INSTALL_RECEIPT.json`. |

In each scenario at C, the two setups gave the same readings:

- Native setup: Ravel 1.1.0 evaluated `policy.ts`. The agent received no
  `RAVEL_SKILL_DIRECTORIES`, and Ravel did not wrap Grove's prompt. Grove
  printed `grove finished — loop complete.`
- Directory setup: Ravel 2.0.0 through `ravel@2` evaluated `config.rhai`.
  The agent received `RAVEL_SKILL_DIRECTORIES`, which names a descriptor
  under the fixture HOME's `.cache/ravel/skills/v1/descriptors`. The
  prepared prompt holds `BEGIN RAVEL CALLER TASK` and names the same
  descriptor. Grove printed `grove finished — loop complete.`
  `ravel skills resolve --json` resolved `design engineering grove-core
  requirements research writing` from
  `Cellar/grove/22.9.0-rc.1/libexec/skill-sources`. The HOME held no
  `settings.json`.

After each package mutation, both 1.1.0 fronts reported `ravel 1.1.0`. These
are the front of the scenario's prefix and the real front. Each refused
`end-session grove-continue` with exit 1 and no output:
`ravel: end-session: no enclosing Ravel session (RAVEL_EXIT_FILE is unset or
empty)`.

The protected subjects had the same digest before and after the run:
`6088371d497c…`. This digest differs from the digest of the section above,
because the subjects now include this repository's `Cargo.toml` and
`Cargo.lock`. The run changed nothing in the real prefix, the real tap, its
remote, the owner's HOME or this repository's version files.

### Homebrew output of the upgrades

Both upgrades to C printed the same output, apart from the scenario's path.
This is upgrade-a-c, with the private prefix written as `WORK/…`, without
the download lines, Homebrew's warning about a prefix that is not
`/opt/homebrew`, and the Grove caveats:

```text
$ git -C $(brew --repository linkuistics/taps) pull --ff-only
tap HEAD: 8cf2c34eb17f5f07e6d7a4b6253a21b188fc3f78
$ brew outdated --verbose
linkuistics/taps/grove (22.8.0) < 22.9.0-rc.1
[exit 0]
$ brew upgrade
==> Would upgrade 1 outdated package
linkuistics/taps/grove 22.8.0 -> 22.9.0-rc.1
✔︎ Formula grove (22.9.0-rc.1)
==> Upgrading linkuistics/taps/grove
  22.8.0 -> 22.9.0-rc.1
🍺  WORK/upgrade-a-c/brew/Cellar/grove/22.9.0-rc.1: 140 files, 4.9MB, built in 1 second
[exit 0]
```

### Observations for state D

- Homebrew 7.0.13 lists `brew list --versions` through a shell fast path
  only while each rack holds one keg. After an upgrade with cleanup off, the
  Grove rack holds two kegs. Homebrew then uses `cmd/list.rb`, which also
  lists casks and fails on a prefix without a `Caskroom`. The owner's prefix
  has a `Caskroom`, so the defect touches only a private prefix. The
  instrument lists formulae only.
- An upgrade at C leaves Grove 22.8.0 in the Cellar. That keg is not linked.
  `share/ravel/skills` in the prefix then names the candidate keg's sources
  only, as `ravel skills resolve` showed.
- A HOME with both configuration files serves both setups at C. State D
  must show the `config_format_retired` refusal for a HOME that holds only
  `policy.ts`. The fixture of `setups` in the script writes both files, so
  k335 needs a separate HOME for that case.
