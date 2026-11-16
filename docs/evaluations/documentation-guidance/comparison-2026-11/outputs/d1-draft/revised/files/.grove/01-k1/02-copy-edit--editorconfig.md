# editorconfig

## Goal

Run the `copy-edit` stage on the guide in `docs/editorconfig/guide/`.

## Context

The `draft` stage wrote the three pages: `README.md`, `explanation.md` and
`reference.md`. The document has no prose contract, so house style applies.

The checker `ste-lint.py` reports no hard findings on the three pages. It
reports 25 advisory findings. Most are the passive "are omitted" in the
example labels of `reference.md`, such as "The preamble and other sections are
omitted." The labels use one wording on purpose, so that a reader can find
what each fragment leaves out. If you change the wording, change every label
the same way.

The terms in the brief's term risks need one name each throughout: the `root`
key against the filesystem root directory, "core", "section" and "pair". The
draft keeps these apart. Check that the pages use each one consistently.

The reference states several gaps in the specification as gaps, such as the
inclusivity of `{num1..num2}`. These are technical statements. Do not resolve
them in the wording.

## Done when

- The `copy-edit` stage's work on the guide is complete, to its contract.
- The gate the root brief names passes.
