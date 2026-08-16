# One build owns a session

The methodology a session reads and the `grove-llm` it invokes must come from one
`grove` build. Grove **reports** that pairing before every launch and declares
the remainder unsupported rather than defending it. No check refuses a launch
over it: which `grove-llm` a session reaches is not a fact the driver can
establish about a process it has not yet started.

The skew is the one this record was written for. [The skill delivers the
methodology](skill-delivers-the-methodology.md), so a **shared mutable
directory** is where a session's methodology comes from: the driver sweeps its
own embed into every installed harness's personal skill directory, those
directories are global while the driver lease is per working tree, and nothing
serializes two builds writing one. The mismatched pair is therefore **two copies
of a whole methodology** — the provisioned skill and the resolved CLI — and the
session reads the first while every verb it runs comes from the second.

**That skew is quiet**, and its quietness is what the two reports exist for.
Nothing errors: a session handed one build's methodology and another build's CLI
simply proceeds, and what it does wrong depends on where the two disagree, which
is not something either binary can notice from where it stands. So the driver
reports the pairing before every launch, and `grove-llm` warns per verb when a
skill directory it can see is stamped with a methodology other than its own. The
per-verb warning is the one a clobber landing *after* launch can still reach.

A build's **methodology identity** is the content hash of its embedded `content/`
**file payload** — every embedded file's path and bytes. It is the identity rather
than the crate version because the version does not move between a released
binary and an edited checkout at that same version, which is precisely the
pairing that has to be detectable and precisely the case a version comparison
cannot see. Both binaries link the embed now — `grove` to provision it and to inline the
session-ending file into the guaranteed core, `grove-llm` to serve units and to
name a foreign skill directory — so both compute the identity from the embed itself
through one implementation, and there is no second traversal that could drift
from it. An empty directory is not part of the payload, which is uninteresting
rather than a trade-off: a directory with no files in it carries no methodology.

`grove-llm`'s answer is a claim about the content it actually **carries**, which
is what makes the comparison worth making. It reports the identity of the embed
it links, not a constant recorded beside it.

Each loop iteration resolves `grove-llm` through **`PATH`** — the way a session
that inherits the driver's environment resolves it — and compares its methodology
identity with the driver's own. A relative or empty entry resolves from the
**worktree root**, and the probe runs there too, because that is the cwd the
session is spawned with while bare `grove` is accepted from any directory inside
the tree; resolving from the driver's own cwd would inspect a binary no session
can reach, and would execute an unrelated repository-local helper to do it. The
probe still runs whatever such an entry names — that is exactly the binary the
session would run, and a probe that declined to would report on a resolution it
had not performed.

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
both binaries side by side. The check stays per iteration rather than per driver
start, because a mid-loop `brew upgrade` replaces the binaries on disk under a
running text segment.

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
all and stalls the loop of a correctly configured machine. The line this draws is
the one Grove's human surface already follows: it stops on what governs its *own*
operation — its configuration, its lease, its workspace layout, its own embed at
build time — and reports what it can only predict about a *session's*
environment. Two cases follow from that line rather than needing their own rule.
A `grove-llm` predating the identity flag is unidentifiable, not mismatched, and
refusing there would make upgrading the pair impossible from inside the loop. A
`grove-llm` the driver cannot resolve at all is the same kind of claim as a
mismatch — the driver never invokes it, and a container or `ssh` target that
supplies its own is a supported shape where the driver's `PATH` is simply not the
one that matters — so it too is reported and the launch proceeds.

The remedy a diagnostic prints is a requirement, not a single command: the build
being driven must be the one that **resolves first on the session's `PATH`**.
Installing the checkout (`cargo install --path .`) achieves that only where
`~/.cargo/bin` outranks every other prefix holding a `grove-llm`; where a
package-manager prefix precedes it, the Cargo-installed pair is present and still
not the one a session reaches, and re-prescribing the install would prescribe
something already done. So a diagnostic that resolved something names the path it
actually resolved — with both identities where both exist — and the operator
makes the intended pair the resolved one, by installing over the prefix that
wins, by reordering `PATH`, or by any equivalent their setup supports. Where
nothing resolved there is no prefix to name, and the requirement alone is the
whole remedy.

What remains is unsupported and now says so. Dogfooding is the constraint seen
from inside this repository: `cargo run --bin grove` drives a checkout's embed
while the session's `PATH` still reaches the installed `grove-llm`, and it is the
obvious next move for anyone who has just read why the skill looked stale. That
pairing is announced before the launch, and it is still not prevented, which is
the honest limit of a design that cannot see into an opaque target. The supported
way to run a build is to make it the installed one that a session's `PATH`
resolves first.

## Considered options

- **Keep comparing crate versions.** Rejected because the version demonstrably
  cannot see the case: an edit at an unbumped version moves the methodology while
  the manifest stands still, and this repository sits in exactly that state
  routinely. Reopen never; the hash is strictly finer and costs 64 bytes.
- **Refuse the launch when the probe disagrees.** Rejected because the driver
  cannot observe which `grove-llm` an opaque configured command resolves, so the
  refusal would rest on a measurement of the *driver's* environment: a supported
  wrapper with the correct CLI would be refused before it started, and bare
  `grove` would launch nothing at all rather than launch something imperfect. A
  binary too old to answer the identity flag would be refused for the same
  reason, making the pair unupgradable from inside the loop. Reopen if a
  configured target ever exposes its effective environment or its resolved tool
  paths to the driver, which would turn the proxy into an observation.
- **Drop the driver-side probe and rely on the stamp warning alone.** Rejected
  because the stamp warning speaks from inside the *session's* `grove-llm`, so it
  is silent about exactly the case where that binary is the wrong one: a session
  whose CLI predates the driver warns about nothing while reading a methodology
  the driver just wrote. The probe's line also lands **between** sessions, in
  scrollback nothing overwrites, where a human watching the loop reliably sees
  it. The two are complementary rather than redundant — one is checked before a
  launch, the other on every verb, which is the only one a clobber landing after
  the launch can reach. Reopen if a session's environment is ever observable to
  the driver, which would collapse the two into one.
- **Prefer the sibling of the running executable.** Rejected because the driver
  discards the binary it resolves, so the sibling is checked and the session's
  `PATH` binary is run — and because the sibling is what hides the motivating
  case: `cargo run` builds `grove` and `grove-llm` side by side, so a checkout
  driver's sibling always agrees with it while the session reaches the installed
  CLI. Reopen if the driver ever invokes `grove-llm` itself, which would make the
  sibling the binary that matters.
- **Prepend a one-entry shim directory holding the driver's own `grove-llm` to
  the launched session's `PATH`.** This is the only option that would *prevent*
  the mismatch rather than report it, and it would do so exactly for the targets
  the probe can see — those that inherit the driver's environment. Rejected on
  its merits rather than by citation: it would decide which binary the operator's
  configured command resolves, which is hidden launch policy of the kind
  [complete session configuration](complete-session-configuration.md) exists to
  keep out of the launch — and here the objection is the thing itself, not the
  other decision's wording, because the effect is to make an uninstalled build
  the effective CLI for a session whose owner never chose it. It also prevents
  nothing behind a wrapper that re-derives `PATH`, so it removes neither report.
  Reopen if a supported launch shape is found that cannot install its own build,
  which would leave prevention as the only coherent pairing mechanism.
- **Carry the driver's methodology identity in the session environment and have
  `grove-llm` compare against it.** Rejected because it duplicates what the
  driver-side probe already reports while adding a second place the answer can be
  wrong, and because it arrives after the session has read its methodology — the
  human who can act on a mismatch is the one watching between sessions. The cost
  is not that Grove may set no variable — it already grants a fresh signal
  path — but that this one carries an answer the probe gives earlier. Reopen if
  the driver-side probe is ever shown to be unobservable in a shape where the
  session's own comparison would not be.
- **Deliver the whole methodology in `${prompt}`, removing the second copy from
  the picture.** This *would* close the pairing question — argv cannot be
  clobbered by another build — and it was tried: it is the mandate, and [the
  skill delivers the methodology](skill-delivers-the-methodology.md) reversed it
  on a measured behavioural cost. So the pairing risk is knowingly re-accepted,
  which is what makes it worth reporting twice. Reopen only together with that
  decision; pairing is not a reason to revisit delivery on its own.
