# External pack contract — views

The [external pack contract](../../specs/external-pack-contract.md) owns the
decisions. These views show the same design. The contract is proposed, and a
`review-design` leaf reads it before the runner work.

Run `task design:external-pack-contract` from the repository root, then open
[the viewer](http://127.0.0.1:8793/). `PORT=...` chooses another port. The
server exposes only this directory. The `.mmd` files and `diagrams.json` are the
editable sources. The viewer loads pinned Mermaid 12.0.0 from jsDelivr, so it
needs network access.

Each graph has one dark node labelled `Start here`, or one for each independent
run.
Numbered edges give the order of the steps. The views show files and checks,
not deployment containers.

| View | Contract illustrated |
| --- | --- |
| [One source directory, three readers](http://127.0.0.1:8793/#diagram-layout) | `plugins/research` is the Ravel source directory and the Claude Code plugin root. The repository root holds what the pack does not deliver. |
| [Directed loads of the research kinds](http://127.0.0.1:8793/#diagram-edges) | Each research kind loads the spine across packs, and the pair loads its family skill. A missing loaded skill sends the session to the launch prompt's rule. |
| [Two checks, each against a pinned revision](http://127.0.0.1:8793/#diagram-conformance) | Grove checks against the pack commit of its marketplace entry. The pack checks against a recorded Grove commit and runs that commit's runner. |
| [Three routes from one published commit](http://127.0.0.1:8793/#diagram-distribution) | The marketplace entry and its dependency, the Ravel source record, and the installer's `--pack` argument. |
