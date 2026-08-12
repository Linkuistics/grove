# One build owns a session

The methodology a session reads and the `grove-llm` it invokes must come from
one `grove` build. Grove enforces that pairing where it can observe it, reports
it where it cannot, and declares the remainder unsupported rather than
defending it. The global skill directories are the one place the pairing can be
broken from outside, because they are shared by every build on the machine
while the driver lease is scoped to a single working tree.

A build's **methodology identity** is the content hash of its embedded
`content/` — the value already written as the provisioning stamp. It is the
identity rather than the crate version because the version does not move
between a released binary and an edited checkout at that same version, which is
precisely the case provisioning had to re-extract for and the case a version
comparison cannot see. `build.rs` emits that hash as a compile-time constant, so
every binary in the crate can name its own identity without linking the embed:
only `grove` extracts content, so only `grove` carries it, and a `grove-llm`
that hashed its own tree at runtime would grow by the size of `content/` for a
value known at compile time. An in-crate test asserts the emitted constant
equals the runtime hash of the linked embed, which is what keeps the build
script's traversal and `provision`'s from drifting apart.

Each loop iteration resolves `grove-llm` **the way the session will** — through
`PATH` — and compares its methodology identity with the driver's own. A missing
or disagreeing binary refuses the launch, naming both identities and the one
supported remedy: install the build you intend to drive. Resolution deliberately
does not prefer the sibling of the running executable. The driver never invokes
`grove-llm` itself, so the sibling is not the binary any session runs; preferring
it measures a binary that agrees with the driver by construction while the one
the session reaches goes unchecked. The check stays per iteration rather than
per driver start, because a mid-loop `brew upgrade` replaces the binaries on
disk under a running text segment.

It resolves in the *driver's* environment, which is taken as representative of
the session's. A wrapper that re-derives `PATH` — a login shell, an `ssh` hop, a
container — is outside what the check can see, in both directions, and can be
refused while correct or admitted while wrong. That is the assumption the
version check already made; pointing it at the binary a session would actually
run makes it finer without widening it, and the agent-side warning below is what
covers the far side of such a wrapper.

Provisioning runs before every launch, but as a **stamp re-verification** rather
than a re-extraction. A matching stamp is the ordinary case and costs one small
read per installed harness root. A differing stamp means another `grove` build
has taken the directory; the driver restores its own embed and prints one
diagnostic naming the directory. Re-*extracting* every iteration would be pure
cost — a driver never re-execs, so the bytes are identical — and that is why the
loop re-verifies instead: the question is ownership, not freshness.

A clobber that lands *after* a session is launched reaches no launch-time check,
so `grove-llm` carries the backstop. On any verb it compares its own methodology
identity with the stamp of each installed skill directory and, on disagreement,
prints one line on stderr naming the directory and both identities. It never
refuses: Grove guides and does not gate on the agent surface, and the session
least able to absorb a hard stop is one already mid-task with uncommitted work.
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
who has just read why the skill looked stale. The launch check refuses that
pairing before a session starts. There is one supported way to run a build —
install it (`cargo install --path .`), which makes the pairing coherent for
every grove on the machine rather than only for the one being driven.

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
- **Keep preferring the sibling of the running executable.** Rejected because
  the driver discards the binary it resolves, so the sibling is checked and the
  session's `PATH` binary is run. Reopen if the driver ever invokes `grove-llm`
  itself, which would make the sibling the binary that matters.
- **Prepend Grove's own directory, or a one-entry shim directory, to the
  launched session's `PATH`.** Rejected because it is prevention that does not
  prevent — a template that re-derives `PATH`, such as a login shell or an
  `ssh` hop, reaches a different binary anyway — while costing a third
  intervention in a session environment that
  [complete session configuration](complete-session-configuration.md) otherwise
  promises to preserve. Installing the build is one command and fixes the
  machine rather than one launch. Reopen if a supported launch shape is found
  that cannot install its own build.
- **Carry the driver's methodology identity in the session environment and have
  `grove-llm` compare against it.** Rejected as the backstop because it answers
  "does my CLI match the driver?" when the load-bearing question is "does my CLI
  match the methodology in front of me?", and it cannot see a clobber that lands
  after launch. Reading the stamps answers the right question and adds no
  ambient variable. Its premise is that every installed root carries one embed,
  so reopen if provisioning ever becomes targeted rather than a sweep of all of
  them.
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
