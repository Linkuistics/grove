# homebrew-tap-trust-docs-k337

## Goal

Make the documented Homebrew installation of Grove and Ravel work for a new
user under Homebrew's tap trust.

## Context

scratch-homebrew-rehearsal-k333 found this on 2026-11-12. In a scratch
Homebrew at the owner's version, 7.0.13-83-ga6fea6b, `brew tap` did not trust
the tap. `brew install linkuistics/taps/grove` trusted only `grove` and then
refused its dependency:

```text
Error: Refusing to load formula linkuistics/taps/ravel from untrusted tap linkuistics/taps.
Run `brew trust --formula linkuistics/taps/ravel` or `brew trust linkuistics/taps` to trust it.
```

The owner does not see the refusal, because the owner's
`~/.homebrew/trust.json` already trusts `linkuistics/taps`.
`Library/Homebrew/trust.rb` at the owner's HEAD holds the rules:
`trust_fully_qualified_items!` and `raise_untrusted!`.
`docs/promotion-upgrade-rehearsal-record.md` records the refusal.

Pages that give the installation command include `README.md` (the `brew tap`
and `brew install grove` lines), `plugins/README.md`, `docs/USAGE.md`, the
`configure-grove` skill's `references/policy.md` and `docs/RELEASING.md`.
Grove's missing-Ravel message names `brew install linkuistics/taps/ravel`.
Ravel's README and book give their own installation commands.

## Done when

- Each installation route that a page documents names the trust step,
  `brew trust linkuistics/taps`, before the first install from the tap. A
  route that does not need the step says why.
- Read Homebrew's documentation for tap trust and cite it beside the step.
  If no official page exists, say so in the record.
- A new user's route, run in a scratch Homebrew with an empty trust store,
  installs a usable pair. `task promotion:rehearsal` can serve as that
  instrument.
- Ravel's pages carry the same correction in Ravel's own workspace, or a
  leaf there owns it.
