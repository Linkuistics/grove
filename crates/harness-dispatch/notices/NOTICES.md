# harness-dispatch notices

harness-dispatch is part of Grove and is licensed under the Apache License 2.0;
the `LICENSE` file distributed with it holds the text. Two third-party
components are compiled into its executables.

## Bun 1.4.2, inside `harness-dispatch-policy`

The policy worker is compiled with Bun 1.4.2 and carries Bun's runtime inside
it: the `bun` executable from the `@oven/bun-<platform>` 1.4.2 package that the
build pins by digest. Bun is MIT-licensed and statically links further
libraries, among them JavaScriptCore and WebKit under the LGPL 2.
[`bun-LICENSE.md`](bun-LICENSE.md) is Bun's own statement of those components,
their licences and where to obtain the patched WebKit source. It is copied
unchanged from `LICENSE.md` at Bun's `bun-v1.4.2` tag.

## SQLite, inside `harness-dispatch`

The front executable compiles in SQLite, from the amalgamation that the
`libsqlite3-sys` crate bundles. SQLite is in the public domain;
[`sqlite.md`](sqlite.md) holds its dedication.
