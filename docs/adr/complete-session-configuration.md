# Complete session configuration

Every configured Grove session kind resolves to one complete, inspectable command
before it can be launched. Personal policy may assemble that command from named
parameterized definitions, bindings and ordered profiles. Completeness belongs
to the **resolved command**, with the origin of each contributing value visible,
rather than to one source string. Grove executes its expanded argv directly and
does not infer a harness, model, approval policy or hidden argument.

The [modular configuration spec](../specs/modular-configuration.md) owns the
grammar and resolution rules. Composition is explicit: a personal base, an
ordered selected profile list with ordered includes, and direct local overrides.
A parameter changes the contents of an already parsed argument, never the number
or boundaries of arguments. Runtime slots remain consumer-declared. Inspection
and launch share the compiled configuration rather than maintaining separate
interpretations of it.

Profile composition applies each occurrence, including repeated and diamond
includes. Global deduplication is rejected: it would let an earlier override
survive a later explicitly selected reuse of its base. Configurations authored
against that order would change meaning under a visited-set optimisation.

Parameter specificity survives composition: a route override beats a shared
command value even when the shared assignment is later. A single chronological
winner across scopes is rejected because a shared edit must leave explicit
per-kind exceptions intact. `unset` removes the exception to expose inheritance.
These two choices bind configuration meaning; changing either requires an
explicit compatibility decision, not a resolver optimisation. The spec owns
their fold mechanics and worked examples.

The trade-off is deliberate. Requiring a whole independently authored template
for every kind made the executable easy to read locally, but repeated almost
identical commands and made a small experiment a many-entry edit. Shared
definitions keep later changes live. Their cost is a resolver and provenance:
one source line no longer explains every argument, so the inspection surface
must. The compiler must produce a complete command before the runner sees it.

Document syntax, shapes and duplicate declarations are always validated. The
selected combination is semantically validated after composition, allowing
partial building blocks and unfinished inactive profiles. Unsupported top-level
declarations fail structurally; only the modular wrapper form is accepted.
**Presence stays per-kind and just-in-time:** before Grove writes or launches kind K, K must resolve. Grove
holds no closed set of kinds and supplies no catch-all route.

The source and authority rule belongs to
[the untracked configuration delta](untracked-configuration-delta.md): global
policy always participates; at most one local file overrides it; and only an
explicit target in active personal policy can admit a kind. A selection can
activate a personal profile but a local route cannot independently introduce a
kind. This is an authority rule over kinds, not a restriction on which executable
an admitted local override may select.

Before spawning the foreground command, Grove removes stale Grove control values
and grants its fresh signal path. Other caller environment values are preserved,
including Git repository selectors. Driver-internal VCS commands instead scrub
repository selectors and anchor themselves to the selected working tree.

## Considered options

- **Whole templates only.** Rejected because reuse and small configuration
  experiments now require changing individual values. Reconsider only if the
  resolver and inspection cannot make a launch's complete provenance legible;
  duplicating policy is not the default remedy for composition.
- **A harness-aware primary model with inferred family routes.** Rejected
  because executable flags and routing are the owner's policy, and an unlisted
  kind must remain unconfigured. Reopen only if Grove deliberately becomes a
  harness-aware router.
- **Shell evaluation or string substitution before splitting.** Rejected
  because either lets a value change argument boundaries or introduces another
  execution language. Explicit wrappers remain available. Reopen only if direct
  argv plus wrappers cannot express a required launch.
- **Validate every profile in isolation.** Rejected because partial profiles are
  useful building blocks and unfinished inactive experiments must coexist with
  working selections. Structural validity still binds the whole document;
  semantic validity binds the active composition.
- **Compare research targets to enforce vendor diversity.** Rejected because
  opaque commands and wrappers do not expose a reliable target identity. The
  owner may express separate bindings; Grove does not interpret their names or
  arguments. Reopen only if target identity becomes an explicit comparable part
  of configuration without harness inference.
