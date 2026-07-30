# retire-help-node-path-k20

**Kind:** impl

## Goal

Replace `grove retire --help`'s original-scheme node-path example with the
current stable-keyed directory grammar.

## Context

Executing generated help during `confirmation-prose-review-k16` printed
`003-session-store` as the example `<PATH>`. Current nodes are named
`NN-<slug>-k<key>/`; the old three-digit, keyless form was removed by ADR
*task-tree-scheme*. This is help text a human copies at the terminal, not tagged
history.

## Done when

- The source argument help demonstrates a current node path (or a stable handle
  if the command accepts one) and no longer teaches `003-<slug>`.
- A CLI test pins the current example or grammar so the generated help cannot
  regress silently.
- The relevant test suite passes and `grove retire --help` is executed to verify
  the rendered text.

## Notes

**What shipped.** `RetireArgs::path`'s doc comment now offers three current
forms: a `.grove/`-relative directory path (`04-session-store-k7`), the nested
case (`04-design-k7/02-store-k9`), and the stable `<slug>-k<key>` handle
(`session-store-k7`) — closing with *"Taken verbatim: nothing here parses it"*,
which is the fact that makes the handle legitimate rather than a guess.
`launch::retire` substitutes the argument straight into `{{NODE_PATH}}` in the
retire prompt and validates nothing, so what the argument accepts is exactly what
the *session* can resolve, and `tree_read::resolve` resolves node **directories**
by key, bare slug and full handle alike.

**The removed example was unusable, not merely stale — and that is measured.**
Against a planted node fixture (`.grove/04-session-store-k7/`), `grove-llm
resolve` returns the directory for all three of `session-store-k7`,
`session-store` and `7`, while `003-session-store` matches nothing. A human
copying the old `<PATH>` at a terminal handed the session a dead reference. The
control case is what turned "obsolete grammar" into a defect.

**The test pins the grammar through the real parser, not the example's
spelling.** `retire_help_teaches_the_current_node_grammar` (`tests/launch.rs`)
harvests every token in the rendered help whose leading digit run is followed by
`-` — the only shape that can be a position prefix, which keeps an incidental
number in the prose out of the candidate set — splits path segments, and runs each
through `tree_id::parse`, requiring a node. A reworded or renumbered example keeps
passing; one that drops the permanent key cannot. **Falsified by mutation**:
restoring `003-session-store` fails the test, and reverting it passes again.

Worth carrying, because it is the opposite of what the leaf's Context implies:
the original scheme's **three-digit width was never the discriminator**.
`tree_id::parse_position` is deliberately lenient on padding, so `003` parses as
position 3 — what rejects `003-session-store` is the missing terminal
`-k<digits>`. A test written against the digit width would have pinned the wrong
property and passed on any keyless two-digit name.

**A clap fact that bounds how this help can be written.** A doc comment with no
blank line collapses into a *single* help string, so `-h` and `--help` render
identically here and there is no short/long split to hide a long example behind;
clap also strips the trailing period. Length is the only lever, which is why the
three forms sit in one sentence rather than a list.

**The extent was settled by executing every help surface, not by a file list.**
Both binaries, every subcommand, dumped and scanned: the only positioned-name
tokens anywhere in generated help are this leaf's three new keyed examples. So
`grove retire --help` was the sole stale instance — the `k11` lesson (grep the
claim, not the file list) applied pre-emptively rather than after a miss.

**No `CHANGELOG.md` entry**, on a stronger basis than `k19`'s inheritance. Every
existing mention of `--help` in that file is *incidental* — help is named as a
surface some shipped change was documented on, never as a change in itself — so
there is no precedent for a help-text-only entry, and adding one would make the
"stale prose corrected against already-shipped behaviour" rule
(`stale-module-headers-k14` / `confirmation-prose-integrate-k17` / `k19`) depend
on which prose surface carried the staleness, which nothing in the preamble
supports. Verified CHANGELOG-free by `jj st`.

**A fourth-generation grep trap, and the worst one yet.** `rg -r` is
`--replace`, not GNU grep's recursive flag: `rg -rn 'task-tree-scheme §5' .`
silently emitted every match with the pattern **replaced by `n`**, so the output
looked like a clean, plausible hit list whose contents had been rewritten. Same
family as `k17`'s `rg -E`-is-`--encoding`, but a level worse — `-E` at least
errors, while `-r` succeeds and hands back fabricated lines. The habit that saves
you is the one this grove keeps re-learning: read the output for whether it looks
like what you asked for, not merely for whether it is non-empty.

**Two things externalized rather than absorbed**, both found by widening the
sweep past this leaf's own surface. `retire-no-launch-help-k21` — `grove retire
--no-launch` has no doc comment at all and renders as a blank row; it is the only
undescribed option in either binary, and writing its description first requires
deciding whether retire's dry run should run the checks `no-launch-config-check-k20`
moved above `do`'s early return. `driving-original-scheme-example-k22` —
`content/driving.md`'s worked research-leaf example cites `050-…` and bare `060`–
`090`, which is *provisioned* methodology content teaching a grammar the tool no
longer writes, and whose referents are in a deleted `.grove/` so no renumber can
reach them.
