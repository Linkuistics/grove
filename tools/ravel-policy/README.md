# Grove policy development

This directory owns the compiler for Grove's readable policy assets under
`assets/ravel-policy/`. It is development tooling and does not ship in Grove's
release archives. The independent Ravel installation supplies the public SDK
and static-example declarations; no Ravel checkout or private worker source is
needed here.

The pins retain Bun 1.4.2, `@types/bun` 1.4.2 and TypeScript 7.0.2 from the
pre-extraction compiler, including its locked transitive packages and strict
compiler flags. The helper resolves declarations from the explicitly supplied
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

Until delivery contraction, the retained dispatch package has its own compiler
and compatibility checks. Its tooling is not a dependency of these policy tasks.
