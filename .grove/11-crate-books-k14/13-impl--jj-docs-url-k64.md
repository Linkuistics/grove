# jj-docs-url-k64

## Goal

Re-point the one Jujutsu documentation URL in production source at jj's current
documentation host, and land it as a corpus change the book contract permits.

## Context

- `crates/jj-workspace/src/refusal.rs:184` prints
  `https://jj-vcs.github.io/jj/latest/install-and-setup/` in the `NotRunnable`
  remedy, which is the URL a user is told to open when jj is not installed.
- Observed while drafting `the-gate-k56`: that host now answers `301 Moved
  Permanently` to `http://docs.jj-vcs.dev/latest/...`. The link still resolves
  through the redirect, so this is staleness rather than a broken remedy — and
  it is the only external URL the crate emits, so it breaks silently and
  user-facing on the day the redirect is retired.
- **Do not widen the scope to a link checker.** Whether the repository should
  test outbound external URLs at all is a separate question with a separate cost
  (network in CI), and nothing here settles it.

## Done when

- The URL names jj's current documentation host, verified by a `200` rather than
  a redirect at the time the change lands.
- The corpus-freeze rule in `.grove/BRIEF.md` is honoured: **one commit** carries
  the source change, every affected book ledger and page, and a green validator
  run over every book it touched. `refusal.rs` is one root of the `jj-workspace`
  book, wholly owned by chapter 6; the change is inside a string literal and
  moves no line boundary, so the affected set is the fragment whose fence quotes
  that line.
- `bash scripts/check.sh` passes.

## Notes

**Placed after every crate book deliberately.** Editing a byte of a frozen root
while a book that quotes it is being written invalidates the very ranges the
freeze protects; run this once the books that own `refusal.rs` exist and can be
re-proved in the same commit.
