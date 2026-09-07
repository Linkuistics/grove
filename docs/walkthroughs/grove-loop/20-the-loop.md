# The loop
<!-- book-page id="the-loop" slice="four-things-a-runner-cannot-choose" order="20" -->
[Previous: The guaranteed core](19-the-core.md) | [Contents](README.md) | [Next: What could not move](21-what-could-not-move.md)

<a id="four-things-a-runner-cannot-choose"></a>
## The rule: all of the spawning, watching and escalating is `keyed-launch`'s

Chapter 19 read the string a session is launched with. This chapter reads the
thing that launches it, and the surprise is how little of that is here. The
module that drives grove's whole runtime is 615 lines, and it spawns nothing it
supervises, watches nothing it spawned, and kills nothing at all.

> **All of the spawning, watching and escalating is `keyed-launch`'s. What stays
> here is the four things a loop has to choose and a runner cannot:** which
> directory the channel is allocated in, which variable publishes it, which
> variables are scrubbed, and how long the two graces are.

That is not a summary of the module; it is the module's own sentence, in bold, in
its header, and it is the last of the nine module headers the book's spine is
recovered from. The four choices are the whole of what the file adds to a
launcher that already knows how to spawn a child, poll it beside a file, and
escalate. Everything else in the 615 lines exists to reach a state in which those
four values can be handed over.

The carried example reaches its last step, and the loop's own shape is the shape
of every chapter before it:

```text
  the tree is transitioned to current      chapter 14
  the walk chooses a leaf                  chapter 7 — kind `impl`, handle `plan-k1`
  the configuration resolves the kind      chapter 18
  the prompt is composed                   chapter 19

  Channel::allocate(control_dir)        -> choice 1: whose directory
  channel_var: "GROVE_SIGNAL_FILE"      -> choice 2: which variable publishes it
  scrub: LOOP_CONTROL_ENV               -> choice 3: what a child may not inherit
  escalation: 2s grace, 5s kill-grace   -> choice 4: how long the two graces are

  keyed_launch::run(Launch { .. })      -> everything else

  -> a token, or none; relaunch, stop, or interrupted
```

This is the outcome's third question one layer out from where chapter 19 asked
it. *On the way out — the policy: what does this layer choose that nothing
beneath it could have defaulted?* Chapter 18 answered with four slot **names**;
chapter 19 answered with the contents of the required slot; this chapter answers
with four **values handed to a library that has no opinion about any of them**.
The cost the outcome names — *a chosen value must be stated where a reader can
find it* — is paid here about as literally as a codebase can pay it: three of the
four are `const` declarations, and between them they carry **fifty lines of doc
comment above six lines of declaration**.

<a id="restart-is-continuation"></a>
## Restart is continuation, and a boundary is not a step

Two claims in the header do the structural work, and both are about what the loop
does **not** hold.

The first is that **restart is continuation**. A `grove` re-run from the same
working tree is not a resume mechanism; it is the same mechanism. The loop body
keeps no cursor, no iteration count and no memory of the previous session,
because position is re-derived from the tree on every pass — the transition, then
the walk, then the configuration read. That is why the header can say a crash and
a `/exit` are the same ending as any other, resumable by re-running the command:
there is no state that a restart could fail to restore, which is chapter 13's
*the tree's shape is the only state* arriving at the one module that could most
easily have broken it.

The second is that **the shell sketch is still the whole loop, because a boundary
is not a step**. The header carries a fourteen-line `while` loop in shell, and
the honest reading of it is that it remains complete after
`loop-crate-driver-k22` moved this module into `grove-loop` — the refactor
changed which *crate* owns the code, and a crate boundary is not an iteration of
the loop. The sketch's first line is the exception the header states outright:
owning the workspace lease now happens in the caller, so the sketch begins one
line before `run` does. Everything after that line is what this file does, in
this order.

Both claims are stated in the header's plain `//` comment block, which matters
more than it looks — see the next section.

<a id="what-the-instruments-see"></a>
## Both blind spots, on one root

**281 of the production half's 546 lines are comment prose, and 6 of the test
block's 69 are** — counting lines whose first non-space characters are `//`. That
is 51.5% and 8.7%, and the structure brief says 51% and 8%. The one-point
disagreement chapter 15 recorded over `verbs.rs` and `driver.rs` does not recur
here, and neither figure is given as a percentage alone: the counts are what a
later page can check. Whole, the root is 287 of 615 lines, 46.7%, which is the
figure chapter 19 forecast for it.

**This root has 204 `///` lines, no `//!` at all, 83 plain `//` lines, and an
inline `#[cfg(test)]` module.** It is therefore the one root in Part V that
carries **both** of the instrument blind spots this book has been working around,
where `session_config.rs` and `prompt.rs` carried neither:

- chapter 16's — `cargo doc` sees only `///` and `//!`, so the entire 54-line
  header is invisible to it;
- chapter 17's — `cargo doc` cannot see inside a `#[cfg(test)]` module at all, so
  the last 69 lines are invisible too.

Chapter 19 measured `prompt.rs` and found it an exception, and said explicitly
that the forecast still stood for this root. It does. The two blind spots together
hide 123 of this file's 615 lines from the instrument, and what is inside them is
not incidental: the header carries the module's whole thesis — the bold rule, the
shell sketch, *restart ≡ continuation* — and the test module carries the entirety
of the file's own evidence. The instrument reads the 492 lines in between.

So the clean run has to be earned rather than assumed. `cargo doc --no-deps
--document-private-items -p grove-loop` reports thirty warnings over the crate and
names this file in **none** of them: all eight of its intra-doc links resolve.
Three controls establish what that silence covers, and each was watched to fail:

| Control | Where the broken link was planted | Crate warnings |
|---|---|---:|
| baseline | — | 30 |
| A | inside a production `///` docblock | **31** |
| B | inside the `#[cfg(test)]` module | 30 |
| C | inside the plain `//` header | 30 |

Control A is what makes the reading a measurement: the instrument does reach this
file's `///` comments, so their silence is about links that resolve. Controls B
and C are what bound it: the same construct in the other two regions changes
nothing, so the run says nothing whatever about those 123 lines. A page that read
the clean result as *this file's citations are fine* would be reporting the reach
of the instrument rather than the state of the file — and the reach is the smaller
half of the point. The citation this chapter adjudicates sits at line 86, well
inside a `///` docblock the instrument does read; it is silent about that
citation because `cargo doc` checks intra-doc links and knows nothing of a bare
parenthesised anchor. A clean run is bounded twice over: by the lines the
instrument cannot see, and by the citation forms it does not look for.

**The block holds exactly two `#[test]` functions.** They are at lines 557 and
593, counted against the bytes rather than against any prose list of them, and
the structure brief names those two and no others. Chapter 17's list was wrong in
both directions and chapter 18's and 19's had to be counted too; this is the
fifth such count in the book and the first that needed no correction.

<a id="the-harness-widened"></a>
## The harness this chapter had to widen, and the direction the error runs

Every chapter from 11 onward has measured coverage the same way: copy the
workspace, run an unmutated control, then replace one construct and diff the
newly-failing set. The command that recipe carries is `cargo test --no-fail-fast
-p grove-loop -p grove-llm`, and **for this block it is short by a package.**

`loop_driver.rs`'s production half is barely reached from either of those two
crates. Its observers are in `crates/grove/tests/` — `loop_driver.rs` with eleven
tests, `lifecycle_cutover.rs` with seventeen, `env_hygiene.rs` with four — and
that directory belongs to the `grove` binary crate, which the promoted command
never builds. Run unchanged, it would have reported a 615-line block held by
nothing at all.

The direction matters and it is the same one the briefs keep flagging. A control
that is *too high* writes off tests that were going to fail anyway and hides them
as observers; a control taken over *too few packages* hides them by never running
them. Both read as a clean zero, and a zero is the reading this chapter's coverage
claims rest on.

So the control here is `cargo test --no-fail-fast -p grove-loop -p grove-llm -p
grove`, over a workspace copy carrying `crates/`, `.cargo/`, `testing/`,
`plugins/`, `scripts/`, `docs/`, `.claude-plugin/` and every root-level file, with
`cargo build -p grove --bins` run first. It reports **626 test runs, 616 passing
and 10 failing** — the ten being `crates/grove-loop/tests/prompt.rs`'s ten that
reach `compose` and die in the `workspace()` fixture because the copy is not a jj
repository. That is the clean ten `the-core-k167` established, and 626 is the
figure chapter 10 reported under its own wider copy.

**626 runs are 625 distinct names**, because
`finish_commit_refuses_a_handle_that_is_not_the_live_finish_leaf` is compiled into
two binaries and runs in both. Nothing in this chapter turns on it, but it is
the shape the briefs warn about — two runs can agree on a total while disagreeing
about which item did what — and it is the reason every mutant below is diffed
name by name rather than by count.

<a id="the-block-declared"></a>
## The block, declared

The 615 lines are one composite whose children are the file's own items in file
order, and the order is the argument: the header states the rule and sketches the
loop, then the four chosen values are declared **before** anything uses them,
then the outcome type, then the loop itself, then the five helpers it calls, and
last the two tests that hold the one ordering the loop cannot get wrong.

<!-- fragment «loop-driver» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="1-615" parent="source-loop-driver" -->
<!-- insert «loop-header» -->
<!-- insert «loop-imports» -->
<!-- insert «loop-worktree-name» -->
<!-- insert «loop-control-env» -->
<!-- insert «loop-channel-var» -->
<!-- insert «loop-scrub-list» -->
<!-- insert «loop-scrub-helper» -->
<!-- insert «loop-outcome» -->
<!-- insert «loop-run» -->
<!-- insert «loop-drive-open» -->
<!-- insert «loop-drive-interrupt» -->
<!-- insert «loop-drive-selection» -->
<!-- insert «loop-drive-expand» -->
<!-- insert «loop-drive-launch» -->
<!-- insert «loop-drive-discard» -->
<!-- insert «loop-drive-interrupted» -->
<!-- insert «loop-drive-endings» -->
<!-- insert «loop-session-prompt» -->
<!-- insert «loop-launch-contract» -->
<!-- insert «loop-launch-spawn» -->
<!-- insert «loop-handoff» -->
<!-- insert «loop-escalation» -->
<!-- insert «loop-reset-terminal» -->
<!-- insert «loop-ignore-interrupts» -->
<!-- insert «loop-picked» -->
<!-- insert «loop-tests-open» -->
<!-- insert «loop-test-handoff-preserves» -->
<!-- insert «loop-test-ordering» -->
<!-- /fragment -->

<a id="the-header"></a>
## The header, and the loop it draws in shell

Fifty-four lines, and they are the chapter. The module states what it is, states
what it is not, sketches the whole of itself in a language that needs none of
this crate, and then explains why the sketch is still true.

<!-- fragment «loop-header» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="1-55" parent="loop-driver" -->
````rust
// The self-driving loop — grove's runtime (self-driving-loop).
//
// Bare `grove` drives the *whole loop*, not one task: it launches a fresh
// foreground session per grove task and relaunches with fresh context each time
// the agent fires the completion signal (`grove-llm complete`). Any other exit —
// human `/exit`/Ctrl-C, or a crash — stops the loop, resumable later by
// re-running `grove` from the same working tree (restart ≡ continuation, the
// loop body holds zero state and re-derives position from the tree).
//
// The configured command is spawned directly — no shell, no PID-export trick —
// and watched while it runs: poll it alongside the completion-signal file, and
// once the file appears, apply grace → SIGTERM → kill-grace → SIGKILL to the
// child itself (driver-side watcher — self-driving-loop). The driver is the
// session's own parent process, outside whatever sandbox the session runs
// under, so it can always signal its child — unlike the in-agent self-kill this
// replaces, which codex's Seatbelt sandbox silently denied.
//
// **All of that is `crates/keyed-launch`'s, not this module's.** What stays here
// is the four things a loop has to choose and a runner cannot: which directory
// the channel is allocated in, which variable publishes it, which variables are
// scrubbed, and how long the two graces are. The shell sketch below is still the
// whole loop, because a boundary is not a step.
//
// **The entry point is [`run`], and it is handed everything it cannot derive.**
// `loop-crate-driver-k22` moved this module into `grove-loop` and left
// `crates/grove` a binary that parses an empty command line, resolves the
// workspace, takes the lease and calls in. So the sketch's first line — owning
// the workspace lease — happens in the caller, and the loop is what follows it.
//
// The driver is deliberately tiny — a plain shell `while` loop could stand in
// (constraint 6, walk-away-able). Nothing below infers anything about the
// session: the selected leaf's filename kind indexes one complete-config entry,
// and that entry's argv is the launch in full.
//
//     # after owning the workspace lease, clean abandoned signal-<128-bit> paths
//     while :; do
//       grove_recover_or_migrate_tree                    # driver-only transition
//       # One in-process selection: the leaf's stable handle *and* its kind.
//       read -r handle kind <<<"$(grove_select_or_materialize_finish)"
//       # The kind indexes the config; there is no default, family, or fallback.
//       argv=$(kdl_lookup "$HOME/.config/grove/config.kdl" "$kind")
//       # Draw a fresh OS-random 128-bit suffix in the workspace control dir;
//       # retry occupied names without touching their contents.
//       sig="$control_dir/signal-<fresh-128-bit-suffix>"
//       GROVE_SIGNAL_FILE="$sig" $argv &                 # ${prompt} carries $handle
//       pid=$!
//       # poll $pid (try_wait) and "$sig" every ~500ms; on signal appearing:
//       # sleep 2, kill -TERM $pid, sleep 5, kill -KILL $pid
//       wait "$pid"
//       stty sane 2>/dev/null
//       disposition=$(read_signal "$sig")
//       rm -f "$sig"                  # only this launch's accepted channel
//       [ -n "$disposition" ] || break # no completion signal → stop
//     done

````
<!-- /fragment -->

Four things in that block are worth reading slowly, and one of them is wrong.

**The bold sentence is the spine's ninth statement, and the strongest.** Eight
module headers before this one open on what could not move; this is the only one
that opens by naming what *did*. `crates/keyed-launch` took the spawning, the
watching and the escalating, and the header does not soften the loss — it says
*all of that*, in bold, and then lists the four remainders. The
`docs/ARCHITECTURE.md` account of the watch and the escalation is joint between
this crate and `keyed-launch` for exactly that reason, and both of its books are
now written.

**The sketch is a rebuttal to a specific objection.** A reader who has just been
told that the loop is fourteen lines of shell may reasonably ask what 615 lines
of Rust are for. The answer is in the two comments inside the sketch — *the kind
indexes the config; there is no default, family, or fallback*, and *draw a fresh
OS-random 128-bit suffix in the workspace control dir* — which are the two places
a shell stand-in would have to be told something a shell does not know. The
header's *constraint 6, walk-away-able* cites the grove spine's sixth constraint,
which is a citation form no instrument in this repository checks and which does
resolve: `plugins/grove/skills/grove/SKILL.md` numbers *Walk-away-able* sixth.

**`${prompt}` carries `$handle`, and that is the whole interface to chapter 19.**
The sketch's launch line is the only place in the file where the prompt and the
handle appear together, and it says the relationship in five characters. The
composition itself is `session_prompt`, forty lines below.

**The header's own three citations all resolve, and the file's fourth does not.**
Within these 54 lines: line 1's `(self-driving-loop)` and line 13's
`(driver-side watcher — self-driving-loop)` both name
`docs/ARCHITECTURE.md`'s *Lifecycle and resumption* section, which argues the
driver-side kill and the sandbox ground the header leans on; line 31's
`(constraint 6, walk-away-able)` names the spine constraint above. The file's
remaining citation of that shape is in the next block, and that one names
nothing at all.

<a id="what-it-imports"></a>
## Eleven imports, and the six that are a vocabulary

Eleven `use` lines, and they divide three ways: three from this crate, two from
the workspace's other crates, and six from `std`.

<!-- fragment «loop-imports» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="56-67" parent="loop-driver" -->
````rust
use crate::driver_lease::DriverLease;
use crate::session_config::{DeltaRoots, ExpansionContext, TemplateSource};
use crate::{interpret, Disposition, Handle, Kind, Reading, Selection, Sought};
use anyhow::{Context, Result};
use jj_workspace::Workspace;
use keyed_launch::{Argv, Channel, End, Ended, Escalation, Launch};
use std::ffi::OsStr;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::time::Duration;

````
<!-- /fragment -->

The `keyed_launch` line is the chapter in miniature. Six types on one line —
`Argv`, `Channel`, `End`, `Ended`, `Escalation`, `Launch` — and the module owns
the value of exactly one of them. Chapter 18's imports named the same crate
because that module *produces* an `Argv`; this one consumes the `Argv`, allocates
a `Channel`, declares an `Escalation`, fills in a `Launch`, and reads an `Ended`
carrying an `End`. Everything the file does with the runner is in those six
nouns.

`libc` is imported nowhere and reached through its crate path at three sites in
two functions — twice in `reset_terminal`, once in `ignore_interrupts`. It is the crate's only non-workspace
runtime dependency beyond `anyhow`, and `the_library_imposes_only_libc` is the
test that holds the manifest's statement of that boundary — the same test the
book's spine is pinned by.

<a id="the-grove-name"></a>
## The name a human sees, and the fallback nothing produces

The file's first function, and the smallest thing in it that is a choice.

<!-- fragment «loop-worktree-name» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="68-75" parent="loop-driver" -->
````rust
/// The grove name is the worktree directory's basename (user-owned-worktrees).
fn worktree_name(worktree: &Path) -> String {
    worktree
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| "grove".to_string())
}

````
<!-- /fragment -->

Six lines, one citation, and a fallback. The citation `(user-owned-worktrees)`
resolves — it is one of `docs/ARCHITECTURE.md`'s twenty-four anchors — and the
claim it supports is that the grove's name is the worktree directory's basename
rather than anything grove records, which is what makes a worktree the operator
renamed simply be a differently-named grove.

The `unwrap_or_else` arm is reached when `file_name()` returns `None`, which for
an absolute path means the path is a root or ends in `..`. The lease has already
proved a `.jj/` beside this directory by the time `drive` calls this, so the arm
is defensive rather than live. It is worth naming as **defensive** rather than
as *untested*: the distinction chapters 16 and 17 kept drawing between an arm no
test distinguishes and an arm no fixture could reach is the same one here, and
the value it would produce — `"grove"` — reaches nothing but a diagnostic string.

<a id="the-third-choice"></a>
## The third choice: what a child may not inherit

The longest doc comment in the file, on the shortest declaration in it. Thirty-eight
lines of argument above a three-element array, and the argument is the reason the
array has three elements rather than one.

<!-- fragment «loop-control-env» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="76-115" parent="loop-driver" -->
````rust
/// The loop driver's **launch-scoped environment** (self-driving-loop) — the
/// variables a descendant could act on, and the exact set every spawn below
/// hands to `keyed_launch` as its scrub list.
///
/// `GROVE_SIGNAL_FILE` is the completion channel: the runner watches that path
/// while its child runs and applies grace → SIGTERM → kill-grace → SIGKILL the
/// moment the file *appears*. Whoever holds the variable can therefore end the
/// session, and the environment is inherited by every descendant — so the
/// authority is ambient unless each spawn scopes it deliberately.
/// `GROVE_HARNESS_PID` / `GROVE_CLAUDE_PID` are the retired pre-watcher handles
/// (driver-side-kill), kept here because a stale, unrelated PID leaking into a
/// nested grove is the same class of mistake one notch quieter — the value is
/// something a reader could still *act on*. That is the bar for membership.
///
/// **Any spawn that is not the configured session itself must scrub this whole
/// list** (guard-loop-signal-k37), and so must the session's own, which then
/// receives the one path it owns. Scrubbing is the default and granting is the
/// exception; `keyed_launch::run` takes the list precisely so the grant cannot
/// happen without the scrub.
///
/// The failure this closes was not hypothetical. This repo is a meta-grove, so
/// its own suite runs as a *descendant* of a live session; a since-removed
/// pre-flight spawned a harness binary without scrubbing, the suite's fake
/// commands write `"$GROVE_SIGNAL_FILE"` unconditionally, and `cargo test`
/// killed the terminal it was typed into.
///
/// Grove's own spawns are exactly two — the configured session and `stty sane`
/// — and both scrub. (There were three: the build-pairing probe went with
/// provisioning at `delete-provisioning-k19`, since a driver that writes no
/// skill directory has no pairing to report.) The one
/// other family, the VCS probes and the teardown commit, went to
/// `jj-workspace`, which scrubs the *repository selectors* itself because
/// choosing the right repository is its guarantee to make. It deliberately does
/// not scrub this list: it has no consumer to speak for, and `jj` reads no
/// `GROVE_*` variable. That is narrower than *nothing downstream of it can act
/// on one* — `jj` execs a user-configured pager, editor and fsmonitor, which
/// inherit whatever `jj` inherited — and the seam's own record is where that
/// belongs rather than here.
const LOOP_CONTROL_ENV: [&str; 3] = ["GROVE_SIGNAL_FILE", "GROVE_HARNESS_PID", "GROVE_CLAUDE_PID"];

````
<!-- /fragment -->

**The membership rule is stated and then applied, which is why the list has two
entries that do nothing.** `GROVE_SIGNAL_FILE` is live: whoever holds it can end
the session, because the runner applies the escalation the moment the file it
names appears. `GROVE_HARNESS_PID` and `GROVE_CLAUDE_PID` are retired and are
kept anyway, and the comment gives the bar — *the value is something a reader
could still act on*. That is a membership test about a **reader**, not about the
current code, and it is what stops the list from shrinking every time a mechanism
is retired.

**The failure it closes is recorded as having happened.** This repository is a
meta-grove, so its own suite runs as a descendant of a live session; a spawn that
did not scrub, plus fake commands that write `"$GROVE_SIGNAL_FILE"`
unconditionally, meant `cargo test` killed the terminal it was typed into. The
comment says *was not hypothetical*, and the guard that now prevents it is two
tests in `crates/grove/tests/env_hygiene.rs` — `the_suite_cannot_reach_a_live_loop_signal_file`
and `both_guards_are_present_and_neither_subsumes_the_other`, the second of which
exists precisely because one guard hiding behind another is how the first one came
to be missed.

**The complementary half is named and disclaimed in the same paragraph.** This is
the `docs/ARCHITECTURE.md` residue subject *the scrub inside the seam, and the
loop's complementary list*, joint with `jj-workspace`, and the comment states
both halves: `jj-workspace` scrubs the repository selectors because choosing the
right repository is its guarantee, and it deliberately does **not** scrub this
list. The reasoning is the interesting part — the comment refuses the stronger
claim it could have made. *Nothing downstream of it can act on one* would be
false, because `jj` execs a user-configured pager, editor and fsmonitor which
inherit whatever `jj` inherited. What it claims instead is narrower and true: `jj`
itself reads no `GROVE_*` variable. Chapter 16's finding was that a guarantee is
worth stating as an enumeration with its exceptions rather than as a record's
slogan; this comment does that to its own neighbour's guarantee.

**The count of grove's own spawns is exact, and the file is its own evidence.**
*Grove's own spawns are exactly two — the configured session and `stty sane` —
and both scrub.* Enumerating `Command::new` and `keyed_launch::run` across this
root returns those two and nothing else: the `stty` at line 492 and the launch at
line 422. The parenthetical *(There were three)* is the same discipline applied
backwards, naming the one that left and the leaf that removed it.

**And here is the address that resolves to nothing.** The clause crediting
`GROVE_HARNESS_PID` and `GROVE_CLAUDE_PID` as *the retired pre-watcher handles*
cites `(driver-side-kill)`. The file carries **seven** parenthesised citations of
that shape, and resolving each one in turn is what isolates it: `self-driving-loop`
at lines 1, 13 and 76 and `user-owned-worktrees` at line 68 are architecture
anchors that exist; `walk-away-able` at line 31 is a spine constraint that exists;
`guard-loop-signal-k37` at line 91 is a leaf handle. That leaves line 86's, and it
is not one of
`docs/ARCHITECTURE.md`'s twenty-four anchors, not a decision-record slug, not an
id in the conformance rule inventory, and it occurs **exactly once in this
repository** — in this comment. The positive control is the sibling citation in
the same file: the same search returns twenty sites for `self-driving-loop`.

The claim is true and only the address is wrong, which is the `paths-k142`
precedent. A reader wanting the argument
should read the section `self-driving-loop` names, which states the sandbox
ground the retirement rests on. It is adjudicated here rather than corrected —
the corpus is frozen and this page reproduces the bytes as they are — and it is the fourth
*form* of broken citation the book has found, each invisible to a different
instrument: chapter 18's reference to a numbered requirement that does not exist
and its Markdown link resolving from neither surface, chapter 19's rule id with
an invented `skill-` prefix, and this one, a bare parenthesised anchor naming no
anchor. **Only
enumerating a block's tokens and resolving each one finds this class**; no
instrument in this repository looks for it, and this one sits in a `///` docblock
that `cargo doc` reads and says nothing about.

<a id="the-second-choice"></a>
## The second choice: which variable publishes the channel

The second `const`, and the one the runner is handed by name.

<!-- fragment «loop-channel-var» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="116-121" parent="loop-driver" -->
````rust
/// The variable the completion channel's path is published under — the name
/// this build and `grove-llm complete` have agreed on. It is the runner's
/// `channel_var`, and it is the first entry of [`LOOP_CONTROL_ENV`] because
/// granting it is exactly the exception scrubbing exists to carve out.
const CHANNEL_VAR: &str = "GROVE_SIGNAL_FILE";

````
<!-- /fragment -->

Five lines, and the last clause is the design. `CHANNEL_VAR` is the first entry
of `LOOP_CONTROL_ENV` *because granting it is exactly the exception scrubbing
exists to carve out* — the one variable the configured session receives is the
one every other spawn must be stripped of, and the two facts are held in one
place so they cannot drift apart. The [glossary's loop control
channel](../../../CONTEXT.md#loop-control-channel) is the term; chapter 15 read
the child's side of it, where `signal_channel` reads this same variable back.

The name is *the name this build and `grove-llm complete` have agreed on*, which
is a coupling between two binaries with no shared constant — and the comment says
so rather than implying a mechanism that is not there.

<!-- fragment «loop-scrub-list» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="122-126" parent="loop-driver" -->
````rust
/// [`LOOP_CONTROL_ENV`] as the runner takes it.
fn scrub_list() -> [&'static OsStr; LOOP_CONTROL_ENV.len()] {
    LOOP_CONTROL_ENV.map(OsStr::new)
}

````
<!-- /fragment -->

And the same list again, in the shape a `Command` takes it.

<!-- fragment «loop-scrub-helper» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="127-137" parent="loop-driver" -->
````rust
/// Deliberately one helper rather than an `env_remove` per site: the list is the
/// interesting part, and a second site open-coding it is how the first one came
/// to be missed. The configured session goes through
/// [`keyed_launch::Launch::scrub`] instead, which is the same list by the same
/// rule.
pub(crate) fn scrub_loop_control_env(cmd: &mut Command) {
    for name in LOOP_CONTROL_ENV {
        cmd.env_remove(name);
    }
}

````
<!-- /fragment -->

Two helpers over one list, in two shapes because the two consumers take it two
ways: the runner wants `&[&OsStr]` in a `Launch`, and a `Command` wants
`env_remove` per name. The doc comment's *deliberately one helper rather than an
`env_remove` per site* names the failure it prevents — *a second site open-coding
it is how the first one came to be missed* — which is the same argument the
`env_hygiene` test about neither guard subsuming the other makes from the outside.

`scrub_loop_control_env` is the only `pub(crate)` item in this file, and it has
one caller inside this module and none outside it. That asymmetry is worth naming
rather than treating as an oversight: the visibility is what lets a future spawn
elsewhere in the crate reach the list, and the header's rule — *any spawn that is
not the configured session itself must scrub this whole list* — is a rule about
the crate, not about this file.

<a id="how-a-loop-ends"></a>
## Three ways a loop ends, and why the third is not the second

The crate's one public enum outside the task-name types, and the only value `run`
returns on success.

<!-- fragment «loop-outcome» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="138-166" parent="loop-driver" -->
````rust
/// Why the loop stopped — the loop's terminal disposition, made first-class so
/// a clean whole-grove finish is distinguishable from an abnormal stop (rather
/// than both looking like "the loop just ended").
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoopOutcome {
    /// The grove finished cleanly: a session signalled `complete --done`.
    Finished,
    /// A non-signalled exit stopped the loop (human `/exit`/Ctrl-C, or a
    /// crash); resumable by re-running `grove` from the same working tree.
    ///
    /// Nothing about the *delivery* of the methodology is among the reasons.
    /// The driver used to report a mismatched, unidentifiable or missing
    /// `grove-llm` here and launch anyway; `delete-provisioning-k19` deleted
    /// both that report and the skill directory it was about, so a session's
    /// environment is now entirely the human's to keep right.
    Stopped,
    /// **The driver itself** was sent SIGTERM or SIGHUP and stopped the loop
    /// because of it — carrying which one, because whoever started the driver
    /// has to be told.
    ///
    /// Distinct from `Stopped`, which is the loop reaching a decision it was
    /// designed to reach. This is the loop being taken away mid-grove: a
    /// systemd unit restarting, a `timeout(1)` firing, a terminal closing. A
    /// caller that mapped it to a clean exit would be telling its own parent
    /// that a grove finished, so `crates/grove` ends on
    /// [`keyed_launch::reraise`] and the parent sees `128 + N` instead.
    Interrupted(i32),
}

````
<!-- /fragment -->

The type exists so that *the loop just ended* is not a thing a caller can
observe. `Finished` is a grove that completed — a session signalled `complete
--done`. `Stopped` is the loop reaching a decision it was designed to reach.
`Interrupted` is the loop being taken away.

**The third variant is the one with an argument, and the argument is why it
exists.** A systemd unit restarting, a `timeout(1)` firing, a terminal closing:
the driver did not decide anything, and *whoever started the driver has to be
told*. The comment states the consequence exactly — a caller that mapped it to a
clean exit would be telling its own parent that a grove finished — and names
where the honesty is paid out: `crates/grove` ends on `keyed_launch::reraise`, so
the parent sees `128 + N`. The signal number is carried through this type only to
be re-raised at the process boundary.

`Stopped`'s comment carries the one piece of history in the enum, and it is a
deletion: the driver used to report a mismatched or missing `grove-llm` and
launch anyway, and `delete-provisioning-k19` removed both the report and the
skill directory it described. *A session's environment is now entirely the
human's to keep right.* That is the same retirement `LOOP_CONTROL_ENV`'s two dead
variables are the residue of, and the third mention of that leaf in fifty lines.

<a id="run"></a>
## `run`: the three things a loop cannot derive

The entry point, under twenty-six lines of doc comment that are mostly about its
signature.

<!-- fragment «loop-run» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="167-201" parent="loop-driver" -->
````rust
/// **The whole loop**: `exists? → create or find next → determine the command →
/// run → finalise`, one configured foreground session per selected task, until
/// a session stops signalling (`docs/specs/module-decomposition.md`, decision
/// 9).
///
/// The three arguments are the three things a loop cannot derive for itself and
/// is therefore handed: the **workspace** it drives (resolved by its caller,
/// which had to resolve one to take the lease), the **lease** proving it is the
/// only driver in that working tree, and the **templates** naming where each
/// launch is read from.
///
/// Nothing here inspects the working tree for a harness, and nothing chooses a
/// binary: the configured argv is the whole of launch policy. Nothing here
/// delivers the methodology either — since `delete-provisioning-k19` the
/// methodology is a plugin a human installs, so the loop's first act is a
/// transition rather than a sweep over three personal skill directories.
///
/// **The lease is taken by value.** It is dropped when the loop returns, which
/// is the point at which the working tree stops being owned; a caller that kept
/// one could go on holding ownership after the loop that justified it ended.
///
/// # Errors
///
/// A configuration that does not load or does not cover the selected kind, a
/// lease that stops naming the descriptors this process owns, a tree the store
/// refuses, or a session that could not be spawned.
pub fn run(
    workspace: &Workspace,
    lease: DriverLease,
    templates: &TemplateSource,
) -> Result<LoopOutcome, crate::Error> {
    ignore_interrupts();
    Ok(drive(workspace, &lease, templates)?)
}

````
<!-- /fragment -->

**The three arguments are the argument.** A loop cannot resolve its own
workspace, cannot prove it is the only driver, and cannot know where launches are
read from — so it is handed all three, and the doc comment says so in those
terms. Each has a chapter: the workspace is `jj-workspace`'s and reaches here
resolved; the lease is chapter 16's; the template source is chapter 18's.

**The lease is taken by value, and that is a lifetime argument made in a
signature.** `DriverLease` is dropped when `run` returns, which is the moment the
working tree stops being owned. A caller holding a `&DriverLease` could go on
owning the tree after the loop that justified the ownership ended — so the
signature makes that unrepresentable rather than documenting against it. This is
the same move chapter 5 read in `Vacancy`: the type system carries the rule, and
the comment explains what the compiler is already enforcing. It is also the one
place `one-live-driver-per-working-tree` binds outside `driver_lease.rs`.

**`run` is four lines and one of them is a side effect.** `ignore_interrupts()`
runs before `drive`, outside the loop, once; everything else is delegation. The
split exists so the SIGINT policy is established before any iteration can begin,
and `drive`'s own error type can be `anyhow::Result` while `run`'s is the crate's
opaque `Error`.

The header's citation of `docs/specs/module-decomposition.md`, decision 9, holds
for what the sentence leans on it for: the loop's shape, *exists? → create or find
next → determine the command → run → finalise*, is that decision's. The record's
own code block has since drifted from the signature above it — it names the third
parameter `&Templates` where the shipped one is `&TemplateSource`, and gives
`LoopOutcome` two variants where the type above has three. The record is not
corpus and the book does not correct it; `decision-nine-loop-signature-k177` holds it.

<a id="the-loop-body"></a>
## The loop body, in eight pieces

`drive` is 142 lines and is the only loop in the crate. It is read here in the
order it runs.

<!-- fragment «loop-drive-open» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="202-225" parent="loop-driver" -->
````rust
fn drive(
    workspace: &Workspace,
    driver_lease: &DriverLease,
    templates: &TemplateSource,
) -> Result<LoopOutcome> {
    // Both taken from the resolution that already happened rather than
    // recomputed here: `main_repo` is the seam's one derivation of *the
    // repository root* and the very value `${repo}` expands to, so the delta
    // search order cannot drift from the template it selects
    // (`docs/adr/untracked-configuration-delta.md`).
    let worktree = driver_lease.worktree_root();
    let repo_path = workspace.main_repo();
    let name = worktree_name(worktree);
    let config_path = templates.personal_path();
    let delta_roots = DeltaRoots {
        worktree,
        repository: repo_path,
    };
    let repo_name = repo_path
        .file_name()
        .map(|value| value.to_string_lossy().into_owned())
        .unwrap_or_else(|| "repo".to_string());
    let session_name = format!("{repo_name}: {name} grove");

````
<!-- /fragment -->

**Everything before the loop is derived once, and the comment says why.**
`main_repo` is *the seam's one derivation of the repository root and the very
value `${repo}` expands to*, so the delta search order cannot drift from the
template it selects — one value, one derivation, cited to
`docs/adr/untracked-configuration-delta.md`. `worktree` comes off the lease
rather than off the workspace, which is the first of the six calls this file
makes into chapter 16's block.

`repo_name` repeats `worktree_name`'s shape inline rather than calling it, with a
different fallback (`"repo"` against `"grove"`). The two are not factored
together, and the reason is visible in the values: one names a grove, the other
names a repository, and `session_name` puts them on either side of a colon.

<!-- fragment «loop-drive-interrupt» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="226-242" parent="loop-driver" -->
````rust
    loop {
        // A SIGTERM or SIGHUP that arrived while no session was running has no
        // launch to be reported against, so the runner discards it rather than
        // spending it on the next child. Collecting it here is what keeps the
        // driver from going on mutating the tree and taking commits after its
        // terminal has gone.
        if let Some(signal) = keyed_launch::take_interrupt() {
            eprintln!(
                "grove: interrupted by signal {signal} between sessions — stopping the loop."
            );
            return Ok(LoopOutcome::Interrupted(signal));
        }
        driver_lease
            .revalidate()
            .context("revalidating driver lease before loop transition")?;
        let pre_transition_config = templates.load(&delta_roots)?;

````
<!-- /fragment -->

**The interrupt collected here is the one with no launch to be reported
against.** A SIGTERM or SIGHUP arriving between sessions belongs to no child, so
the runner holds it and this is where the loop spends it. The comment names the
failure it prevents, and it is a mutation failure rather than a signalling one:
*keeps the driver from going on mutating the tree and taking commits after its
terminal has gone*. The next two statements are exactly the mutation it is
guarding — a lease revalidation and a configuration load, immediately before
`transition_to_current` writes.

**And this is the first of the two configuration reads.**

<!-- fragment «loop-drive-selection» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="243-259" parent="loop-driver" -->
````rust
        crate::driver::transition_to_current(worktree)?;
        let selection = match picked(worktree)? {
            Sought::Match(selection) => selection,
            Sought::Nothing => {
                // The finish sentinel is a leaf grove writes itself, so the
                // just-in-time presence rule binds it exactly as it binds
                // `leaf-add` — before the write, not at the launch that follows
                // (`docs/adr/complete-session-configuration.md`). Asked against
                // the pre-transition load, which is the document as it stood
                // before anything was mutated.
                pre_transition_config
                    .require(Kind::finish().label())
                    .context("materializing the driver-owned finish leaf")?;
                crate::driver::materialize_finish(worktree)?
            }
        };

````
<!-- /fragment -->

<a id="the-two-reads"></a>
## Two reads, two reasons — the claim chapter 18 could not settle alone

Chapter 18 reproduces `TemplateSource`'s doc comment, which said **the loop
re-reads the configuration once per iteration** while this book was drafted, and
adjudicated it as far as that page could: the design argument is right, the count
in it is wrong, and the refutation is on this page. Here it is, against the bytes
— and the bytes are what the comment now says, because
`template-source-read-count-k86` corrected the count from this page's evidence.

`templates.load(&delta_roots)` is called **twice** in one pass of the loop body,
at line 241 and again at line 260, with `transition_to_current` and the whole of
the walk between them. They are two reads for two reasons:

| | Line | Bound to | Read *before* | Asked for |
|---:|---:|---|---|---|
| 1 | 241 | `pre_transition_config` | `transition_to_current` | whether a `finish` template exists at all |
| 2 | 260 | `config` | the launch | the selected kind's template, expanded |

**The first read is a snapshot taken deliberately early.** The finish sentinel is
a leaf grove writes itself, so — as the comment says — the just-in-time presence
rule binds it exactly as it binds `leaf-add`: *before the write, not at the launch
that follows*. If a grove has no live leaf and no `finish` template is configured,
grove must decline to write the leaf rather than write it and fail to launch it,
because `entries-are-never-removed` means a leaf written in error stays written.
Asking against the **pre-transition** document is what makes the question fair:
the answer describes the configuration as it stood before anything was mutated,
so a session cannot be blamed for a template that appeared or vanished during its
own transition. `docs/adr/complete-session-configuration.md` is the record.

**The second read is deliberately late for the mirror-image reason.** By line 260
the leaf is chosen, and the launch should expand the template from the document as
it stands *now* — which is what makes `relaunch_reloads_config_and_uses_the_new_filename_kind`
possible, and what `TemplateSource`'s own *a source rather than a snapshot*
argument is for. A session that adds a kind to `config.kdl` and then signals
relaunch is launched from the file it just wrote.

**So the count was stale and the design was not.** The type's doc comment named
both reads in the very sentence that said *once* — the just-in-time presence rule
and the document as it stands were its two clauses — so the comment described a
two-read loop and miscounted it in its own first clause. Neither chapter could
correct it while the book was being written: the corpus was frozen, and both
pages reproduce the bytes as they stand. `template-source-read-count-k86` carried
the fix once the book had landed, rewriting both adjudicating paragraphs in the
same commit as the comment, and the sentence now reads *twice per iteration* with
each clause attached to the read it belongs to. The two rows of the table above
are what it enumerates.

**And the count is the only half of this that any test can see.** Three mutations
bracket it, against the same 277-test control used later in this chapter:

| Mutation | Newly failing |
|---|---:|
| delete the presence check entirely | **1** — `a_finish_leaf_is_not_written_when_no_finish_template_resolves` |
| ask the presence rule against a **post**-transition load instead | 0 |
| let the launch reuse the **pre**-transition load instead of reading again | 0 |

So *that* the finish template's presence is checked is pinned by one test, and
*when* it is checked is pinned by nothing — in either direction. The reason is
structural rather than an oversight: the only things standing between the two
reads are `transition_to_current` and `materialize_finish`, both of which write
into `.grove/`, and `tree_lifecycle.rs` — where both of them do their work —
contains no reference to a `.kdl` file at all. Nothing the loop does between the
two calls can change what the second one reads, so the two documents are
identical unless something *outside* grove rewrites a configuration file inside
that window.

That is worth stating precisely, because it is the k157 and k158 lesson in a
place the reader would not look for it: **a zero here means the distinction is
unreachable through this path, not that the placement is untested by oversight.**
The pre-transition timing is a guarantee about a race with an external writer,
and a fixture that exercises it would have to be one — which is also why the
guarantee is argued in a record rather than in a test.

One consequence follows from reading the arms rather than the calls: on the
ordinary path — a grove with a live leaf, which is every iteration but the last —
`pre_transition_config` is loaded and then **never used**, because the only thing
that reads it is the `Sought::Nothing` arm. The loop parses the configuration
twice per iteration and discards one of the two results. That is the cost the
placement buys, and it is visible in the block above rather than inferred.

The claim and its refutation were both inside this book's corpus, and it is
worth naming what that bought. Several of this book's adjudications rest on
evidence the reader has to take on trust from another crate's page or from a
record; this one does not, and neither did
[chapter 6](06-paths.md#canonicalise-to-compare)'s canonicalisation clause,
which `task_tree.rs` refuted in the same file that asserted it, and which a leaf
of its own has since corrected. Here the reader can hold
`18-which-files.md`'s fragment and this page's fragment side by side and count
the calls — which is also why this defect
could be fixed without leaving the book, and the outcome's third question — *what does this layer choose that
nothing beneath it could have defaulted* — is exactly what the two reads answer
differently. Neither read is a default; each is a choice about **when** to ask.

<a id="the-selection"></a>
## What `Sought::Nothing` means here

The `match` above turns *no live leaf* into a leaf rather than into an ending,
and that is the only place in the crate where grove writes a task file nobody
asked for. `materialize_finish` is chapter 14's, reached here through
`crate::driver` — the module chapter 15 read whose two operations are *not* verbs
precisely because a session may not call them.

**This loop body is the only production call site of either of them**, which is
what the enumeration shows rather than what the module's visibility implies.
`crate::driver::transition_to_current` is called at line 243 and nowhere else in
the workspace; `crate::driver::materialize_finish` is called at line 256 and,
beyond that, only from `crates/grove-loop/tests/verbs.rs`, which is evidence and
not a root. The count needs the module prefix to be true: `tree_lifecycle`'s
functions of the same two names carry twelve further call sites between them, all
inline tests in that file, and a sweep for the bare names would return them and
read as though the operations had many callers. Chapter 15 recorded the
`transition_to_current` half of this; the pair is enumerated here because this is
the page where both are reached.

<!-- fragment «loop-drive-expand» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="260-279" parent="loop-driver" -->
````rust
        let config = templates.load(&delta_roots)?;
        // The file this kind actually resolved from — the personal file, or the
        // delta that overrode it. Every diagnostic below names *that*, because
        // naming the personal file for a delta-supplied kind points a reader at
        // a file which never held the failing template.
        let resolved_source = config
            .source(selection.kind.label())
            .unwrap_or(config_path.as_path())
            .to_path_buf();
        let prompt = session_prompt(&selection.handle, &selection.kind, workspace);
        let argv = config.expand(
            selection.kind.label(),
            &ExpansionContext {
                prompt: &prompt,
                session_name: &session_name,
                worktree,
                repository: repo_path,
            },
        )?;

````
<!-- /fragment -->

**`resolved_source` exists for the error message, and the comment argues the
case.** *Naming the personal file for a delta-supplied kind points a reader at a
file which never held the failing template* — so every diagnostic below names the
file the kind actually resolved from. The `unwrap_or` fallback to `config_path`
is what happens when `source()` cannot attribute the kind, which is the case
where the personal file genuinely is the answer. `docs/adr/untracked-configuration-delta.md`
is the record, cited here and at `launch_configured_session` for the same reason.

**The kind is passed, never re-read**, and the chain is visible in this block:
`selection.kind` indexes `config.source`, indexes `config.expand`, and is handed
to `session_prompt`. One guarded selection reaches four consumers, so the prompt
and the command a session receives cannot disagree about what kind it is. That is
`session_prompt`'s doc comment's claim, and this block is where it is true.

<!-- fragment «loop-drive-launch» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="280-307" parent="loop-driver" -->
````rust
        driver_lease
            .revalidate()
            .context("revalidating driver lease before foreground launch")?;
        let channel = Channel::allocate(driver_lease.control_dir())
            .context("allocating a fresh foreground-session signal channel")?;
        let ended = launch_configured_session(
            &argv,
            &selection,
            &resolved_source,
            worktree,
            &channel,
            driver_lease,
        );
        // Unconditionally, and before the invalidation gate below: the session
        // may have left the terminal in raw mode and on the alternate screen,
        // and an error path that returns without restoring it hands the human
        // an unusable shell to read the error in. Restoring is not
        // interpretation, so it is not what the gate is protecting.
        reset_terminal();
        let (ended, signal) = complete_post_reap_epoch_handoff(
            ended,
            || driver_lease.invalidate_session_epoch(),
            |ended: Ended| {
                let signal = interpret(ended.token.as_ref());
                (ended, signal)
            },
        )?;

````
<!-- /fragment -->

**The four chosen values meet here, and three of them are `const`s declared two
hundred lines above.** `Channel::allocate(driver_lease.control_dir())` is the
first choice — the directory — and it is the lease's directory rather than a
temporary one, which is what makes an abandoned channel a thing the *next*
driver can find and clean. The other three arrive inside
`launch_configured_session`.

**`reset_terminal` runs unconditionally and before the gate**, and the comment is
explicit that this is not tidiness: a session may have left the terminal in raw
mode and on the alternate screen, and an error path that returns without
restoring hands the human an unusable shell *to read the error in*. The
justification for putting it ahead of the invalidation gate is the sharper half —
*restoring is not interpretation, so it is not what the gate is protecting* —
which states the gate's scope rather than carving an exception out of it.

**The handoff is a two-argument closure call and the ordering is the point.**
`complete_post_reap_epoch_handoff` takes the launch result, a closure that
invalidates the epoch, and a closure that interprets the token. Passing
interpretation as a *closure* rather than calling it inline is what makes the
ordering a property of the function rather than of this call site — and it is
what the two inline tests can then assert without a session, a channel or a
child. That is the whole reason this block is shaped the way it is, and the tests
at the end of the file are its proof.

<!-- fragment «loop-drive-discard» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="308-313" parent="loop-driver" -->
````rust
        if let Err(error) = channel.discard() {
            eprintln!(
                "grove: warning: could not remove the interpreted foreground-session signal channel; preserving the session outcome: {error}"
            );
        }

````
<!-- /fragment -->

**The channel is discarded, and a failure to discard it is a warning rather than
an error.** The message says why in its own words — *preserving the session
outcome* — and the ordering is the argument: the token has already been
interpreted, so a leftover file cannot change what this iteration decided. Losing
a completed grove's `Finished` because a file could not be unlinked would be a
worse failure than the leftover. `a_signal_removal_failure_does_not_override_a_done_disposition`
is the test that holds it.

<!-- fragment «loop-drive-interrupted» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="314-318" parent="loop-driver" -->
````rust
        if let End::Interrupted { signal } = ended.end {
            eprintln!("grove: interrupted by signal {signal} — stopping the loop.");
            return Ok(LoopOutcome::Interrupted(signal));
        }

````
<!-- /fragment -->

And the last statement in the loop body is the one that decides whether there is
another iteration.

<!-- fragment «loop-drive-endings» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="319-344" parent="loop-driver" -->
````rust
        match signal {
            Some(Disposition::Relaunch) => continue,
            Some(Disposition::Done) => {
                eprintln!("grove: grove finished — loop complete.");
                return Ok(LoopOutcome::Finished);
            }
            None => {
                eprintln!(
                    "grove: session ended without a completion signal — status {}, elapsed {:.3}s; loop stopped.",
                    ended.status,
                    ended.elapsed.as_secs_f64()
                );
                if !ended.status.success() {
                    eprintln!(
                        "       configured session kind `{}` failed via {:?} from {}.",
                        selection.kind.label(),
                        argv.program(),
                        resolved_source.display()
                    );
                }
                return Ok(LoopOutcome::Stopped);
            }
        }
    }
}

````
<!-- /fragment -->

**Three endings, and the diagnostics are the durable record.** `Relaunch`
continues the loop with no message at all — the next iteration's launch line is
the only trace, which is why that line names the stable handle. `Done` prints and
returns `Finished`. `None` — a session that ended without signalling — prints the
status and elapsed time, and then prints a *second* line only if the exit was
non-zero, naming the kind, the program and the file the template came from.

That second line is the whole of what grove adds to a spawn failure. The runner's
own message names the program and says to check that it is executable; the two
things only grove knows are the **kind** and the **file that supplied it**, and
`spawn_failure_names_the_kind_executable_and_config_without_retiring_the_leaf`
holds both halves — including the half in its own name, that a leaf whose session
failed to launch is *not* retired.

The `End::Interrupted` arm above it is checked before the token is consulted at
all. A driver that was signalled while its child ran has an ending that is not
about the child's disposition, and reading a token in that case would let a
session that happened to signal turn a `timeout(1)` into a clean finish.

<a id="session-prompt"></a>
## `session_prompt`: a pointer, not the methodology

The first of the five helpers `drive` calls, and the one that reaches chapter 19.

<!-- fragment «loop-session-prompt» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="345-377" parent="loop-driver" -->
````rust
/// The whole `${prompt}`: the guaranteed core, composed for the launched kind.
///
/// **The driver hands a session a pointer, not the methodology.** What earns a
/// place, and why the facts below arrive as bare values with no normative tail,
/// is [`crate::prompt`]'s to state and this function's to supply: the selected
/// leaf's stable handle, the resolved workspace the prompt states the version
/// control from, and grove's own published release version.
///
/// The kind is passed in rather than re-read: it is the same value that indexed
/// the configuration entry, taken from the one guarded selection, so the prompt
/// and the command a session receives cannot disagree about what kind it is.
///
/// **The version is `CARGO_PKG_VERSION`, read here rather than in `prompt`**, so
/// composition takes a value like every other runtime fact and the module that
/// composes text does not also decide what build it is part of. It is the same
/// value `grove --version` renders — clap derives that from this constant — which
/// is what makes the flag a fallback for the published fact rather than a second
/// source of it (`docs/specs/module-decomposition.md`, decision 10).
///
/// **Infallible**, and that is what taking the workspace as an argument bought.
/// This used to resolve one, which could fail in a driver that had already
/// proved a marker existed by leasing the `.jj/` beside it — an unreachable
/// error arm the loop still had to carry. The loop is handed the workspace it
/// is driving, so there is nothing left here to fail.
fn session_prompt(handle: &Handle, kind: &Kind, workspace: &Workspace) -> String {
    crate::prompt::compose(&crate::prompt::Mandate {
        handle,
        kind,
        workspace,
        version: crate::VERSION,
    })
}

````
<!-- /fragment -->

Eight lines of code under twenty-four of argument, and chapter 19 is the other
half of it. Three things are decided here rather than there.

**The kind is passed rather than re-read.** *The same value that indexed the
configuration entry, taken from the one guarded selection* — so the prompt and the
command cannot disagree about what kind the session is. That is a claim about
`drive`'s block above, and it is checkable there: `selection.kind` reaches
`config.source`, `config.expand` and this call, and nothing re-derives it.
`insertion_during_launch_does_not_change_the_session_mandate` is the test that
holds the selection stable across a concurrent tree mutation.

**The version is read here rather than in `prompt`.** The reason given is a
separation-of-concerns one with a consequence: *composition takes a value like
every other runtime fact and the module that composes text does not also decide
what build it is part of*. `crate::VERSION` is `CARGO_PKG_VERSION`, the same
constant clap derives `grove --version` from, which is what makes the flag a
fallback for the published fact rather than a second source of it. Decision 10 of
`docs/specs/module-decomposition.md` is the record, and it says exactly that —
*the version-flag output of the verb binary remains as a fallback, not as the
mechanism*. The pin is
`the_prompt_publishes_the_release_version_the_version_flag_renders`.

**It is infallible, and the comment says what bought that.** This function used to
resolve a workspace, which could fail — in a driver that had already proved a
marker existed by leasing the `.jj/` beside it. That is an unreachable error arm
the loop still had to carry, and taking the workspace as an argument to `run`
deleted it. The [stated VCS](../../../CONTEXT.md#stated-vcs) the prompt publishes
is therefore a value the driver resolved once, at the top, and never re-derives —
which is the `docs/ARCHITECTURE.md` residue subject *the stated VCS in
`${prompt}`* seen from the supplying side, chapter 19 having read the composing
side.

<a id="launch"></a>
## `launch_configured_session`: where the other three choices are handed over

Twenty-five lines of contract, then a signature and a single diagnostic line.

<!-- fragment «loop-launch-contract» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="378-417" parent="loop-driver" -->
````rust
/// Launch one fresh foreground session owning the real TTY, and hand it to
/// `keyed_launch::run`, which spawns it directly — no shell — and supervises it
/// until it ends.
///
/// The argv is taken whole from the expanded configuration. Nothing is appended,
/// injected, or reordered here: no session-name argument, no model flag, no
/// sandbox grant. A target that needs any of those spells them out in its own
/// command template, where the configuration owner can see them.
///
/// Prints one diagnostic line naming the kind, the executable and the selected
/// handle. That line is the only durable record of what each session in a loop
/// was working on, so it names the **stable handle** rather than a path, which
/// moves under `leaf-insert`.
///
/// A spawn failure names `resolved_source` — the file this kind's template was
/// actually read from, personal or delta — rather than the personal path
/// unconditionally, which would name a file that never held the failing
/// template (`docs/adr/untracked-configuration-delta.md`). The runner's own
/// message names the program and says to check that it is executable; grove
/// adds the two things only grove knows, the kind and the file that supplied
/// it.
///
/// The epoch is activated **before** the spawn and never after: a child that is
/// already running under an inactive epoch would have its own `grove-llm` verbs
/// refused.
fn launch_configured_session(
    argv: &Argv,
    selection: &Selection,
    resolved_source: &Path,
    worktree: &Path,
    channel: &Channel,
    driver_lease: &DriverLease,
) -> Result<Ended> {
    eprintln!(
        "grove: launching {} with configured {:?} — {}",
        selection.kind.label(),
        argv.program(),
        selection.handle
    );

````
<!-- /fragment -->

**The strongest sentence in the file is a list of things that do not happen.**
*Nothing is appended, injected, or reordered here: no session-name argument, no
model flag, no sandbox grant.* A target needing any of those spells them out in
its own template *where the configuration owner can see them* — which is the
outcome's third cost stated as a prohibition rather than as a value. Grove chooses
four things and refuses to choose a fifth, and
`bare_grove_launches_the_selected_filename_kind_with_one_mandate_argument` is what
holds the refusal: one argument, and it is the mandate.

**The diagnostic names the handle because paths move.** *That line is the only
durable record of what each session in a loop was working on* — and a path moves
under `leaf-insert`, while the handle does not. This is chapter 3's
handle-not-position rule arriving at the one place where a human reads it back,
and it is why the loop's log is legible after a tree has been reordered under it.

<!-- fragment «loop-launch-spawn» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="418-439" parent="loop-driver" -->
````rust
    driver_lease
        .activate_session_epoch(channel.path())
        .context("activating the foreground session epoch before spawn")?;

    keyed_launch::run(Launch {
        argv,
        channel,
        channel_var: CHANNEL_VAR,
        scrub: &scrub_list(),
        cwd: Some(worktree),
        escalation: ESCALATION,
    })
    .with_context(|| {
        format!(
            "launching configured session kind `{}` via {:?} from {}",
            selection.kind.label(),
            argv.program(),
            resolved_source.display()
        )
    })
}

````
<!-- /fragment -->

**The epoch is activated before the spawn and never after, and the reason is a
child that would refuse itself.** A session running under an inactive epoch has
its own `grove-llm` verbs refused — chapter 17's admission ladder is what does the
refusing — so an activation that raced the spawn would produce a session that
cannot mutate the tree it was launched to mutate. The pin is
`the_driver_activates_immediately_before_spawn_and_invalidates_after_reap`, which
holds both ends of the window in one test, and
`a_session_mutates_the_tree_through_grove_llm_without_deadlocking_the_driver` is
what proves the window is usable rather than merely open.

**And here are the other three choices, in four fields.** `channel_var` is
`CHANNEL_VAR`; `scrub` is `scrub_list()`; `escalation` is `ESCALATION`; `cwd` is
the worktree. The `Launch` struct is the whole interface to the runner, and the
file's four `const`-and-helper declarations exist to fill in three of its fields.
`keyed_launch::run` takes the scrub list *precisely so the grant cannot happen
without the scrub* — the one spawn allowed to publish the channel cannot be
written without first removing whatever it inherited.

That is the `docs/ARCHITECTURE.md` residue subject *the watch and the escalation*,
joint with `keyed-launch`: everything after this call — the direct spawn, the
poll, the grace, the SIGTERM, the kill-grace, the SIGKILL — is that crate's, and
this book says what grove asked for and stops. `the-launched-child-is-a-job` is
the record behind the process-group half of it, named here and cited nowhere.

<a id="the-handoff"></a>
## The handoff: the one ordering the loop cannot get wrong

The only function in the file with no doc comment at all.

<!-- fragment «loop-handoff» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="440-457" parent="loop-driver" -->
````rust
fn complete_post_reap_epoch_handoff<E, T>(
    ended: Result<E>,
    invalidate: impl FnOnce() -> Result<()>,
    continue_after_invalidation: impl FnOnce(E) -> T,
) -> Result<T> {
    const INVALIDATION_CONTEXT: &str =
        "post-reap session epoch invalidation blocked; completion signal left unconsumed";

    match (ended, invalidate()) {
        (Ok(ended), Ok(())) => Ok(continue_after_invalidation(ended)),
        (Err(launch_error), Ok(())) => Err(launch_error),
        (Ok(_), Err(invalidation_error)) => Err(invalidation_error.context(INVALIDATION_CONTEXT)),
        (Err(launch_error), Err(invalidation_error)) => Err(invalidation_error.context(format!(
            "{INVALIDATION_CONTEXT}; foreground session also failed: {launch_error:#}"
        ))),
    }
}

````
<!-- /fragment -->

Seventeen lines, no doc comment at all, and it is the only function in the file
the inline tests touch. The absence of a doc comment is not an omission — the
four-arm `match` **is** the statement, and the two tests below say what it means.

The rule it enforces: **signal interpretation may not run until epoch
invalidation has succeeded.** `invalidate()` is called eagerly in the match
scrutinee, so it always runs; `continue_after_invalidation` is a closure that runs
in exactly one of the four arms. The arms and their reasons:

| `ended` | `invalidate()` | Result |
|---|---|---|
| `Ok` | `Ok` | interpret the token — the only arm that does |
| `Err` | `Ok` | the launch error, unchanged |
| `Ok` | `Err` | the invalidation error, wrapped in `INVALIDATION_CONTEXT` |
| `Err` | `Err` | the invalidation error, carrying the launch error's text |

**The asymmetry in the last two arms is the design.** When both fail, the
*invalidation* error is the one returned and the launch error is folded into its
message with `{launch_error:#}`. That ordering is deliberate: a launch that failed
is a session that did not run, while an epoch that would not invalidate is a
working tree left in a state the next iteration cannot enter. The second is the
condition an operator has to act on, so it is the one that survives as the error's
identity — and the first is not lost, which is what the third arm of the second
test below checks.

`INVALIDATION_CONTEXT` says what the consequence is rather than what failed:
*completion signal left unconsumed*. A signal left unconsumed is a channel file
still on disk with a token in it, which the next driver will find.

<a id="the-fourth-choice"></a>
## The fourth choice: how long the two graces are

Two durations, and eight lines saying where each number came from.

<!-- fragment «loop-escalation» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="458-470" parent="loop-driver" -->
````rust
/// The kill escalation the runner applies once the completion channel appears.
///
/// Built-in constants, not knobs. Two seconds lets the agent's `complete` tool
/// call return and its turn end before its session dies; five more is time for
/// an orderly SIGTERM shutdown before SIGKILL. Why an escalation is needed at
/// all — an interactive session is never reaped on its own, and cannot be
/// trusted to end itself under every sandbox — is `keyed_launch::Escalation`'s
/// to state, and it states it.
const ESCALATION: Escalation = Escalation {
    grace: Duration::from_secs(2),
    kill_grace: Duration::from_secs(5),
};

````
<!-- /fragment -->

**Built-in constants, not knobs** — and the comment gives a reason per value
rather than a policy for both. Two seconds lets the agent's `complete` tool call
return and its turn end before its session dies; five more is time for an orderly
SIGTERM shutdown before SIGKILL. The first number is about an *agent's* turn
structure, which is the kind of thing only the layer that knows what a session is
could pick — and it is the clearest single illustration of the chapter's rule.
`keyed-launch` could not have defaulted two seconds, because two seconds is a fact
about the thing grove launches and not about launching.

**Why an escalation is needed at all is deliberately not argued here.** *An
interactive session is never reaped on its own, and cannot be trusted to end
itself under every sandbox* is `keyed_launch::Escalation`'s to state, and the
comment says *it states it* rather than restating it. That is the book's *do not
restate* instruction appearing inside the source it applies to.

<a id="reset-terminal"></a>
## `reset_terminal`: a guard that is a hang, not a tidiness

Thirteen lines of comment over eighteen of code, and the comment is entirely
about the second of the function's two early returns.

<!-- fragment «loop-reset-terminal» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="471-502" parent="loop-driver" -->
````rust
/// Reset the terminal after a (possibly SIGTERM'd) TUI: restore cooked mode,
/// leave the alternate screen, show the cursor. No-op when stdin isn't a TTY
/// (headless / test runs), or when this driver is not the terminal's foreground
/// process group.
///
/// **The second guard is not tidiness, it is a hang.** `stty` sets terminal
/// attributes, and `tcsetattr` from a *background* process group raises SIGTTOU
/// at the caller whatever `TOSTOP` says — so a driver that is not the terminal's
/// owner would spawn an `stty` that stops on its first act, and wait on a
/// stopped child forever. A driver reaches here as the owner in the ordinary
/// case, because the runner hands the terminal to the session and takes it back
/// before returning; what this covers is the case where it never had it, which
/// is `grove &` from a shell that kept the foreground for itself.
fn reset_terminal() {
    if unsafe { libc::isatty(libc::STDIN_FILENO) } != 1 {
        return;
    }
    // SAFETY: `tcgetpgrp(3)` on stdin, proved a terminal above, and `getpgrp(2)`.
    if unsafe { libc::tcgetpgrp(libc::STDIN_FILENO) != libc::getpgrp() } {
        return;
    }
    let mut stty = Command::new("stty");
    stty.arg("sane");
    // `stty` reads no `GROVE_*` variable, so this grants it nothing it could
    // act on — and it is scrubbed anyway, because a rule with one argued
    // exception is a rule the next spawn has to re-argue.
    scrub_loop_control_env(&mut stty);
    let _ = stty.status();
    print!("\x1b[?1049l\x1b[?25h\x1b[0m");
    let _ = std::io::stdout().flush();
}

````
<!-- /fragment -->

**Two guards, and the comment insists the second is load-bearing.** The first is
ordinary: no TTY, nothing to reset. The second — *when this driver is not the
terminal's foreground process group* — is the interesting one, and the comment
states the mechanism rather than the symptom. `tcsetattr` from a background
process group raises SIGTTOU **at the caller whatever `TOSTOP` says**, so an
`stty` spawned from a background driver stops on its first act, and the driver
waits on a stopped child forever.

The comment then bounds when that can happen, which is what makes it an
enumeration rather than a slogan: a driver reaches here as the terminal's owner
in the ordinary case, *because the runner hands the terminal to the session and
takes it back before returning*. What the guard covers is the case where the
driver never had the terminal — `grove &` from a shell that kept the foreground.
That is a claim about `keyed-launch`'s behaviour, stated as a dependency rather
than re-derived, and it is the reason the guard is described as covering an
unusual case rather than the usual one.

**The `stty` spawn scrubs, and the comment argues the exception it is not
taking.** `stty` reads no `GROVE_*` variable, so scrubbing grants it nothing —
and it is scrubbed anyway, *because a rule with one argued exception is a rule
the next spawn has to re-argue*. That is the same reasoning `LOOP_CONTROL_ENV`'s
header applies to its two retired members, applied to a call site instead of to a
list, and it is why the file's spawn count of two has a scrub count of two.

The three escape sequences are not commented and do not need to be: leave the
alternate screen, show the cursor, reset attributes. The `let _ =` on both the
`status()` and the `flush()` is the same judgement in both places — a terminal
that cannot be reset is not a reason to fail an iteration that has otherwise
succeeded.

<a id="ignore-interrupts"></a>
## `ignore_interrupts`: one signal, and the reason it is only one

The other half of what `run` does before it delegates.

<!-- fragment «loop-ignore-interrupts» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="503-533" parent="loop-driver" -->
````rust
/// Ignore SIGINT in the driver so a terminal Ctrl-C does not kill the loop. The
/// driver must survive the interrupt to reach the relaunch-vs-stop decision.
///
/// **This covers the gaps between sessions, and only those.** While a session
/// is running it is the terminal's foreground process group in its own right —
/// the runner puts it there and hands it the terminal — so a typed Ctrl-C is
/// delivered to the session and never reaches the driver at all. This ignore is
/// what holds in the moments the driver is transitioning the tree, selecting a
/// leaf and expanding a template with no child in front of it.
///
/// **It does not leak into the session.** An ignored disposition is the one
/// kind that survives `execve`, which is exactly why this used to reach the
/// configured session, everything it spawned, and every wrapper a template
/// named — a login shell that inherits an ignored SIGINT keeps ignoring it and
/// passes it on, so Ctrl-C did nothing at all and nothing in the session could
/// say why. The runner now resets the child's dispositions to their defaults
/// across the spawn (`keyed_launch::run`), so this stays the driver's own
/// policy rather than the subtree's.
///
/// SIGINT and SIGINT alone, because it is the one disposition that is a
/// *policy* rather than a mechanism: what a loop does about the human's Ctrl-C
/// is the loop's business. SIGTERM and SIGHUP belong to the runner, which
/// catches them itself so it can forward one to its child's process group and
/// reap it rather than orphan it onto the terminal — and reports that, with the
/// signal, as [`keyed_launch::End::Interrupted`].
fn ignore_interrupts() {
    unsafe {
        libc::signal(libc::SIGINT, libc::SIG_IGN);
    }
}

````
<!-- /fragment -->

Twenty-five lines of comment over three lines of code, and the comment is three
separate arguments.

**What it covers is the gaps between sessions, and only those.** While a session
runs it is the terminal's foreground process group in its own right — the runner
puts it there — so a typed Ctrl-C is delivered to the session and never reaches
the driver at all. The ignore holds in the moments the driver is transitioning
the tree, selecting a leaf and expanding a template *with no child in front of
it*, which is precisely the window in which a half-finished tree mutation would
be the cost.

**It does not leak into the session, and the comment records that it used to.**
An ignored disposition is the one kind that survives `execve` — so this ignore
once reached the configured session, everything it spawned, and every wrapper a
template named. A login shell that inherits an ignored SIGINT keeps ignoring it
and passes it on, so *Ctrl-C did nothing at all and nothing in the session could
say why*. The fix was not here: `keyed_launch::run` now resets the child's
dispositions to their defaults across the spawn, which is what lets this stay the
driver's own policy rather than the subtree's. This is the second time in the file
that a bug is fixed in the runner and the loop keeps the sentence explaining why.

**SIGINT and SIGINT alone, and the criterion is stated.** It is *the one
disposition that is a policy rather than a mechanism* — what a loop does about the
human's Ctrl-C is the loop's business. SIGTERM and SIGHUP are the runner's, which
catches them itself so it can forward one to its child's process group and reap it
rather than orphan it onto the terminal. That division is what `End::Interrupted`
carries back, and it is why the loop body has two interrupt checks rather than a
signal handler: one for a signal that arrived between sessions, one for a signal
the runner caught during one.

`a_sigtermed_driver_stops_and_reaps_its_child` and
`the_escalation_reaps_the_sessions_descendants` are the tests, and both are
`crates/grove/tests/loop_driver.rs`'s rather than this block's.

<a id="picked"></a>
## `picked`: an arm the transition makes unreachable

The last production item in the file, and the smallest.

<!-- fragment «loop-picked» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="534-546" parent="loop-driver" -->
````rust
/// The driver's own `pick`, over the worktree it is driving.
///
/// The transition above has already brought the worktree to a grove, so the
/// vacant arm is unreachable in practice — but it is an arm of [`crate::read`],
/// and answering it as *no live leaves* is the same thing
/// the transition would have made true a moment earlier.
fn picked(worktree: &Path) -> anyhow::Result<Sought<Selection>> {
    match crate::read(worktree)? {
        Reading::Tree(tree) => Ok(crate::verbs::pick(&tree)?),
        Reading::Vacant => Ok(Sought::Nothing),
    }
}

````
<!-- /fragment -->

The file's last production item, and it is six lines that exist to turn two
readings into one `Sought`.

**The comment makes a reachability claim and hedges it correctly.** *The
transition above has already brought the worktree to a grove, so the vacant arm
is unreachable in practice — but it is an arm of `crate::read`.* Both halves are
right, and the second is the one that matters: `read` returns `Reading::Vacant`
whether or not this caller can produce it, so the arm has to be written. What the
comment declines to do is claim the arm is dead — it says *in practice*, and then
gives the answer it would produce anyway, which is *the same thing the transition
would have made true a moment earlier*.

That hedge is the difference chapters 16 and 17 kept insisting on, made by the
source itself: an arm no fixture can reach through this path is not the same as an
arm that could not exist. The mutation below measures which it is.

<a id="the-test-block"></a>
## The inline test block — sixty-nine lines, two tests, one claim

**The prose obligation changes here, inside this chapter.** Everything above
takes *do not restate*: the production half is 51.5% comment prose, the comments
already argue, and the fragments quote them verbatim. The 69 lines below are 8.7%
comment prose, and they take *supply the claim* — for each reproduced test, the
property it establishes **and what would have to be true for it to pass while the
property was broken**. Chapter 20 is the last of the thirteen chapters on that
list, and the only one where the two instructions meet on one page.

<!-- fragment «loop-tests-open» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="547-556" parent="loop-driver" -->
````rust
#[cfg(test)]
mod tests {
    use super::*;

    /// The two `complete_post_reap_epoch_handoff` cases below drive that
    /// ordering with a stand-in for the launch result, because what the
    /// ordering is *about* is which of the two failures survives — not what a
    /// session left behind. The launch and escalation themselves are the
    /// runner's, and `crates/keyed-launch/tests/launch.rs` drives them end to
    /// end against a fake child.
````
<!-- /fragment -->

**The module's one comment is a scoping statement, and it is doing real work.**
It says the two cases *drive that ordering with a stand-in for the launch result,
because what the ordering is about is which of the two failures survives — not
what a session left behind*. That is why `complete_post_reap_epoch_handoff` is
generic over `<E, T>`: `E` is instantiated as `&str` in both tests, so neither
constructs an `Ended`, a `Channel`, a `Selection` or a child process. The
end-to-end behaviour is `crates/keyed-launch/tests/launch.rs`'s, against a fake
child, and the comment says so rather than leaving the gap to be read as an
omission.

<!-- fragment «loop-test-handoff-preserves» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="557-592" parent="loop-driver" -->
````rust
    #[test]
    fn an_epoch_handoff_failure_preserves_the_launch_failure_that_preceded_it() {
        let launch: Result<&str> = Err(anyhow::anyhow!(
            "launching the session: executable was not found"
        ));
        let continuation_called = std::cell::Cell::new(false);

        let error = complete_post_reap_epoch_handoff(
            launch,
            || {
                Err(anyhow::anyhow!(
                    "timed out waiting for exclusive session epoch lock"
                ))
            },
            |_| continuation_called.set(true),
        )
        .unwrap_err();

        let message = format!("{error:#}");
        assert!(
            message.contains(
                "post-reap session epoch invalidation blocked; completion signal left unconsumed"
            ),
            "{message}"
        );
        assert!(message.contains("executable was not found"), "{message}");
        assert!(
            message.contains("timed out waiting for exclusive session epoch lock"),
            "{message}"
        );
        assert!(
            !continuation_called.get(),
            "signal interpretation must remain behind successful epoch invalidation"
        );
    }

````
<!-- /fragment -->

**What it establishes.** In the arm where *both* the launch and the invalidation
failed, the returned error carries all three texts — the invalidation context, the
launch error's message and the invalidation error's message — and the
continuation does not run. The last assertion is the ordering one, and it is the
only assertion in the test that cannot be satisfied by a message.

**What would have to be true for it to pass while the property was broken.** Two
things, and they are different in kind.

- **The asymmetry is not pinned.** The property the arm exists for is that the
  *invalidation* error is the one returned and the launch error is folded into
  its message — because a launch that failed is a session that did not run, while
  an epoch that will not invalidate is a working tree the next iteration cannot
  enter. An implementation that swapped them, returning the launch error with the
  invalidation error folded in, would put all three of these substrings into
  `format!("{error:#}")` just the same. The test reads the rendered chain with
  `{:#}`, which flattens context and cause into one string, so *which error is the
  identity* is exactly the distinction the rendering erases. It would pass with
  the asymmetry inverted.
- **The context string is shared with its sibling arm.** `INVALIDATION_CONTEXT` is
  the same constant in the `(Ok, Err)` and `(Err, Err)` arms, so
  `contains("post-reap session epoch invalidation blocked…")` pins *an
  invalidation failure was reported* and not *this arm ran*. It is the shared-context
  problem chapter 17 met in `admit_session`'s seven rungs, in miniature: a
  substring matching two sites looks exactly like one matching one. What
  distinguishes the arms here is the launch error's text, and that is asserted —
  so the pair of assertions together does reach this arm, though neither does
  alone.

<!-- fragment «loop-test-ordering» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="593-615" parent="loop-driver" -->
````rust
    #[test]
    fn signal_interpretation_cannot_run_before_epoch_invalidation_succeeds() {
        let continuation_called = std::cell::Cell::new(false);

        let error = complete_post_reap_epoch_handoff(
            Ok("a session that ended"),
            || Err(anyhow::anyhow!("exclusive epoch handoff timed out")),
            |_| continuation_called.set(true),
        )
        .unwrap_err();

        assert!(
            error.to_string().contains(
                "post-reap session epoch invalidation blocked; completion signal left unconsumed"
            ),
            "{error:#}"
        );
        assert!(
            !continuation_called.get(),
            "signal interpretation must remain behind successful epoch invalidation"
        );
    }
}
````
<!-- /fragment -->

**What it establishes.** In the arm where the session ended cleanly but
invalidation failed, the error's *outermost* context is `INVALIDATION_CONTEXT`
and the continuation does not run. This is the stricter of the two readings:
`error.to_string()` renders only the outermost context, where the first test's
`format!("{error:#}")` renders the whole chain. So this test does pin that the
invalidation failure is the returned error's identity — for this arm.

**What would have to be true for it to pass while the property was broken.** The
underlying error is never asserted. `"exclusive epoch handoff timed out"` — the
text the closure returns — appears nowhere in the assertions, so an
implementation that discarded the invalidation error entirely and returned a bare
`anyhow!(INVALIDATION_CONTEXT)` would satisfy both assertions. What the test pins
is *the context, and the ordering*; what it does not pin is *the preservation of
the cause*. Its sibling above pins preservation and not the identity. **Neither
test pins both, and between them the two properties are covered once each** —
which is a more useful thing to know than either name suggests, since both names
promise ordering and the ordering is the half they agree on.

**What neither test reaches.** The `(Ok, Ok)` arm — the ordinary path, where
interpretation actually runs — and the `(Err, Ok)` arm, a launch failure with a
clean invalidation, have no inline test. They are not untested: they are the
paths every end-to-end launch takes, and the mutation below attributes them.

<a id="six-calls-into-the-lease"></a>
## Six calls into chapter 16, and what each one is for

Chapter 16 closed by enumerating its block's outward surface as nine production
call sites in three files, and noting that **six of them are this chapter's**.
Enumerating from this side returns the same six, and this is the page that says
what each reaches rather than re-deriving the lease's contract:

| Line | Call | What the loop needs it for |
|---:|---|---|
| 212 | `worktree_root` | the directory every later step addresses, taken from the lease rather than the workspace |
| 238 | `revalidate` | *am I still the only driver* — before the tree is transitioned |
| 280 | `revalidate` | the same question again, before a child is spawned |
| 283 | `control_dir` | choice 1: the directory the channel is allocated in |
| 418 | `activate_session_epoch` | opening the window in which the child's verbs are admitted |
| 301 | `invalidate_session_epoch` | closing it, before the token is read |

**The two `revalidate`s are the pair worth pausing on.** They are not
belt-and-braces: they protect two different things. The first stands in front of
a *tree mutation*, and the second in front of a *spawn*, and between them sit the
walk and the template expansion — long enough for another driver to have taken
the lease. A single call at the top of the iteration would leave whichever of the
two came later unguarded, and neither failure is one the other would catch.

That is the [driver lease](../../../CONTEXT.md#driver-lease) and the
[session epoch](../../../CONTEXT.md#session-epoch) used rather than explained;
`one-live-driver-per-working-tree` is the record, named here and cited nowhere.

<a id="what-holds-the-four-choices"></a>
## What holds the four choices, measured

The four values this module exists to choose are not held evenly, and the spread
is wide enough to be the chapter's last technical point. Every row below is a
mutation in a copy of the workspace, diffed name by name against a control of
**277 tests with no failures at all** — `cargo test --no-fail-fast -p grove-loop
--lib` plus `-p grove` over `env_hygiene`, `lifecycle_cutover` and `loop_driver`.
A control with nothing already red is what makes each row attributable: there is
nothing to subtract.

| The four choices | Newly failing | Never reported |
|---|---:|---:|
| 1 · channel allocated in `/tmp` rather than the lease's control directory | 3 | 1 |
| 2 · `CHANNEL_VAR` renamed | **7** | 2 |
| 3 · all three scrub names replaced | 1 | 0 |
| 4 · both graces zeroed | **0** | 0 |

The run is drawn as three tables because the mutations were aimed at three
different things, and the split is what makes the counts comparable. The first
table is the four choices this chapter is about. The second is the machinery that
places them — the orderings and the match arms — and it is here so that a choice's
score can be read against the score of the code that carries it. The third is
every mutation that moved nothing, gathered rather than scattered so that the
zeros can be read as a set; its last row is a negative control and is expected
there.

| The orderings, and the arms | Newly failing | Never reported |
|---|---:|---:|
| epoch invalidation skipped entirely | **4** | 0 |
| epoch activation before the spawn deleted | 2 | 1 |
| handoff `(Ok, Err)` arm replaced by a silent panic | 2 | 0 |
| handoff `(Err, Ok)` arm replaced by a silent panic | 1 | 0 |
| handoff `(Err, Err)` arm replaced by a silent panic | 1 | 0 |
| the finish template's presence check deleted | 1 | 0 |

| The zeros | Newly failing |
|---|---:|
| `reset_terminal`'s call deleted | 0 |
| the `stty` spawn's scrub deleted | 0 |
| `ignore_interrupts` deleted | 0 |
| `picked`'s `Vacant` arm replaced by a silent panic | 0 |
| `worktree_name`'s fallback replaced by a silent panic | 0 |
| a `revalidate` context string reworded — negative control | 0 |

**The instrument detects the mutations, as established by the rows that fire
rather than asserted.** Nine of the ten rows in the first two tables turn tests red, one of
them by four; the mutation form
throughout is a panic that **says nothing**, replacing a whole macro call, because
a message-preserving panic is invisible to an out-of-process suite asserting on
stderr substrings.

**Choice 2 is the most heavily observed thing in the file.** Rename the variable
and the child's `grove-llm complete` writes where nothing is watching, so no
signal ever arrives. Two of the tests that would have caught it never finished —
they wait on a completion that cannot come — and **a hang is not a failure**. It
is reported in its own column rather than folded into the count, because a test
that never ran is exactly what a broken instrument also produces.

**Choice 3 is observed, but not where the comment works hardest.** Dropping the
whole list and dropping only the two retired members give the *identical*
one-test failing set, so the isolating pair settles it:

| Mutation | Newly failing |
|---|---:|
| drop only `GROVE_SIGNAL_FILE`, keep the two retired | **0** |
| drop only `GROVE_HARNESS_PID` and `GROVE_CLAUDE_PID`, keep the live one | **1** |

So the two **retired** variables — the ones the comment has to argue hardest to
justify keeping — are the only members any test pins, through
`bare_grove_launches_the_selected_filename_kind_with_one_mandate_argument`'s two
assertions that each read back `<unset>`. The **live** one's membership is
observed by nothing, and that is a **guard rather than a gap**: the configured
session's spawn grants the variable back on the very next field, so scrubbing it
there is a no-op by construction, and the only spawn that would show the
difference is the `stty` in `reset_terminal` — which, as the zeros below
establish, never runs under the suite at all.

**And the test whose name most suggests otherwise is about a different list.**
`the_shared_scrub_list_covers_the_loop_control_channel` asserts that
`support::grove_env_names()` contains `GROVE_SIGNAL_FILE` — the scrub list in
`testing/support.rs` that the *suite* uses to protect itself, not
`LOOP_CONTROL_ENV`. It passes under every mutation of this constant because it
never reads it. Chapter 17's lesson was to read a test's assertions rather than
its name; this is that lesson one level out, where the name points at the right
subject in the wrong file.

**Choice 4 is held by nothing executable, and that is honest rather than
alarming.** Zeroing both graces leaves all 277 green, including
`the_escalation_reaps_the_sessions_descendants` — which establishes that the test
observes *that* descendants are reaped and not *how long* the two waits are. Two
seconds is a duration chosen against an agent's turn structure, and the fixtures
launch children that exit on their own rather than children that must be escalated
out of an unfinished turn. It is pinned by the comment's argument and by nothing
else in the corpus, which is the shape chapter 17 found for
`EPOCH_HANDOFF_TIMEOUT` and `EPOCH_WAIT_INTERVAL`.

**The 69-line test block is not redundant with the end-to-end suite.** The four
handoff arms attribute cleanly, and one of them has the inline tests as its only
observer: `(Err, Ok)` is held by
`spawn_failure_names_the_kind_executable_and_config_without_retiring_the_leaf`
alone; `(Ok, Err)` by the second inline test **and**
`an_orphaned_epoch_guard_stops_before_consuming_the_relaunch_signal`; `(Err, Err)`
by the first inline test and nothing else. Skipping invalidation altogether turns
four red, both inline tests among them. The block therefore covers the one
combination no end-to-end fixture produces — two failures at once.

<a id="the-zeros"></a>
## Five zeros, and which kind of zero each one is

A zero means *no test distinguishes this* and never *no test could*, and the two
need separating on the page rather than being reported as one absence. The
separating move is to enumerate the guards on the path, and in one case it takes
a second mutation.

**`reset_terminal` and its `stty` scrub are unreachable, not untested — and the
guard that stops them is nameable.** Two mutations bracket it: a silent panic on
**entry** to the function turns 10 of `lifecycle_cutover`'s 17 and 10 of
`loop_driver`'s 11 red, so the function is very much reached; the same panic
placed one line **after** the `isatty` guard turns **none** red. Nothing under
`cargo test` has a terminal on stdin, so every test returns at the first line of
the body. Everything below it — the SIGTTOU guard the comment argues hardest for,
the `stty` spawn, its scrub, and the three escape sequences — cannot execute under
the suite at all. That is why deleting the call and deleting the scrub both read
zero, and it is a different finding from *nobody wrote a test*.

**`ignore_interrupts` is unobservable for want of a stimulus.** Nothing in the
suite sends the driver a SIGINT, so ignoring it and not ignoring it are the same
run. The disposition is a policy about a human's keystroke, and the harness has no
human.

**`picked`'s `Vacant` arm is unreachable through this path, and the comment said
so first.** A silent panic in it turns nothing red, which is what *the transition
above has already brought the worktree to a grove* predicts: by the time `picked`
runs, `transition_to_current` has guaranteed a tree. The comment declined to call
the arm dead and gave the answer it would produce anyway; the measurement agrees
with the hedge rather than with the stronger claim it avoided making.

**`worktree_name`'s fallback is the same shape.** A silent panic in the
`unwrap_or_else` turns nothing red: the lease has proved a `.jj/` beside a named
directory before `drive` ever asks for its basename, so `file_name()` returning
`None` is not a state the loop can reach.

**And the negative control is the row that should be zero.** Rewording a
`revalidate` context string changes no behaviour and turns nothing red, which is
the expected reading — but it is the weakest evidence in the table, because a
broken instrument would report it identically. The rows that carry the table are
the nine that fire.

<a id="what-could-not-move-here"></a>
## What could not move

The book's three questions, asked of the last root and the last chapter that owns
one. This chapter states no rank for it: chapter 19 is the one page a later page
should take a Part V size from, and nothing in the questions below wants an
ordinal.

**On the way in — the names.** None of the grammar's, and one of its own. This
module parses no filename, spells no token and constructs no name: `Handle`,
`Kind` and `Selection` all arrive already made, through one guarded `pick`. The
grammar's cost — a conformance kit, canonicity, `format(parse(f)) == f` — was paid
across `task_name.rs`'s 1,714 lines in chapters 2 to 4, and what reaches here is
the return on it. Chapter 19 answered this question the same way and found one
name of its own, `PLUGIN`; this chapter finds one too, and it is not in the
grammar at all but in the *environment*: `CHANNEL_VAR`,
a string two binaries have agreed on with no shared constant between them, whose
only enforcement is that renaming it makes nine tests fail or hang.

**On the way through — the preconditions.** Two, and they are placed rather than
merely present. The lease is revalidated **twice** in every iteration — once
before the tree transition, once before the launch — because the question *am I
still the only driver* has two different answers to protect: a tree about to be
mutated, and a child about to be spawned. And the finish template's presence is
asked against the **pre-transition** document, before anything is written, because
`entries-are-never-removed` makes a leaf written in error permanent. That is the
outcome's second cost — *the check must run against the same snapshot the
operation then plans from* — and it is the reason this file reads its
configuration twice rather than once, which is what `session_config.rs`'s own
doc comment miscounted until `template-source-read-count-k86` corrected it.

**On the way out — the policy.** Four values, and the module is nothing else.
`keyed-launch` can spawn a child, publish a path to it, poll a file beside it and
escalate — and it has no way to know *which* directory survives a driver's death,
*which* variable another binary reads, *which* names would let a descendant kill
its own parent, or how long an agent needs to finish a turn. Each of those is a
fact about grove and not about launching, which is why the header can hand away
the spawning, the watching and the escalating in one bold sentence and still have
something left to be.

**And the thing this chapter is really for.** The loop is the one place where all
twenty preceding chapters are used at once: the transition from chapter 14, the
walk from chapter 7, the lease and epoch from chapters 16 and 17, the
configuration from chapter 18, the prompt from chapter 19 — five subsystems,
called in order, in the 142 lines of `drive`. It could have been the place where
the crate's thesis broke down, because a loop is the natural home for a cursor,
a cache and a retry count. It holds none of them. Position is re-derived from the
tree on every pass, the configuration is re-read rather than remembered, and the
only value that survives an iteration is the lease that proves the driver may
still act. **Restart is continuation** because there is nothing to restore — and
that is chapter 13's *the tree's shape is the only state* proved in the one module
that had every excuse to break it.

[Previous: The guaranteed core](19-the-core.md) | [Contents](README.md) | [Next: What could not move](21-what-could-not-move.md)
