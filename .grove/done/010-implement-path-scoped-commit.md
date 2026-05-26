# 010-implement-path-scoped-commit

**Kind:** work

## Goal
Land the path-scoped auto-commit for `grove install` and `grove update` per ADR-0001, including CLI flags, multi-harness combined commit, fail-loud error handling, tests, and docs.

## Context
- ADR-0001 specifies the behavior end-to-end — treat it as the spec.
- `src/install.rs::run_with_fetcher` is the integration point. The pre-flight check happens *before* materialisation; the commit step runs *after* the harness loop and *before* the existing `Mode::Update` eprintln nudge (which stays).
- `src/cli.rs::InstallArgs` is where `--no-commit` and `--message <text>` land.
- `src/repo.rs` already shells out to `git` via `Command::new("git").arg("-C").arg(repo)…` — follow that style; do not introduce `libgit2` or a new git crate.
- The install-scope path set is `harnesses.iter().map(|h| h.install_path(&repo_path)).collect::<Vec<_>>()`.

## Done when
- `grove install` and `grove update` produce one path-scoped commit by default, message per ADR.
- `--no-commit` skips the commit cleanly and prints the staging command the user would run instead.
- `--message <text>` overrides the default message.
- Pre-flight refuses if any install-scope path has pre-existing staged hunks (`git diff --cached --quiet -- <paths>` non-zero), before any materialisation.
- Multi-harness invocation produces one combined commit covering all targeted harness paths.
- A no-op materialisation (target identical to current) does not produce an empty commit; grove exits 0 with a "no changes" notice.
- Commit failure leaves the materialisation in place, exits non-zero, prints `git commit -- <paths>` as the follow-up.
- Integration tests cover: default-on commit; `--no-commit`; `--message`; unrelated dirty paths preserved; install-scope staged hunks refused; multi-harness combined commit; commit-failure path; no-op skip.
- README's "Use" section reflects the new default; `grove install --help` / `grove update --help` documents the flags.

## Notes
- Keep the commit logic in `src/install.rs` as a small private helper (`fn commit_install_scope(repo, paths, mode, version, message: Option<&str>) -> Result<()>`); promote to its own module only if it grows past ~80 lines.
- Tests can reuse the existing `Fetcher` trait pattern to inject a fake tarball; `git` itself runs against a real temp-dir repo (`tempfile::tempdir` + `git init`).
- For commit identity, do not set author overrides — use the user's git config as-is.
- For the staging command printed in `--no-commit` mode: literally `git add -- <paths>` then `git commit -m "<default message>"` — copy-pasteable.
