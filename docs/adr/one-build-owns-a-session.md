# One build owns a session

The methodology a session reads and the `grove-llm` it invokes must come from
one `grove` build. Grove **repairs** that pairing in the one artifact it owns,
**reports** it everywhere else, and declares the remainder unsupported rather
than defending it. No check refuses a launch over it: which `grove-llm` a
session reaches is not a fact the driver can establish about a process it has
not yet started. The global skill directories are the one place the pairing can
be broken from outside, because they are shared by every build on the machine
while the driver lease is scoped to a single working tree.

A build's **methodology identity** is the content hash of its embedded
`content/` **file payload** — every embedded file's path and bytes, which is the
value already written as the provisioning stamp. It is the identity rather than
the crate version because the version does not move between a released binary
and an edited checkout at that same version, which is precisely the case
provisioning had to re-extract for and the case a version comparison cannot see.
`build.rs` emits that hash as a compile-time constant, so every binary in the
crate can name its own identity without linking the embed: only `grove` extracts
content, so only `grove` carries it, and a `grove-llm` that hashed its own tree
at runtime would grow by the size of `content/` for a value known at compile
time. An in-crate test asserts the emitted constant equals the runtime hash of
the linked embed, which is what keeps the build script's traversal and
`provision`'s from drifting apart.

The grain is the file payload and not the embedded directory *structure*.
`include_dir` embeds an entry for every directory, including an empty one, so an
empty directory added to `content/` would change what is extracted without
changing the identity. That is named rather than closed: a directory with no
files in it carries no methodology, and hashing typed directory paths would
oblige the build script to reproduce `include_dir`'s directory semantics as well
as its file selection — doubling the surface the equality test exists to keep
from drifting. Both traversals hash files, and empty directories are explicitly
not part of a build's identity.

Each loop iteration resolves `grove-llm` through **`PATH`** — the way a session
that inherits the driver's environment resolves it — and compares its
methodology identity with the driver's own. A relative or empty entry resolves
from the **worktree root**, and the probe runs there too, because that is the cwd
the session is spawned with while bare `grove` is accepted from any directory
inside the tree; resolving from the driver's own cwd would inspect a binary no
session can reach, and would execute an unrelated repository-local helper to do
it. The probe still runs whatever such an entry names — that is exactly the
binary the session would run, and a probe that declined to would report on a
resolution it had not performed.

A missing, unidentifiable, or disagreeing binary prints one diagnostic and the
launch proceeds. **Each names only what that branch can know**, because two of
the three have no peer to name: nothing resolved at all, so the missing
diagnostic names this build's identity and the search it performed and there is
no path or peer identity to give; an unidentifiable binary resolved but could not
answer, so its diagnostic names the resolved path, this build's identity, and why
the answer was not one; only a mismatch has both operands, and names the resolved
path and both identities. All three end in the same requirement, which is what
makes the missing case actionable without a peer.

Resolution deliberately does not prefer the sibling of the running executable:
the driver never invokes `grove-llm` itself, so the sibling agrees with it by
construction while the binary a session reaches goes unchecked — and the sibling
is exactly what makes the dogfooding case invisible, since `cargo run` builds
both binaries side by side. The check stays
per iteration rather than per driver start, because a mid-loop `brew upgrade`
replaces the binaries on disk under a running text segment.

It resolves in the *driver's* environment, which is the session's environment
only when the configured command inherits it. A template that re-derives
`PATH` — a login shell, an `ssh` hop, a container, any wrapper — is supported
policy under [complete session
configuration](complete-session-configuration.md) and opaque by design, so
behind one of those the probe can disagree while the session is correct or agree
while it is wrong. **The CLI behind an opaque target is observable only when it
runs.** That is why the probe reports rather than refuses. A refusal would be a
claim the driver cannot support, and the two errors do not cost the same: a
missed mismatch misleads one session, while a false refusal launches nothing at
all and stalls the loop of a correctly configured machine. The line this draws
is the one Grove's human surface already follows: it stops on what governs its
*own* operation — its configuration, its lease, its workspace layout — and
reports what it can only predict about a *session's* environment. Two cases
follow from that line rather than needing their own rule. A `grove-llm`
predating the identity flag is unidentifiable, not mismatched, and refusing
there would make upgrading the pair impossible from inside the loop. A
`grove-llm` the driver cannot resolve at all is the same kind of claim as a
mismatch — the driver never invokes it, and a container or `ssh` target that
supplies its own is a supported shape where the driver's `PATH` is simply not
the one that matters — so it too is reported and the launch proceeds.

The remedy a diagnostic prints is a requirement, not a single command: the build
being driven must be the one that **resolves first on the session's `PATH`**.
Installing the checkout (`cargo install --path .`) achieves that only where
`~/.cargo/bin` outranks every other prefix holding a `grove-llm`; where a
package-manager prefix precedes it, the Cargo-installed pair is present and
still not the one a session reaches, and re-prescribing the install would
prescribe something already done. So a diagnostic that resolved something names
the path it actually resolved — with both identities where both exist — and the
operator makes the intended pair the resolved one, by installing over the prefix
that wins, by reordering `PATH`, or by any equivalent their setup supports. Where
nothing resolved there is no prefix to name, and the requirement alone is the
whole remedy.

Provisioning runs before every launch, but as a **stamp re-verification** rather
than a re-extraction. A matching stamp is the ordinary case and costs one small
read per installed harness root. A differing stamp means another `grove` build
has taken the directory; the driver restores its own embed and prints one
diagnostic naming the directory. Re-*extracting* every iteration would be pure
cost — a driver never re-execs, so the bytes are identical — and that is why the
loop re-verifies instead: the question is ownership, not freshness.

The check inside the session is the one whose operands are the two that matter:
the CLI actually invoked, and the methodology actually on disk in front of it.
It is also the only one a clobber landing *after* launch can reach. On any verb
`grove-llm` compares its own methodology identity with the stamp of each
installed skill directory and, on disagreement, prints one line on stderr naming
the directory and both identities. It never refuses: Grove guides and does not
gate on the agent surface, and the session least able to absorb a hard stop is
one already mid-task with uncommitted work.
Absence is not disagreement — an unprovisioned or missing directory is silent.
The check is deliberately conservative in one direction: a clobber landing after
the harness loaded its skill catalog warns a session whose in-context
methodology is still correct, because the human reading that line is the one who
can stop the *next* session from being wrong.

What remains is unsupported and now says so. Two concurrent groves at different
builds share one global directory and cannot both be served; they alternate,
each restoring its own embed and each announcing the other's clobber, and the
documented resolution is to run one build. Dogfooding is the same constraint
seen from inside this repository: `cargo run --bin grove` provisions a
checkout's `content/` over the installed copy while the session's `PATH` still
reaches the installed `grove-llm`, and it is the obvious next move for anyone
who has just read why the skill looked stale. That pairing is now announced
twice — once by the driver before the launch, once by the session's own
`grove-llm` — and it is still not prevented, which is the honest limit of a
design that cannot see into an opaque target. The supported way to run a build
is to make it the installed one that a session's `PATH` resolves first.

## Considered options

- **Document the hazard and spend nothing on mechanism.** Rejected because the
  likeliest way in is a shortcut the documentation itself provokes, and a
  prose-only answer leaves Grove manufacturing on its own dogfooding path the
  skew its build-boundary rule forbids. Reopen if a checkout-built driver stops
  being able to write a directory a released build owns.
- **Keep comparing crate versions.** Rejected because the version demonstrably
  cannot see the case: provisioning already stamps a hash rather than a version
  so that an edit at an unbumped version re-extracts, and the pair check has the
  same blind spot — this repository sits in it, with the manifest and both
  installed binaries at one version while `content/` has moved a release ahead.
  Reopen never; the hash is strictly finer and costs 64 bytes.
- **Hash the embedded directory structure as well as the files.** Rejected
  because it buys only empty directories — every directory holding a file is
  already named by that file's path — while requiring the build script to
  reproduce `include_dir`'s directory-entry semantics on top of its file
  selection, which is the drift the one equality test exists to prevent and
  which a second traversal doubles. The consequence is stated rather than
  hidden: an empty directory is not part of a build's identity. Reopen if
  extraction ever depends on an empty directory existing, which would make its
  absence a difference a session can feel.
- **Refuse the launch when the driver-side probe disagrees.** Rejected because
  the driver cannot observe which `grove-llm` an opaque configured command
  resolves, so the refusal would rest on a measurement of the *driver's*
  environment: a supported wrapper with the correct CLI would be refused before
  it started, and bare `grove` would launch nothing at all rather than launch
  something imperfect. A binary too old to answer the identity flag would be
  refused for the same reason, making the pair unupgradable from inside the
  loop. Reopen if a configured target ever exposes its effective environment or
  its resolved tool paths to the driver, which would turn the proxy into an
  observation.
- **Drop the driver-side probe and rely on the session's own warning.** Rejected
  because that warning is emitted inside a harness that commonly repaints the
  terminal, and it arrives after the session has already read the methodology.
  The driver's line lands between sessions, in scrollback nothing overwrites,
  and it is the only one a human watching the loop reliably sees. Reopen if the
  agent-side warning is ever surfaced somewhere the loop's operator reads.
- **Keep preferring the sibling of the running executable.** Rejected because
  the driver discards the binary it resolves, so the sibling is checked and the
  session's `PATH` binary is run — and because the sibling is what hides the
  motivating case: `cargo run` builds `grove` and `grove-llm` side by side, so a
  checkout driver's sibling always agrees with it while the session reaches the
  installed CLI. Reopen if the driver ever invokes `grove-llm` itself, which
  would make the sibling the binary that matters.
- **Prepend a one-entry shim directory holding the driver's own `grove-llm` to
  the launched session's `PATH`.** This is the only option that would *prevent*
  the mismatch rather than report it, and it would do so exactly for the targets
  the probe can see — those that inherit the driver's environment. Rejected on
  its merits rather than by citation: it would decide which binary the
  operator's configured command resolves, which is hidden launch policy of the
  kind [complete session
  configuration](complete-session-configuration.md) exists to keep out of the
  launch — and here the objection is the thing itself, not the other decision's
  wording, because the effect is to make an uninstalled build the effective CLI
  for a session whose owner never chose it. It also prevents nothing behind a
  wrapper that re-derives `PATH`, so it removes neither of the two reports.
  Reopen if a supported launch shape is found that cannot install its own build,
  which would leave prevention as the only coherent pairing mechanism.
- **Carry the driver's methodology identity in the session environment and have
  `grove-llm` compare against it.** Rejected because it answers "does my CLI
  match the driver?" when the load-bearing question is "does my CLI match the
  methodology in front of me?", and because it cannot see a clobber that lands
  after launch. The cost is not that Grove may set no variable — it already
  grants a fresh signal path — but that this one would carry an answer the
  stamps give more directly. Its premise is that every installed root carries one
  embed, so reopen if provisioning ever becomes targeted rather than a sweep of
  all of them.
- **Have `grove-llm` refuse on a stamp mismatch.** Rejected because a refusal
  mid-task destroys more than the mismatch does, and the human who can act on it
  is reading the same stream. Reopen if a mismatch is shown to corrupt a task
  tree rather than mislead a session.
- **Give each session a private skill directory.** Rejected because skill
  discovery is a personal, global harness contract, and reaching it would mean
  Grove adding an argument or a variable to the command it launches — the one
  thing complete session configuration forbids. Reopen if every supported
  harness gains a skill-path input Grove can set without touching the configured
  argv.
- **Re-extract the embed every iteration.** Rejected because a driver never
  re-execs, so the write is identical bytes at full cost. Stamp re-verification
  subsumes it and extracts only when another build has taken the directory.
  Reopen never.
- **Serialize provisioning behind a machine-wide lock.** Rejected because a lock
  orders the writes without choosing between them: the loser still holds the
  wrong directory, and the driver lease is deliberately per working tree. Reopen
  only if Grove acquires a machine-wide owner whose state it can verify.
