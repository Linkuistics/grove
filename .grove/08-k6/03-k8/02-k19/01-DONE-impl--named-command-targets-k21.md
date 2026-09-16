# named-command-targets-k21

## Goal

Reuse parameter-free named commands through bindings and routes, from the public
Catalog seam through Grove launch and mutation admission.

## Done when

- Wrapper command/bind/targeted-route grammar coexists with flat entries, including
  a flat `config` key. Names, duplicates, shapes and local restrictions validate.
- Effective references validate after local replacement; dormant command templates
  are not compiled, but every effective binding validates its command template.
- Named scanning supports literal `$$`, whole runtime slots, and literal executable
  checks. Parameter declarations/references and patches remain explicit errors.
- Inspection retains reference chains, replaced target histories and source spans;
  expansion agrees with its words and convenience loading with Catalog resolution.
- Generic public tests and Grove fake-launch/mutation tests cover reuse, local
  targets, non-admission and refusal. Documentation and source-exact books are
  current; `bash scripts/check.sh` passes.

## Implementation plan

1. Add failing public-seam tests in `crates/keyed-launch/tests/named_commands.rs`.
2. Extend captured declarations in `templates.rs` via a private named-command
   module; fold targets and compile active definitions without changing public APIs.
3. Project declaration histories and compiled commands into existing Inspection
   records. Keep legacy scanner and eager checks unchanged.
4. Exercise existing Grove launch and tree-mutation seams with wrapper fixtures.
5. Update configuration references and every affected book manifest/fragment;
   run focused tests followed by the root principal checks, retire and seal.

## Decisions (running log)

The reviewed grammar is retained. Decompose named-command-reuse-k19 at the
parameter-free reuse boundary: reference folding and scanner escaping ship here;
parameter schemas/defaults and fragment substitution ship in named-command-parameters-k22.
No parser-only interface or accepted-but-ignored syntax is introduced.

The one in-session adversarial review found an empty-route-block grammar gap:
`route "k" "b" {}` is parameter-free and must be accepted. Classified as valid
and actionable; a public failing regression covers the correction, while empty
binding blocks remain invalid. No other actionable finding was reported. The
fix is conclusively covered by this test seam, so no second review is needed.

## Verification

Public tests cover shared reuse, local binding/route changes, literal transitions,
dormant definitions versus effective bindings, grammar refusals, escaped dollars,
source-independent inspection, ordered diagnostics and native/NUL values. Grove's
real fake-launch and leaf-add seams cover exact argv and refusal before use.

`bash scripts/check.sh` passed all eight principal checks, including workspace
tests and final validation of all six books. The keyed-launch book reconstructs
11 files and 3,172 lines with no deferred ranges. SHA-256 digests before and after
the final run matched for 1,768 inputs under crates, docs, plugins, scripts,
testing and .cargo, plus root TOML manifests, Cargo.lock and CONTEXT.md.
An earlier run caught a displaced usage anchor; restoring its heading adjacency
passed the focused coverage/book checks and the subsequent full run.

The public interface and ADR decisions are unchanged. Parameter-free reference
reuse is delivered; k22 retains the parameter obligations, so k19 remains live.
