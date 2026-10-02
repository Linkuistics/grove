# Harness selection is owned by policy

Which harness, model and reasoning effort runs a piece of work is decided by one
function the owner writes, and by nothing else. The `harness-dispatch`
executable passes that `select` function what its caller passed, the prompt
included, and acts on what it returns: the command to run, with its provider,
model and effort labels, or a refusal. The executable holds no catalog of
harnesses, no table of kinds and no rule about which choice is allowed. Grove
runs it for every session it launches, lifecycle and standalone alike, and
carries no launch configuration of its own. The
[area specification](../specs/harness-selection-and-execution.md) owns the
protocol, identity, records and delivery contracts. The
[worker and handoff decision](policy-evaluation-precedes-process-replacement.md)
chooses the Rust front process and bundled TypeScript runtime separately from
this boundary.

Grove supplies its already selected session kind, task file, stable task handle
and unchanged prompt, and the session name and the two roots as named
parameters. Callers supply stable task identities as data, so an association
survives a Grove task's retirement and reordering without any filename being
parsed. The discovery guarantee spans sessions in the same live grove and
workspace. Provenance travels as globally unique run identities, so handles need
no namespace. Run and observation records live outside task bodies and survive
the tree's teardown, and a review task carries only its creator reference. There
is no automatic cross-checkout discovery and no post-teardown artifact lookup.

The dependency runs one way. Grove requires the executable and is one caller
among many; the package depends on no Grove package and has no Grove concept in
its core, so ordinary use needs no Grove installation or task-tree conventions,
and whatever Grove needs from it is a capability any caller can use. The
executable is a separate package in this repository and ships through Grove's
installation and release, with everything needed to evaluate TypeScript without
a separately installed runtime. Co-location makes development, integration tests
and deployment easier, and the package can still be extracted into a repository
of its own.

## What each half of the decision costs

**The function returns the command, so the tool guarantees less.** A catalog
that the function named by ID let the tool validate every candidate when the
policy loaded, and made it impossible for a function's result to introduce a
program or an argument. Both are now the owner's function's to hold, with the
type checker and whatever tables it keeps. What that buys is one form and no
vocabulary: a value the caller passes reaches the arguments because the function
puts it there, a value inside an argument is string building, and nothing the
command reports distinguishes a policy that consults a table from one that
evaluates the task. The guarantee that mattered most is restated as guidance
where an owner reads it: a policy that turns outside text into a command chooses
among commands its own code wrote.

**Grove has no launch configuration, so nothing is checked before a launch.**
Commands, bindings, routes, parameters, profiles, the per-checkout delta with
its trackedness refusal, and both `grove config` commands are gone, and Grove
reads no personal file. A kind the policy does not route is caught when its
leaf launches: the launch refuses with a remedy, the leaf stays live, the loop
stops, and rerunning continues. A session that cuts a leaf of an unroutable
kind no longer fails on the spot, and an unattended run can stop on a leaf
authored earlier. That is the accepted cost of one place to decide. An old
configuration file left on disk is ignored without a word, and the release that
removes the configuration breaks every owner who had one.

**Dynamic selection is possible, and only possible.** A policy can hand the
prompt to a deciding agent it starts in the caller's directory and reaps, within
a selection bound the owner sets. Nothing ships that does so, and no shipped
policy calls a model. The prompt is a mandate, so keeping a deciding agent from
executing the task it evaluates is the job of the owner who writes that policy;
the documentation says so. The
[routing research](../research/grove-model-effort-routing.md) motivates the
computation boundary and documents possible experiments; its model comparisons
are priors, not validation of a routing policy.

## What policy owns

The policy owns model preferences, reasoning effort and review-provider rules.
A shipped example review policy requires a different provider from the
artifact's original creator for every review it selects, including
re-invocations. It is activated explicitly through personal policy and tested as
the delivered artifact. Provider is the owner's label on the command a policy
returns, for model origin; it is not a gateway or an identity inferred from the
program or its arguments. The package exposes each recorded run's provider to
TypeScript, and a review names its creator's run, as
[a review carries its creator reference](a-review-carries-its-creator-reference.md)
records; today's policy cannot reconstruct the original creator's identity. An
explicit owner declaration remedies missing execution records and is visibly
labelled as declared; absence of both forms stops review. This is a simple
original-creator comparison, with no separate model-inequality check or
multi-author exclusion set. The generic executable contains no review-specific
provider rule. The supplied Grove policy/context adapter interprets review
relationships; independent callers can supply equivalent generic associations.

Policy ownership also determines which code may run. Personal policy is loaded
by default. Repository TypeScript executes only when explicitly selected through
personal policy or a `--config` argument. Merely cloning or entering a
repository grants no authority to run its policy, and the host discovers no
ambient repository configuration on its behalf. What an owner sets about every
invocation, the selection bound and the environment granted to the policy among
them, lives in a settings file with the same personal authority, and is the
same for every kind. A missing
policy stops with a diagnostic: the executable invents no default, and a policy
arrives only by the owner writing one or running the subcommand that installs
the shipped sample.

One checkout gets a different selection from another through policy code. A
supported helper reads a local choice file, and that file can only name options
the owner's policy already offers. It can introduce no program and no argument,
which is why it needs no version-control check where the configuration delta
needed one.

Selection helpers are not granted Grove completion authority, and the final
harness retains the driver's foreground-job contract
([the launched child is a job](the-launched-child-is-a-job.md)); the
specification owns the bounds, cancellation and authority contracts that
preserve this. Trusted TypeScript is executable configuration, not a sandbox
against a hostile local owner. A confined standalone invocation selects outside
its sandbox, through inspection, and runs only the harness inside it. Grove's
runner launches the file inspection reports, so `harness-dispatch` launches
nothing there and records no run.

## Considered options

- **Put selection rules in Grove.** That would couple model choices, provider
  conventions and selection experiments to Grove releases, and make reuse by
  other callers harder. Reconsider only if independent use is abandoned.
- **Keep Grove's command configuration as the integration point**, with the
  dispatcher reached from an owner-written command definition, or kept as a
  per-kind override beside a built-in route. Rejected because it leaves two
  places that decide what runs, and keeps a resolver, an inspector and a
  security check alive to serve the smaller of them. Reopen only if Grove must
  launch something no policy can express.
- **Keep a catalog the function names by ID.** It keeps load-time validation and
  the no-new-argument guarantee, and it needs a slot vocabulary that grows with
  every value a caller wants in an argument, a second static form, and a
  tool-level rule for explicit choices. Rejected for the simpler contract.
  Reopen if owners are seen to build commands from text they did not write.
- **Check a kind's route before writing its leaf.** A query could answer for a
  policy that consults a table, and no query can answer for one that evaluates
  the task. Rejected because it would keep a check that is true only of some
  policies.
- **Refuse while an old configuration file is present, or convert it.** Rejected
  by the owner in favour of ignoring the file and shipping a sample that is a
  conversion of their own configuration.
- **Install the sample when no policy exists.** Rejected because the sample runs
  a harness with approvals off, and an invocation that found no policy would
  have launched it unread. Installation is an explicit subcommand.
- **Let the executable discover a per-checkout override file.** Rejected because
  it reverses the rule that a repository grants nothing. The helper is taken up
  only where the owner's policy calls it.
- **Have the executable launch a confined invocation too**, by running a
  launcher the caller names ahead of the selected command. That would give a
  standalone invocation one launch path and a run record. The sandbox has to
  name the resolved program, so the launcher would be a Grove helper, and the
  runner's environment scrub would have to move behind selection so that the
  policy keeps the owner's grants. Rejected because that is a launcher option,
  a helper and a moved scrub where inspection needs none of them, and what
  inspection gives up is the record. Reopen when the executable supervises
  the harness itself, as the
  [worker and handoff decision](policy-evaluation-precedes-process-replacement.md)
  leaves open, or when a standalone invocation needs a run record.
- **Let owner settings differ by kind.** An owner's separate Grove command
  definitions could carry different flags for different kinds. Rejected because
  it puts a table of kinds back in the tool. Each bound is a ceiling, so one
  value that admits the slowest selection serves the rest, at the cost of a
  shorter failure bound for the others, and a record directory per kind would
  split the store a run lookup reads. Reopen if an owner needs a grant or a
  bound that one kind must not share.
- **Hard-code review diversity in the executable.** The rule is an owner's
  policy and belongs with the rest of selection computation. Provenance is data
  the executable makes available, not a reason to give the core review
  semantics. Reconsider only if the tool acquires an explicit requirement to
  enforce this rule independently of its owner's configuration.
- **Keep the executable in a repository of its own.** Independent ownership
  stays possible, but development, integration testing and deployment benefit
  from one repository and release route. Extract when those benefits no longer
  outweigh independent maintenance and delivery.
