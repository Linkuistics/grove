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
  commit `f7f9b9b0bc4d`, which adds `ravel@2` 2.0.0.
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
  installed real tap and the tap remote's `main`. The last are the owner's
  `~/.homebrew` and `~/.config/ravel`. The run fails if any digest changes.

A usable pair at state A or B means four facts. The Homebrew tests of `ravel`
and `grove` pass, and `bin/ravel` links the 1.1.0 keg. With `ravel@2`
installed, its test also passes. `ravel@2` must also report 2.0.0 through
`opt/ravel@2` and have no link in the shared `bin` or `share`.

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
- Homebrew's formula test of Ravel 1.1.0 prints
  `stop attempt … Operation not permitted (os error 1)` inside Homebrew's
  sandbox and still passes. ravel-stop-eperm-k302 corrected that stop for
  2.0.
- Each private prefix prints Homebrew's warning that the prefix is not
  `/opt/homebrew`, a Tier 3 configuration. The Grove and Ravel formulae have
  no bottles, so the warning changes nothing that these formulae install.
