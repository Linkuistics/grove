# absent-skill-destination-k8

## Goal

Make the driver report, before every launch, when **no known harness root exists**
— and launch anyway.

## Why

If no known harness root exists, nothing is provisioned, and after the cutover a
session receives a core pointing at a skill that is not there. That is a **total
failure that is currently silent**.

## The rule this follows

Grove **reports and never refuses**, on the line its surface already draws: it
*stops* on what governs its own operation, and *reports* what it can only predict
about a session's environment. Which harness an opaque configured command reaches
is firmly the latter — Grove executes the configured command directly and cannot
know what it is.

## Requirement

> The driver SHALL report, before every launch, when no known harness root exists,
> and SHALL launch anyway.

- **No harness installed** — no known harness home marker present → the driver
  prints one diagnostic naming the roots it looked for, and the launch proceeds.
- **At least one harness installed** — any known harness root exists → **nothing is
  printed.** Absence of a destination is the only claim on offer, and it cannot be
  made about a machine that has one.

That second scenario is the one worth being careful about: a report that fires when
a destination *does* exist is a different, weaker claim (*we do not know whether
your harness reads it*), and the driver has no standing to make it.

## Done when

Both scenarios are covered by tests, the diagnostic names the roots by absolute
path, and no launch is refused on this ground.

## Notes

Positioned before `guaranteed-core-k9` deliberately: it is small, it is
independently useful **today** — provisioning already runs from `src/launch.rs`
and can already find nothing — and it means the cutover does not ship a known
silent total failure for even one increment.

`src/provision.rs` and the harness registry (`src/harness.rs`) are where the known
roots are enumerated. The registry is what "supported" means; see the spec's
*Provisioning and build pairing return unchanged*.

The neighbouring unsupported shape is **named and not solved**, and this leaf does
not try to: a launch target that can receive a large `${prompt}` but cannot read a
provisioned skill now gets a session with no methodology at all, and the driver
cannot detect it for the same reason it cannot detect the harness.
