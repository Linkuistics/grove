# Harness selection is owned by policy

One function the owner writes decides which harness, model and reasoning effort
run a piece of work, and nothing else decides it. The `ravel`
executable passes that `select` function what its caller passed, the prompt
included. The executable acts on what the function returns: the command to run,
with its provider, model and effort labels, or a refusal. The executable holds no
catalog of harnesses, no table of kinds and no rule about which choice is
allowed. Grove runs it for every session it launches, lifecycle and standalone
alike, and carries no launch configuration of its own.

The [supervision decision](dispatch-supervises-the-harness.md) owns the
Grove caller boundary. The independent
[Ravel specification](https://github.com/Linkuistics/Ravel/blob/v1.0.0/docs/specs/session-execution.md)
owns executable/runtime behaviour. The
[worker decision](policy-evaluation-precedes-the-launch.md) records the native
selection boundary. [Supervision](dispatch-supervises-the-harness.md) records
why the selected process remains a managed job.

Grove supplies kind, task file, stable handle and unchanged prompt as ordinary
caller data, with no parameter. Lifecycle work runs in the workspace root, and
standalone work runs in its staged cwd. An owner policy can derive workspace
grants from `request.cwd` and `.jj/repo`. No jj/task convention belongs to Ravel.
Grove's readable assets interpret task relationships and preserve inline creator
receipts. Ravel supplies no run lookup, persistent records or historical API.

The dependency runs one way. Grove requires the public CLI of an independently
installed Ravel. Grove resolves it once from PATH and checks its version. Ravel
needs no Grove installation. Its front and compiled worker ship independently,
without host Bun/Node.

Grove keeps its own local runner and caller-owned policy assets.
Grove archives contain its executables and caller assets. The Homebrew formula
depends on separate Ravel. Grove bundles no executable worker and no shared Rust
dependency ([delivery boundary](../grove-delivery-contraction.md)).

## What each half of the decision costs

**The function returns the command, so the tool guarantees less.** A catalog
that the function named by ID let the tool check every candidate when the
policy loaded. The same catalog made it impossible for a function's result to
introduce a program or an argument. The owner's function now holds both, with the
type checker and whatever tables it keeps.

What that buys is one form and no vocabulary. A value the caller passes reaches
the arguments because the function puts it there. A value inside an argument is
string building. Nothing the command reports distinguishes a policy that consults
a table from one that evaluates the task. The guidance where an owner reads it
restates the guarantee that mattered most. It says that a policy that turns
outside text into a command chooses among commands its own code wrote.

**Grove has no launch configuration, so Grove checks nothing before a launch.**
Commands, bindings, routes, parameters, profiles, the per-checkout delta with
its trackedness refusal, and both `grove config` commands are gone. Grove
reads no personal file. Grove catches a kind that the policy does not route when
its leaf launches. The launch refuses with a remedy, the leaf stays live, and the
loop stops. Rerunning continues.

A session that cuts a leaf of an unroutable kind no longer fails on the spot.
An unattended run can stop on a leaf authored earlier. That is the accepted cost
of one place to decide. Grove ignores an old configuration file left on disk
without a word. The release that removes the configuration breaks every owner
who had one.

**Dynamic selection is possible, and only possible.** A policy can hand the
prompt to a deciding agent. The policy launches that agent in the caller's
directory and reaps it, within a selection bound the owner sets. Nothing ships
that does so. No shipped policy calls a model.

Because the prompt is a mandate, the owner who writes that policy must keep a
deciding agent from executing the task it evaluates. The documentation says so.
The [routing research](../research/grove-model-effort-routing.md) motivates the
computation boundary and documents possible experiments. Its model comparisons
are priors, not validation of a routing policy.

## What policy owns

The policy owns model preferences, reasoning effort and review-provider rules.
A shipped example review policy requires a different provider from the
artifact's original creator for every review it selects, including
re-invocations. It is activated explicitly through personal policy and tested as
the delivered artifact. Provider is the owner's label on the command a policy
returns, for model origin. Provider is not a gateway or an identity inferred from
the program or its arguments.

The opt-in Grove provenance wrapper preserves the actual selected provider in
a canonical receipt. A review carries that receipt or an owner declaration, as
[a review carries its creator reference](a-review-carries-its-creator-reference.md)
records. Today's policy cannot reconstruct the original creator's identity. An
explicit owner declaration remedies missing caller evidence and is visibly
labelled as declared. Absence of both forms stops review.

This is a simple
original-creator comparison, with no separate model-inequality check or
multi-author exclusion set. The generic executable contains no review-specific
provider rule. The supplied Grove policy/context adapter interprets review
relationships. Independent callers can supply equivalent generic associations.

Policy ownership also determines which code may run. Personal policy is loaded
by default. Repository TypeScript executes only when explicitly selected through
personal policy or a `--config` argument. Merely cloning or entering a
repository grants no authority to run its policy. The host discovers no ambient
repository configuration on its behalf.

An owner sets some things about every invocation, among them the selection bound
and the environment granted to the policy. These live in a settings file with the
same personal authority, and they are the same for every kind. A missing
policy stops with a diagnostic. The executable invents no default. A policy
arrives only when the owner writes one or explicitly copies Grove's readable
sample.

One checkout gets a different selection from another through policy code. A
supported helper reads a local choice file, and that file can only name options
the owner's policy already offers. It can introduce no program and no argument,
which is why it needs no version-control check where the configuration delta
needed one.

Selection helpers hold no authority to end a run. The harness is Ravel's own
foreground job
([the launched child is a job](the-launched-child-is-a-job.md)). The
specification owns the bounds, cancellation and authority contracts that
preserve this. Trusted TypeScript is executable configuration, not a sandbox
against a hostile local owner.

Grove's confined standalone invocation uses
`ravel start-session --confine`. In that mode, Ravel selects outside the
sandbox, with the owner's grants and bounds, and confines only the managed
process. That process receives its own ephemeral ending context, with no run ID
or persistent record. Grove scrubs enclosing session authority before invoking
Ravel. Ravel isolates policy evaluation and supplies fresh ending context to the
managed process.

Grove's optional wrapper supplies caller-owned provider receipts separately.
The public CLI carries confinement and result transport, so Grove needs no
private worker protocol or launch hook.

## Considered options

- **Put selection rules in Grove.** That would couple model choices, provider
  conventions and selection experiments to Grove releases, and make reuse by
  other callers harder. Reconsider only if independent use is abandoned.
- **Keep Grove's command configuration as the integration point.** Either an
  owner-written command definition would reach the dispatcher, or the dispatcher
  would stay as a per-kind override beside a built-in route. Rejected because it
  leaves two places that decide what runs. It also keeps a resolver, an inspector
  and a security check alive to serve the smaller of them. Reopen only if Grove must
  launch something no policy can express.
- **Keep a catalog the function names by ID.** It keeps load-time validation and
  the no-new-argument guarantee. It needs a slot vocabulary that grows with
  every value a caller wants in an argument. It also needs a second static form
  and a tool-level rule for explicit choices. Rejected for the simpler contract.
  Reopen on evidence that owners build commands from text they did not write.
- **Check a kind's route before writing its leaf.** A query could answer for a
  policy that consults a table, and no query can answer for one that evaluates
  the task. Rejected because it would keep a check that is true only of some
  policies.
- **Refuse while an old configuration file is present, or convert it.** The owner
  rejected it in favour of ignoring the file and shipping a sample that is a
  conversion of their own configuration.
- **Install the sample when no policy exists.** Rejected because the sample runs
  a harness with approvals off, and an invocation that found no policy would
  launch it unread. The owner explicitly reads and copies Grove's
  readable assets, then imports them from personal policy. Neither executable
  installs a policy automatically, and Ravel has no installation subcommand.
  Reopen only if an explicit owner-reviewed setup workflow becomes a requirement.
- **Let the executable discover a per-checkout override file.** Rejected because
  it reverses the rule that a repository grants nothing. The helper takes effect
  only where the owner's policy calls it.
- **Let owner settings differ by kind.** An owner's separate Grove command
  definitions could carry different flags for different kinds. Rejected because
  it puts a table of kinds back in the tool. Each bound is a ceiling, so one
  value that admits the slowest selection serves the rest. The cost is a
  longer wait before failure for faster routes. No record directory or store
  participates in these settings. Reopen if an owner needs a grant or a
  bound that one kind must not share.
- **Hard-code review diversity in the executable.** The rule is an owner's
  policy and belongs with the rest of selection computation. Grove preserves
  the selected provider in caller-owned receipts. Ravel supplies no provenance
  store or review lookup. Reconsider only if the tool acquires a requirement to
  enforce this rule independently of its owner's configuration.
- **Bundle the executable with Grove's repository and release.** A shared
  route simplifies development and delivery but couples independent callers to
  Grove's packaging and versions. Ravel has its own repository, release tasks,
  archives and Homebrew formula. Grove checks the public CLI contract at runtime.
  Reopen only if independent use and delivery cease to be requirements.
