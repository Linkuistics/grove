# Grove policy development

This directory owns the compiler for Grove's readable policy assets under
`assets/ravel-policy/`. It is development tooling and does not ship in Grove's
release archives. The independent Ravel installation supplies the public SDK
and static-example declarations; no Ravel checkout or private worker source is
needed here.

The repository's root `mise.toml` pins the Bun runtime.
`scripts/pinned-bun.sh` runs that version for the dependency task and the
typecheck, through mise when `PATH` offers another Bun. The package pins
retain `@types/bun` 1.4.3 and TypeScript 7.0.2 from the pre-extraction
compiler, including its locked transitive packages and strict compiler flags.
`@types/bun` is a lockfile pin, and it does not set the runtime version. The helper resolves declarations from the explicitly supplied
Ravel front's canonical installation prefix. Ordinary relative policy imports
and type fixtures stay in the same compilation.

```sh
task ravel:policy:typecheck RAVEL=/opt/homebrew/bin/ravel
task ravel:policy:compiler-test RAVEL=/opt/homebrew/bin/ravel
task ravel:review RAVEL=/opt/homebrew/bin/ravel
```

`ravel:policy:deps` installs the frozen compiler graph. The compiler test runs
the actual Taskfile/helper in a fragment containing no `crates/` directory,
rejects an invalid policy type and refuses a prefix without installed SDK
declarations. The normal policy/review checks include that test plus real
installed-worker fixtures with fake agents.

Delivery contraction removed the bundled dispatch package and its compiler/
compatibility checks. This directory is Grove's policy compiler; executable
runtime tooling belongs to independently maintained Ravel.
