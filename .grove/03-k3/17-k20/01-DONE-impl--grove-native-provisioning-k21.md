# grove-native-provisioning-k21

## Goal

Remove automatic Codex snapshot provisioning while keeping native-only Grove
launches and explicit installation usable with Ravel 1.1.

## Context

Use the delivery specification and its evidence document's consumer map.
Recheck `cli::execute`, `provision`, the generated build inventory and binary
module, provisioning/lifecycle tests and package checks. Do not infer a shared
dependency is dead merely because the provisioner used it.

## Done when

- Remove the startup writer, private provisioner and embedded corpus/build
  coupling together. Retain manual installers and marketplace sources; add no
  fallback, skill presence gate or automatic legacy cleanup.
- Replace obsolete installation assertions with behavioral no-mutation checks
  covering fresh, owned and foreign entries, aliases, modes, current pointers,
  failure and an active old reader. Verify a native-only lifecycle with complete
  explicit skills and directory injection disabled against actual Ravel 1.1,
  using a managed fixture. This proves caller/lifecycle behavior, not an LLM's
  adherence to the mandate.
- Update installation/plugin guidance, architecture/glossary, relevant
  source-exact overview books/indexes and formula caveats/checks alongside code.
  Reconcile the old provisioning plan. Preserve the archive's executable/assets
  boundary; startup and package upgrade leave old installations untouched.
- Run relevant tasks (`check:skills`, affected book and installed-layout checks)
  and the full `check`/`bash scripts/check.sh` with explicit compatible Ravel
  inputs. Record exact build identities and limits on this task. Decompose if
  necessary rather than leaving code and required consumers inconsistent.

## Notes

This removes the writer and all consumers made false by its removal.
The following grove-native-cleanup-k33 supplies the optional cleanup procedure
and its disposable-layout exercise before this native increment can close.
No owner settings, shared native links or installed binaries are changed.
Mandate wording and real implicit-discovery trials have separate later work.

## Decisions (running log)

- One session delivers the whole increment. The code removal is small. The
  tests, the overview book and the documentation consumers follow it directly.
- `crates/grove/tests/native_skill_delivery.rs` replaces
  `skill_provisioning.rs`. It compares type, mode, inode, modification time,
  bytes and link target of every native entry before and after startup. The
  legacy layout holds entries the old provisioner refused, repaired or removed.
- The native-only lifecycle fixture uses the real `plugins/install.sh --force`
  into an isolated HOME. Each session derives its kind from the leaf filename,
  so mandate rewording in grove-skill-mandate-k22 does not break the fixture.
- Mutation control: with the provisioner restored from `@-`, all four new tests
  fail. Without it, all four pass against Ravel 1.1.0.
- Codex 0.166.0 (the installed cask) still reads `$CODEX_HOME/skills`
  (`codex-home-skills`) beside `~/.agents/skills` (`user-agent-skills`). Its
  bundled skill installer defaults to `$CODEX_HOME/skills`. Evidence: strings in
  `/opt/homebrew/Caskroom/codex/0.166.0/bin/codex`. The official page
  (`learn.chatgpt.com/docs/build-skills`) names only `~/.agents/skills`. The
  manual installer's `~/.codex/skills` target is therefore a valid explicit
  route for that version. Guidance names both roots without claiming more.
- Shipped guidance names native installation as the available route. The
  guidance describes Ravel directory delivery as a later Ravel feature, because no released
  Ravel implements it yet. The formula test now asserts no `.agents` tree and an
  unchanged empty `.codex` after a lifecycle run.
- The full check found three links to the removed files in the delivery
  evidence. They became plain paths, because that document records source at
  `8d45d75e`. It also found three fragment links to a non-Markdown page in
  `docs/specs/owner-skill-delivery.md`, written by
  owner-ravel-skill-delivery-k37. This session repaired them mechanically: the
  text now links the design README and names each view's anchor. The design
  content is unchanged, and owner-ravel-skill-delivery-k41 still reviews it.

## Verification record

Base `6b2ec873e3fd`. Rust 1.99.1. Ravel: the installed
`/opt/homebrew/Cellar/ravel/1.1.0/bin/ravel` (SHA-256 prefix
`c52f990b2059c8bb`), passed explicitly as `GROVE_TEST_RAVEL`.

- `task check:skills`: installer suite, conformance, and 10 of 10 tests in
  `plugin_fallback` and `native_skill_delivery`.
- `task release:archives TARGETS=aarch64-apple-darwin`, then
  `task release:formula:test` on that archive (SHA-256
  `6df89ba752e57b6b76d196efc0ef29471d0a947f7cedf4752d3ad72edc8d324b`): the real
  Homebrew install and test methods pass, including the new no-`.agents` and
  empty-`.codex` assertions. The archive lists `bin/`, `libexec/ravel-policy/`,
  `LICENSE` and `README.md` only.
- Embedded-corpus control: three skill phrases occur once each in the installed
  22.8.0 `grove` and zero times in the new release `grove`.
- `book-check` on the overview book: 7 files, 1,069 resolved lines, final.
- `GROVE_TEST_RAVEL=… mise exec bun@1.4.3 -- bash scripts/check.sh`: all 20
  principal checks pass, 1,323 nextest tests and 5 books. An earlier run without
  `mise exec bun@1.4.3` resolved Bun 1.4.4 and failed the pinned policy
  typecheck. That run also exposed the link defects above.

Limits: the lifecycle proof uses a managed fixture, not an LLM. Only the native
macOS archive was built and tested. The Linux archives and the installed smoke
were not run. No owner settings, native links or installed binaries changed.
The optional cleanup procedure belongs to grove-native-cleanup-k33.
