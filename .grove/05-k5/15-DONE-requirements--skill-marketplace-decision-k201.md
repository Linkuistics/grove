# skill-marketplace-decision-k201

## Goal

Stop before `skill-marketplace-k185`. The human is not sure that this
workstream needs a separate skills marketplace. Get the human's decision
before any marketplace work starts.

## Context

- The human inserted this stop during `design-native-namespace-k200`, with
  the words "Please insert a stop before k185 - I'm not sure I want a skills
  marketplace".
- The root brief's "Repository destinations" requires the marketplace-only
  repository `~/Development/Delivery/skill-marketplace`. Its "Done when" says
  that the marketplace references the source repositories.
- `skill-marketplace-k185` states the planned marketplace and its open
  questions in "On the horizon".
- Native installation remains a supported user choice in the root brief.
  Today, Grove's `.claude-plugin/marketplace.json` lists the packs with
  `git-subdir` sources. The native trials of the moves install from it.
- `moves-handoff-k187` compares the marketplace pins with the owner's pins.

## Done when

- The human chooses one route, in their own words. Possible routes:
  1. Keep `skill-marketplace-k185` as planned.
  2. Keep the marketplace in Grove's `.claude-plugin/` and prune the
     separate repository.
  3. Drop native marketplace distribution for the packs, and use only
     Ravel's directory delivery for the owner.
  4. Another route that the human names.
- The root brief, the brief of `repository-moves-k5` and the affected live
  leaves agree with that decision. These include `skill-marketplace-k185`,
  `moves-handoff-k187` and any rollout text. Prune a node only after the
  human approves it.

## Notes

Before the interview, state the effect of each route on a native user, on
the owner's cutover and on the marketplace trials. Give a recommendation with its evidence.

## Route effects

The page "Install and manage plugins" of the Claude Code documentation
(<https://code.claude.com/docs/en/discover-plugins>) gives three facts:

- When a user removes a marketplace, Claude Code uninstalls every plugin
  from it.
- A plugin installs as `<plugin>@<marketplace>`, and `<marketplace>` is the
  registered name. One name holds one registration.
- `claude plugin marketplace update` without a name refreshes every listing.
  It leaves each installed plugin at its current version.

The owner's Claude Code registers `linkuistics` from `Linkuistics/grove`,
with automatic updates on. Its three Linkuistics plugins come from Grove
commit `238c3a839ab4`. An earlier marketplace repository,
`Linkuistics/skills`, moved into Grove. Its description still tells each
user to remove the marketplace and add it again. The six pack READMEs tell
a user to add `Linkuistics/grove`.

### Route 1: the separate repository

- Native user: each user adds the new marketplace. Under the name
  `linkuistics`, the user first removes the old registration, which
  uninstalls every plugin from it. Under a new name, every plugin identifier
  changes. Each of the six packs needs a new README commit.
- Owner's cutover: the owner design works as written only under a new name.
  Preparation then adds that marketplace and installs the two skill-free
  plugins disabled. The frozen `linkuistics` registration stays until purge.
  Under the name `linkuistics`, the owner cannot add the new registration
  before purge.
- Finish: the core entry pins a Grove commit. The released commit exists
  only inside finish, so the marketplace needs a commit there, or the record
  names the difference.
- Trials: each of the ten plugins installs from the new marketplace in an
  isolated home. The new repository checks its own pins. Grove adds a pin
  record of its own for `check:packs`.

### Route 2: Grove keeps the marketplace

- Native user: the registration and every plugin identifier stay. After
  finish, a user installs each new pack plugin once, as under every route.
  The READMEs of Grove and of the packs already give those commands.
- Owner's cutover: Grove's marketplace lists `technical-communication-hooks`
  and `guardrail` on this line. They reach the owner's registration only when
  finish moves `main`. Order 6 freezes that registration, so the owner design
  needs an amendment. The likely amendment installs both plugins inside
  finish, after the release and before the backup of switch step 8. A staging
  measurement must show that the refresh and the two installations change no
  version that finish loaded and write no orphan marker.
- Finish: the core entry stays `./plugins/grove`. It names the released
  commit without a pin step.
- Trials: no relocation trial is owed. A final check can install the ten
  plugins from a plain checkout of the candidate commit, in an isolated home.

### Route 3: no native marketplace for the packs

- Native user: `/plugin install` no longer supplies the packs. The manual
  installer and Ravel remain. This reverses the root brief and issue #25,
  which keep the marketplace as a supported user choice.
- Owner's cutover: Ravel installs no hook. The two skill-free plugins still
  need a listing, so the owner keeps a marketplace. The design's last native
  return installs the current marketplace plugins, and that return loses the
  pack entries.
- Trials: the relocation trial goes, and the recorded native trials describe
  a withdrawn route. The listing of the two plugins still needs a trial. The
  six packs and TestAnyware each need a new README commit.

### Recommendation

Route 2. This line already holds its state. Grove's marketplace lists all ten
plugins with pinned `git-subdir` sources. `check:packs` reads those entries as
Grove's record of each pack revision. The guidance of Grove and of each pack
already names Grove's marketplace.

Route 2 moves no native user, adds no step to finish and changes no pack.
Its cost is one owner-design amendment and one staging measurement. Route 1
keeps the owner design unchanged. In return, every native user reinstalls,
each pack changes its README, and Grove keeps a second record of the pins.

## Decisions (running log)

**The route (2026-11-06).** The human chose route 3, "No native marketplace
for packs", after the comparison above. The recommendation was route 2. The
packs leave native Claude Code marketplace distribution. The separate
repository of `skill-marketplace-k185` is not built.

The chosen option stated two consequences. The two hook plugins still need a
listing. The last native return of the owner design loses its route. The
human then wrote "I think maybe installation purely through homebrew?".

**Homebrew installation (2026-11-06).** Homebrew installs every Linkuistics
skill source in this workstream, the owner's machine included. Formulae
install core inside Grove's keg, TestAnyware's skill inside TestAnyware's keg,
and each pack. The human wrote: "I would rather use homebrew and let mise run
homebrew than do anything for mise specifically." The owner's mise
configuration therefore only declares formulae in `[bootstrap.packages]`. The
reviewed pack checkouts in mise's `[bootstrap.repos]` are not built.

**Third-party sources (2026-11-06).** The same words settle the two
third-party Ravel sources, superpowers and quint-llm-kit. Formulae in the
Linkuistics tap install them, because a `[bootstrap.repos]` entry is specific
to mise. The human received this reading and can correct it.

**Outside this grove (2026-11-06).** The human wrote: "I do want to
re-organise the Ravel/*Skills/grove/Ravel-Strategies into a single repo, but
not in this grove." No leaf of this tree does that reorganisation.

**Packaging (2026-11-06).** The human chose "Inside Grove's formula". Grove's
keg carries core and the five domain packs: requirements, design,
engineering, research and writing. Each pack is at the commit that Grove's
pack check accepted.

A change to a pack reaches users through a Grove patch release. Personal and
the two third-party sources get formulae of their own. A user who wants only
craft skills installs Grove, and with it Ravel.

**Hook setup (2026-11-06).** The human chose "Register from the keg". Grove's
keg ships the Claude Code and Codex marketplace files and the Pi extension of
the writing hooks. The personal keg ships the Claude Code marketplace file of
`guardrail`. One documented command for each harness registers each file by
its Homebrew path. No remote listing remains.

<!-- tc: depart long-sentence because these sentences quote the human verbatim, and the quoted words carry the decision -->
<!-- tc: depart synonym-rotation because "modify" occurs only inside the human's quoted words -->
**Ravel's later role (2026-11-06).** The human wrote that Ravel "will be used
for *all* llm invocation, and so it can configure/modify-the-prompt in any way
required". The human sees skills installed "*into ravel*", with Ravel taking
"care of the per-harness use of this skills, including any configuration
required to use them". The human called this "a 'universal' skills provider
that abstracts over the harnesses it invokes" and "a somewhat separate aspect
of ravel".

The human added: "Ultimately we might define our own 'super-skill' format that
includes executable plugins for various UI features needed by skills etc."
This workstream builds neither the provider nor the format. Its choices keep
both open: the hook adapters stay in their packs, and each formula installs at
a stable Homebrew path. The human received this reading and can correct it.

**Skills outside Ravel (2026-11-06).** The human chose "Ravel only". Homebrew
installs the skills, and Ravel delivers them. Grove's formula depends on
Ravel, so every Grove user has Ravel. The native skill routes retire: the
manual installer's skill links and the root brief's promise of native
installation. Only the hook adapters keep a native registration, until Ravel
can configure hooks.

A harness session that its user starts directly sees no Linkuistics skills.
That user can still read the folders that `ravel skills resolve` lists. The
installer's link of the Pi extension becomes one documented command.

**Issues (2026-11-06).** At the human's request, Linkuistics/Ravel#4 holds
the universal skills provider and the "super-skill" format.
Linkuistics/grove#28 holds the single-repository reorganisation. This
workstream depends on neither.

**Test seams (2026-11-06).** The human agreed six seams. Four existing seams
move to Homebrew paths. Two of them are the release archive and formula tests
and Grove's pack check. The other two are the real sessions through Ravel and
the owner design's staging trials.

**Shared understanding (2026-11-06).** The human agreed with the summary of
these decisions. That agreement approved four tree changes. Prune
`skill-marketplace-k185` with its planning leaf. Insert the node
`homebrew-skill-delivery`, with a design leaf and then a planning leaf, before
`moves-handoff-k187`. Update the root brief, the brief of
`repository-moves-k5`, k187, the brief of `discovery-rollout-k6` and k93.

Two seams are new. Each hook adapter registers from
a keg in an isolated home. One measurement shows that the hook installation
inside finish changes no plugin that finish loaded.
