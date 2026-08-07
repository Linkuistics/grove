# msrv-claim-k74

**Kind:** impl

## Goal

Make the crate's declared minimum supported Rust version true, so a `rust-version`
claim the repository repeatedly relies on stops being unverifiable.

## Context

- Raised by `session-config-review-k20 E8` as out of scope for that chain:
  the breach predates `session-config-k19` and is not a defect of it.
- `Cargo.toml:8` declares `rust-version = "1.74"`. `Cargo.lock` pins `clap`
  4.6.1 and `clap_lex` 1.1.0, both `edition = "2024"` with `rust-version =
  "1.85"`, so `rustup run 1.74 cargo check --locked --all-targets` fails before
  compiling any Grove code: "this version of Cargo is older than the `2024`
  edition".
- The claim is load-bearing in prose, not just metadata: task briefs in this
  grove instruct sessions to "preserve Rust 1.74 compatibility", and a reviewer
  cannot check that against a tree that cannot build on 1.74 at all.
- There is no CI workflow in the repository, so nothing else would catch the
  drift.

## Done when

- `rust-version` states a value the locked dependency graph can actually build,
  established by running the toolchain rather than by reading manifests — either
  by raising it to the real floor (`>= 1.85`, driven by `clap`) or by pinning
  `clap` back to an edition-2021 release and keeping 1.74.
- The chosen floor is verified with `rustup run <version> cargo check --locked
  --all-targets` on a clean registry, and the verification command is recorded
  where a later session will find it.
- Any remaining prose asserting a different MSRV — task briefs excepted, since
  they are historical — is reconciled, including `docs/` if it names a version.

## Notes

Decide the direction first: raising the floor is a one-line change that admits
what is already true, while pinning `clap` back preserves an older-toolchain
promise nothing currently tests. The second is only worth its cost if some
consumer actually builds on 1.74; no evidence of one was found.
