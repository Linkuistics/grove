# Homebrew installs skills and Ravel delivers them

Homebrew installs every Linkuistics skill source, and Ravel is the only route
that delivers a skill to a session. Grove's formula carries core and the five
domain packs, each at the commit that Grove's pack check accepted. The personal
pack, superpowers, quint-llm-kit and TestAnyware's skill have formulae of their
own. Grove's binary installs no skill and resolves no skill location. The
[delivery specification](../specs/codex-skill-provisioning.md) owns the layout,
the setup and the acceptance.

**Homebrew installation is the whole skill setup.** Each formula links its
sources into `$(brew --prefix)/share/ravel/skills/`, and Ravel 2.0.0 reads that
directory when its settings name no sources. Homebrew cannot write into HOME,
so no setting or link there could make `brew install` enough. In return, Ravel
knows one location in the Homebrew tree, and a source installed another way
needs explicit settings. Links in `~/.config/ravel/skills/` were rejected,
because they need a step outside Homebrew.

**Grove's formula couples Grove's release to the packs.** A change to a pack
reaches users only through a Grove patch release. In return, each Grove
version installs one aggregate of core and the packs, and Grove's checks read
that aggregate before the release. A formula for each pack was rejected. The
packs would then release apart, and no check would read the combination that a
user installed.

**Native skill installation retires.** No remote marketplace lists a pack, and
no installer links a skill. A harness session that its user opens directly
sees no Linkuistics skill. That user can read the folders that
`ravel skills resolve` lists. In return, each harness that Ravel launches reads
the same bytes, and the methodology keeps one route working.

The design rejected native installation beside Ravel. Two routes let one skill differ
between harnesses, as the owner's baseline showed. It also rejected a separate
marketplace repository and Grove's own marketplace, because each keeps that
second route.

**Hooks keep a native registration.** Ravel installs no hook. Each keg that
holds a hook adapter ships the marketplace file of that adapter. Each harness
registers the file from the keg's Homebrew path by a documented command. This
lasts until Ravel can configure hooks, which Linkuistics/Ravel#4 holds.

**Nothing writes a native skill installation automatically.** Grove's startup
and upgrades write, repair and remove no native skill entry. Existing
installations stay until the user removes them through the cleanup procedure of
the delivery specification. That includes the Codex snapshots of Grove 22.8 and
earlier.

An automatic removal would break readers whose lifetime Grove cannot
establish. An embedded core-only fallback in the binary stays rejected. It
cannot supply every kind or an external resource, and it would add a second
source beside the keg.

Reopen this decision if Ravel cannot deliver to a harness that the owner's
policy selects. Reopen it also if pack changes must reach users faster than
Grove patch releases allow.
