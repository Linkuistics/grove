# subprocess-seam-k57

## Goal

Draft `03-subprocess-seam.md`, slice `nothing-ambient`, and take the book to
`book-check --through nothing-ambient --check all`.

## Context

- Responsibilities: [`docs/specs/jj-workspace-book-structure.md`](../../../docs/specs/jj-workspace-book-structure.md),
  *3 · The subprocess seam*. Thesis: **no ambient state selects the repository.**
- One block, 81 lines: `subprocess-seam-source`, the whole of
  `crates/jj-workspace/src/jj.rs`.
- Required worked-example anchor `worked-invocation`: one invocation built end to
  end — the exact argv, the working directory, the four removed variables, and
  both failure endings (jj absent, and jj declining).
- The argued **absence** is as much of the chapter as the presence: there is no
  `JJ_*` counterpart to remove, because jj selects its repository by walking up
  from the working directory and its own variables configure the *user*, so
  stripping them would change who a commit is attributed to. State that jj
  behaviour as this chapter's premise, with a link to jj's documentation.
- `crates/jj-workspace/tests/environment.rs` is the evidence for the hygiene
  claims — cite it, do not reproduce it.

## Done when

- `03-subprocess-seam.md` exists; the block is resolved, its defer replaced,
  ledger row moved to `resolved`, fragment index rows added, navigation and
  contents updated, concept-index entries curated.
- `book-check --through nothing-ambient --check all` exits 0.
- The draft stage record's `## Provenance` names this commit.
- `cargo test --locked --workspace`, `cargo clippy` and `cargo fmt --all --check`
  pass. `scripts/check.sh` is red on `book-check` alone.

## Notes

**A `.git` beside a `.jj` is jj's business.** The chapter states once why four
`GIT_*` variables are nevertheless removed — a Git-aware child following an
inherited foreign repository is a real hazard where the backend is real — and
does not otherwise explore colocation.

## Decisions (running log)

**1 · The chapter's premise is cited to jj's own CLI documentation for the walk,
and verified by inspection for the identity variables.** The structure brief
requires the argued absence of a `JJ_*` counterpart to rest on stated jj
behaviour with a link. Two halves, and they have different evidence. The walk is
documented: `jj --help` on 0.44.0 prints, under `-R, --repository`, "By default,
Jujutsu searches for the closest `.jj/` directory in an ancestor of the current
working directory" — so repository selection is the working directory or an
explicit flag, and the same sentence is published at
`https://docs.jj-vcs.dev/latest/cli-reference/`. The other half — that jj's own
variables configure the *user* — is **not** documented on jj's configuration
page, which lists only `JJ_CONFIG`, `JJ_EDITOR` and `JJ_PAGER`; `JJ_USER`,
`JJ_EMAIL` and `JJ_TIMESTAMP` appear nowhere in it. So it was verified directly
on jj 0.44.0: a `jj commit` run with `JJ_EMAIL` set attributes the commit jj
creates to that address and not to `user.email`. The page says which half is
documented and which was measured, rather than presenting both as jj's word.
Rejected: asserting the undocumented half from the source comment, which is the
claim the chapter is supposed to adjudicate rather than repeat.

**2 · A sweep for `[env:` markers in jj's help was run and is not used as
evidence.** It found none anywhere in `jj help` or in five subcommands' help,
which would read as "no jj option is environment-backed" — but the same sweep
finds none for `JJ_CONFIG` either, which jj demonstrably honours outside clap. A
control that cannot distinguish the two answers is not a control, so the negative
is recorded here and kept off the page. The positive statement the page makes is
the documented default above, which does not depend on it.

**3 · The block is refined into ten intent-named literal children, each leading
with the blank line that separates it from its predecessor.** The specification
permits refinement and the structure brief leaves the partition to the owning
slice; the top-level ID, owner, range and line-count credit are unchanged.
Eighty-one lines would otherwise be one fence carrying seven separate arguments.
The module comment splits three ways because it makes three claims — the seam's
existence, the repository claim, and the consumer-environment claim — and
`raw_output` splits in two at line 58, between building the child and running it,
because the hygiene and the two failure endings are the chapter's two halves and
`M105`'s one-paragraph-per-literal rule is what makes carrying them in one fence
wrong.

**4 · One early-use row is added, for the three refusal constructors this page is
the first to name.** `Refusal::not_runnable`, `Refusal::command_failed` and
`Refusal::output_not_text` are owned by `no-remedy-of-its-own` and first used at
`#worked-invocation`. The structure brief's *Early uses* table is a minimum and
directs authors to add a row before introducing any additional later-owned name;
chapter 2's row covers the gate's two constructors and not these three. Rejected:
widening chapter 2's existing row, which would edit a ledger statement an earlier
slice authored and would make its declared first use false.

**5 · No in-session reviewer, and no `review-impl` leaf.** Same reasoning as
`the-gate-k56`'s decision 4, and it is the standing position for every child of
this draft range: the draft is the unedited baseline of a preregistered
measurement with five editorial stages already scheduled against it at
`pilot-measure-k26`, and a reviewer here would either pre-empt those stages'
claims or contaminate what they are diffed against. Byte-exactness is proved
mechanically by `book-check`.
