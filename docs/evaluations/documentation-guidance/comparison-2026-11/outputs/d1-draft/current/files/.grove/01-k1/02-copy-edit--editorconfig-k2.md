# editorconfig-k2

## Goal

Run the `copy-edit` stage on the document this node's brief describes.

## Context

The node brief names the document's contract and carries what earlier stages
handed forward. The contract declares no prose contract. Check terminology
against the contract's term risks:

- The draft uses "pair" only, and says once in `README.md` that the source also says "rule".
- The draft separates "the `root` key" from "the root directory of the filesystem". Keep that separation in every sentence that says "root".
- "Section" has two meanings in the draft. It names the EditorConfig construct, and it also names a part of a page, for example "The glob forms section gives the full rule" in `explanation.md`. Decide whether the second meaning stays.

The lint run by the draft reports nine advisory passive-voice hits and no hard hits.

## Done when

- The `copy-edit` stage's work on the document is complete, to its contract.
- The gate the root brief names passes.
