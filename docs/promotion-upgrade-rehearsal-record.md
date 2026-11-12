# Promotion upgrade rehearsal record

`promotion-upgrade-rehearsal-k318` rehearses fresh installs and upgrades
through the tap states of
[Compatible tap states](specs/coordinated-release.md#compatible-tap-states)
with real Homebrew. Its result decides whether finish promotes Ravel 2.0.0 or
the grove stays in state C. This record holds each scenario's commands and
results, and the decision once the controls have run. It records no
publication and no change to the owner's installation.

## Instrument

`task promotion:rehearsal WORK=<empty directory> TAP=<tap working copy>
RAVEL=<Ravel's default workspace>` runs `scripts/promotion-rehearsal.sh`. The script's header states its rules. In
summary:

- The scratch tap is a bare clone of the real tap's Git history,
  `WORK/origin.git`. Its `main` names the tap state of a scenario. State A is
  tap commit `12b53f8c449d`, the commit before `ravel@2`. State B is tap
  commit `f7f9b9b0bc4d`, which adds `ravel@2` 2.0.0. State C is one commit
  on state B that the script makes. It replaces only `Formula/grove.rb` with
  the candidate formula. State D is the commit on state C that Ravel's
  `release:promote` makes.
- The candidate Grove is the working result of this repository under the
  next minor version, 22.9.0 while the workspace is 22.8.0. "State D" below
  gives the reason. The script copies the files that a commit of the working
  copy would hold into `WORK/candidate-src`. It sets the label in the copy's
  `Cargo.toml` and `Cargo.lock`, and runs `release-build.sh --unreleased`
  there for the three release targets. It renders the copy's
  `scripts/templates/grove.rb.tmpl` with those archives through `file://`
  URLs. It also renders the release form with the same checksums, and fails
  unless the two differ in the three `url` lines only. It commits the
  candidate formula to a clone of the scratch tap and pushes it to the
  scratch `origin`. The real working copy is only read.
- State D comes from `task release:promote VERSION=2.0.0` in `WORK/ravel`,
  a colocated jj clone of Ravel's `main`. The promotion writes its evidence
  in that clone, never in Ravel's workspace. The clone's `origin` fetches
  the tag, the Release and the book from Ravel's GitHub repository. Its push
  URL names no repository. `RAVEL_TAP_DIR` is `WORK/tap-promote`, a
  colocated jj clone of the scratch `origin` at state C. The evidence is a
  copy of the cut's evidence from Ravel's workspace. The readiness receipt
  names the candidate Grove.
- The promotion runs with a `brew` wrapper first on `PATH`. The wrapper runs
  the private Homebrew of the upgrade-c-d scenario with that scenario's HOME.
  Ravel's `release-install.sh` runs `brew update`. In Homebrew's default
  mode that command moves to the latest stable tag, `7.0.13`, away from the
  owner's commit. The owner's Homebrew has developer mode on, which keeps
  `origin/main`. The scenario therefore turns developer mode on before the
  promotion. The run fails if Homebrew's commit moves.
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
  output. A fresh install at D has no 1.1.0 keg, so there the real front
  alone is checked. Each call runs under `env -i`. Ravel 1.1.0 finds its session
  through `RAVEL_EXIT_FILE`, so no inherited session authority reaches it.
- Before and after the run, the script digests the protected subjects. These
  are the real prefix's `grove`, `ravel` and `ravel@2` kegs and links, and
  the list of the real Cellar. They are also the real tap working copy, the
  installed real tap and the tap remote's `main`. Then come this
  repository's `Cargo.toml` and `Cargo.lock`, and the owner's `~/.homebrew`
  and `~/.config/ravel`. The last are Ravel's `main` and its
  `target/release-evidence`. The run fails if any digest changes.

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

At state D, the default `ravel` is 2.0.0, and `bin/ravel` links the 2.0.0
keg. With `ravel@2` installed, it must also be deprecated in favour of
`ravel`. Each upgrade to D then has two more checks:

- **Retired format.** A HOME holds only `policy.ts`. A direct
  `ravel start-session` must refuse with `config_format_retired`, exit 3 and
  the remedy that names chapter 4 of the book. A Grove launch must show the
  same refusal and remedy, report exit status 3 and keep the leaf live.
- **Conversion.** The HOME gains `config.rhai` with the same route, and
  keeps `policy.ts`, as Grove's caveats direct while a Ravel 1.x launch
  remains. A Grove launch through the default `ravel` must then deliver the
  candidate keg's skills, with the readings of directory setup.

A fresh install at D copies Grove's `config.rhai` sample, as the caveats
direct. A fake `claude` first on `PATH` records its arguments and prompt, and
ends the session through the default `ravel`.

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

The candidate label of this run was `22.9.0-rc.1`. State D replaced it with
`22.9.0`, and "Candidate label" in "State D" gives the reason. The script
computed the earlier label as the next minor after the workspace version,
22.8.0, with the pre-release suffix `-rc.1`. The reasons were:

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

## State D

promotion-state-rehearsal-k335 ran the instrument on 2026-11-12. The owner's
Homebrew was `7.0.13-83-ga6fea6b`, commit `a6fea6bd59bb`. Ravel's `main` was
`ed6cedbf9a14`, the tag commit of v2.0.0. The measured run used
`scripts/promotion-rehearsal.sh` with SHA-256 `0eccc6f3511a…` and
`Taskfile.yml` with SHA-256 `4a7cfd4abaad…`. The digests of both files were
the same before and after the run. It was the first run after the changes,
and it passed.

The run log stayed in the session's scratch directory.

### Candidate label

The candidate label is now `22.9.0`, the next minor after the workspace
version, for every candidate scenario. Ravel's `release-promote.sh` admits
only an `X.Y.Z` `consumer.version` in the readiness receipt. The tap's Grove
formula must carry the same version. The label `22.9.0-rc.1` of state C
therefore cannot reach state D. The real state C holds a released Grove
`X.Y.Z`, so the release-shaped label is also the faithful one.

The label stays in the scratch tap and the work directory. It publishes no
version, and it reserves no public identity. The second reason of state C,
that a later real 22.9.0 supersedes the candidate, concerns only a scratch
prefix, which the run discards.

The same run repeated states A to C with the new label. Each check of the
sections above passed. The tables of "State C" hold for `22.9.0` in place
of `22.9.0-rc.1`. The candidate archives of this run are:

| Archive | SHA-256 |
| --- | --- |
| `grove-v22.9.0-aarch64-apple-darwin.tar.xz` | `f8345b40075f…` |
| `grove-v22.9.0-aarch64-unknown-linux-gnu.tar.xz` | `ab608e88e6b1…` |
| `grove-v22.9.0-x86_64-unknown-linux-gnu.tar.xz` | `746eca2751e4…` |

State C was tap commit `2253f5fbe4c8`.

### Promotion

The upgrade-c-d scenario installed Grove 22.9.0 and `ravel@2` at state C,
with the pair of fresh-c. It then wrote the readiness receipt from the cut's
evidence:

```json
{"name":"grove","version":"22.9.0","formula":"grove.rb","verifiedAt":"2026-11-12T12:38:05Z","verifiedRoutes":["native-1.1","native-2.0-delivery-disabled","directory-2.0"]}
```

That is the receipt's `consumer` object. Its `ravel` object holds the tag
commit and the three asset digests of the cut's evidence. The scenario
turned Homebrew's developer mode on, and ran this command in `WORK/ravel`
with the scenario's `brew` wrapper first on `PATH`:

```sh
RAVEL_TAP_DIR=WORK/tap-promote task release:promote VERSION=2.0.0 \
  READINESS=WORK/promotion/v2.0.0.readiness.json EVIDENCE=WORK/promotion/v2.0.0.json
```

The task exited 0, and its steps gave these results:

1. It verified the public tag, the three asset digests and the book of
   v2.0.0 against the cut's evidence, through GitHub.
2. It rendered both classes from the downloaded assets and passed the
   Homebrew test of `release-formula.sh --test`. Homebrew loaded the
   promoted `ravel@2` as deprecated in favour of `ravel`.
3. It made one jj commit in the tap clone, `38a63bbf3e1e`,
   `ravel v2.0.0 (promote ravel@2 v2.0.0)`. It pushed that commit to the
   scratch `origin`. That commit is state D.
4. `release-install.sh` ran `brew update`, which updated only the tap.
   Homebrew's commit stayed at `a6fea6bd59bb`. `brew upgrade ravel`
   upgraded Ravel from 1.1.0 to 2.0.0. The keg check found every earlier
   keg unchanged, and the installed check verified the 2.0.0 keg.
5. It wrote its promotion evidence in `WORK/ravel/target/release-evidence`.

State D changes only `Formula/ravel.rb` and `Formula/ravel@2.rb` on state
C. `ravel.rb` is the unversioned class at version 2.0.0 with no `keg_only`
line. `ravel@2.rb` gains one line:

```ruby
  deprecate! date: "2026-11-11", because: :versioned_formula, replacement_formula: "ravel"
```

The tag commit's time is `2026-11-12T20:58:47+11:00`. At UTC−12 that is
2026-11-11, the rendered date. The promotion evidence records the same
`deprecationDate`, the tap commit, and the formula digests `675cc0322036…`
for `ravel.rb` and `884612f38258…` for `ravel@2.rb`. Its retained kegs are
the 1.1.0 keg and the `ravel@2` 2.0.0 keg.

`release-promote.sh` and `release-formula.sh` made their temporary
directories under the owner's system temporary directory, not under
`WORK`. Their exit traps removed both, and both were absent after the run.

### Controls of the state D checks

After the run, the instrument's functions ran against subjects that are
known to be wrong. Each check failed. The same diff check accepted the real
diff of the run.

| Subject | Check | Result |
| --- | --- | --- |
| A prefix whose default `ravel` is 1.1.0, with a HOME that holds only `policy.ts` | Retired format | Fails: the default ravel did not refuse as documented |
| A HOME that Ravel 2.0.0 accepts, launched as a refusal | Refused launch | Fails: the launch did not show the refusal |
| The upgrade-a-d prefix, which holds a 1.1.0 keg | Real front only | Fails: the prefix holds a 1.1.0 keg |
| The fresh-d sample with `{"skills": {"schemaVersion": 1, "sources": []}}` | Sample launch | Fails: no descriptor |
| The upgrade-c-d prefix with its tap back at state C | Deprecated `ravel@2` | Fails: `deprecated` is false |
| The run's `ravel@2.rb` diff with one more added line | Promotion diff | Fails |
| The same diff with `replacement_formula: "ravel@3"` | Promotion diff | Fails |

An earlier form of the sample control wrote a top-level `"sources": []`.
Ravel 2.0.0 refused that file as `settings_invalid`, an unknown key
`sources`. Its remedy lists `skills` among the permitted keys. That launch
failed before the delivery check, so the table holds the corrected form.
"Observations for the decision" below states the consequence.

### Results

| Scenario | Commands after tapping and trust | Result |
| --- | --- | --- |
| upgrade-c-d | At C, `brew install linkuistics/taps/grove` and `brew install linkuistics/taps/ravel@2`. Then `task release:promote`, which upgrades Ravel | At C, the pair of fresh-c. After the promotion, `bin/ravel` links `Cellar/ravel/2.0.0`, and Grove stays at 22.9.0. The tests of `ravel`, `grove` and `ravel@2` pass. `ravel@2` stays keg-only and unlinked at 2.0.0, and Homebrew reports it deprecated since 2026-11-11 in favour of `ravel`. The Cellar holds Ravel 2.0.0 and 1.1.0. |
| fresh-d | `brew install linkuistics/taps/grove`, then `brew install linkuistics/taps/ravel@2` | Grove 22.9.0 installed with Ravel 2.0.0 as its dependency. The tests of `ravel` and `grove` pass. `bin/ravel` links 2.0.0. The `config.rhai` sample launch works. `ravel@2` still installs, keg-only and unlinked, with Homebrew's deprecation warning. Its test passes. |
| upgrade-a-d | At A, `brew install linkuistics/taps/grove`. Then the tap moves to D. Then `brew outdated --verbose`, `brew upgrade linkuistics/taps/ravel`, `brew outdated --verbose` and `brew upgrade` | At A, the pair of fresh-a. At D, `brew outdated` lists Grove and Ravel. The Ravel-only upgrade upgrades Ravel alone and leaves Grove 22.8.0 linked. `brew upgrade` then upgrades Grove. After that, the pair is that of upgrade-c-d without `ravel@2`. The Cellar keeps Grove 22.8.0 and Ravel 1.1.0. |

At D, both upgrades gave the same readings for a HOME that holds only
`policy.ts`:

- A direct `ravel start-session --kind impl --prompt probe` through the
  default `ravel` exited 3 with this refusal:

  ```text
  ravel: refused (config_format_retired, stage authority): the selected configuration entry WORK/upgrade-a-d/retired-home/.config/ravel/policy.ts is TypeScript or JavaScript, which this release no longer evaluates
    source: WORK/upgrade-a-d/retired-home/.config/ravel/policy.ts
    remedy: write config.rhai using chapter 4 of the book, "Write a configuration script", and name a .rhai file with --config PATH if it is elsewhere
  ```

- A Grove launch printed the same refusal and remedy. It then printed
  `grove: session ended without a supported Grove result — status exit
  status: 3` and the advice to rerun `grove`. Grove exited 0, and the leaf
  stayed live. The agent did not run.
- The conversion wrote `config.rhai` with the same route and kept
  `policy.ts`. The next Grove launch through the default `ravel` printed
  `grove finished — loop complete.` The agent ran Ravel 2.0.0 and received
  `RAVEL_SKILL_DIRECTORIES` and a prepared prompt that names its
  descriptor. `ravel skills resolve --json` resolved the six sources from
  `Cellar/grove/22.9.0/libexec/skill-sources`. The HOME held no
  `settings.json`.

In fresh-d, the fixture HOME held only Grove's `config.rhai` sample, copied
from `opt/grove/libexec/ravel-policy/sample/config.rhai`. The sample's
default, `claude-led codex-sol`, routed `impl` to the fake `claude` with
`--add-dir` for each skill snapshot, `--model claude-opus-5-5` and
`--effort high`. The fake `claude` received a prepared prompt and a
descriptor, and Grove printed `grove finished — loop complete.`

After each package mutation, the real 1.1.0 front reported `ravel 1.1.0`. It
refused `end-session grove-continue` with exit 1 and no output:
`ravel: end-session: no enclosing Ravel session (RAVEL_EXIT_FILE is unset or
empty)`. The 1.1.0 front of the scenario's prefix gave the same readings in
upgrade-c-d and upgrade-a-d. The fresh-d prefix has no 1.1.0 keg.

### Retained kegs

No upgrade changed any file of a retained keg. That also holds for
`INSTALL_RECEIPT.json`, which the check compares separately:

- In upgrade-c-d, `release-install.sh` found the 1.1.0 keg and the
  `ravel@2` 2.0.0 keg unchanged by its own manifest. The instrument's
  manifests before and after also agree for both kegs. Homebrew left
  `INSTALL_RECEIPT.json` unchanged in both.
- In upgrade-a-d, the manifests of the 1.1.0 keg before the Ravel-only
  upgrade and after the full upgrade agree. Homebrew left
  `INSTALL_RECEIPT.json` unchanged.
- The upgrades of states B and C gave the same readings.

The protected subjects had the same digest before and after the run:
`dda4bec07d76…`. This digest differs from those of the sections above,
because the subjects now include Ravel's `main` and its
`target/release-evidence`. The run changed nothing in the real prefix, the
real tap, its remote, the owner's HOME, this repository's version files or
Ravel's workspace.

### Homebrew output of the Ravel-only upgrade

This is upgrade-a-d, with the private prefix written as `WORK/…`, without
the download lines and Homebrew's warning about a prefix that is not
`/opt/homebrew`:

```text
$ git -C $(brew --repository linkuistics/taps) pull --ff-only
tap HEAD: 38a63bbf3e1e4a253ba416fbbabd946b40b9735d
$ brew outdated --verbose
linkuistics/taps/grove (22.8.0) < 22.9.0
linkuistics/taps/ravel (1.1.0) < 2.0.0
[exit 0]
$ brew upgrade linkuistics/taps/ravel
==> Trusted formula linkuistics/taps/ravel
✔︎ Formula ravel (2.0.0)
==> Upgrading linkuistics/taps/ravel
  1.1.0 -> 2.0.0
🍺  WORK/upgrade-a-d/brew/Cellar/ravel/2.0.0: 11 files, 7.7MB, built in 1 second
[exit 0]
$ brew outdated --verbose
linkuistics/taps/grove (22.8.0) < 22.9.0
[exit 0]
$ brew upgrade
==> Would upgrade 1 outdated package
linkuistics/taps/grove 22.8.0 -> 22.9.0
✔︎ Formula grove (22.9.0)
==> Upgrading linkuistics/taps/grove
  22.8.0 -> 22.9.0
🍺  WORK/upgrade-a-d/brew/Cellar/grove/22.9.0: 140 files, 4.9MB, built in 1 second
[exit 0]
```

Homebrew left the outdated Grove because it has no bottle.
`Library/Homebrew/upgrade.rb` at the owner's HEAD selects outdated runtime
dependents, then keeps only those where the dependent and each of its
dependencies are bottled. It does so to avoid a source build. Grove and
Ravel install from archives with no bottle, so a Ravel-only upgrade never
upgrades Grove.

### Grove 22.8.0 beside Ravel 2.0.0

After the run, Grove 22.8.0 from the retained keg of upgrade-a-d ran with
that prefix's `bin` on `PATH`, where the default `ravel` is 2.0.0. It ran
once in a HOME that holds only `policy.ts`, and once with `config.rhai`
beside it. Both runs gave the same result:

```text
grove: updated bundled Codex skills in WINDOW/home-retired/.agents/skills
Error: WORK/upgrade-a-d/brew/Cellar/ravel/2.0.0/bin/ravel must report a stable Ravel 1.x CLI version from 1.1.0. Run `brew upgrade ravel` and select that installation on PATH. Received status exit status: 0 and version "ravel 2.0.0\n".
[exit 1]
```

Grove 22.8.0 refuses Ravel 2.0.0 before any launch, and the leaf stays live.
The pair fails closed. Before the refusal, Grove 22.8.0 still writes its
native Codex skills into the HOME, as every 22.8 launch does. The remedy in
the message is wrong for this state, because `brew upgrade ravel` made the
pair. The released 22.8.0 binary cannot change.

### Observations for the decision

- Every rehearsed route to D ends with a usable pair: the promotion from C,
  a fresh install, and the upgrade from A after `brew upgrade`. Each
  upgrade shows the documented refusal and remedy for a lone `policy.ts`,
  and a working launch after the conversion.
- `docs/specs/coordinated-release.md` says that Homebrew normally updates
  outdated dependents. That is false for these formulae. A Ravel-only
  upgrade leaves Grove 22.8.0, which refuses Ravel 2.0.0 and names the
  wrong remedy. The spec requires a published remedy before promotion:
  upgrade Grove first. `brew upgrade` with no argument also upgrades both.
  promotion-upgrade-remedy-docs-k338 owns the correction of the spec and of
  each upgrade page.
- Grove's caveats and spec row D say that `"sources": []` in
  `settings.json` turns delivery off. Ravel 2.0.0 refuses that key at the
  top level. It accepts it in the `skills` object, as Ravel's book says.
  promotion-upgrade-remedy-docs-k338 also owns that correction.
- promotion-controls-decision-k336 runs the six promotion controls and
  makes the decision.
