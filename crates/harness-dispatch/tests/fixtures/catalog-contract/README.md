# Catalog-contract fixture

`records.sqlite3` is a record store that harness-dispatch 21.13.0 wrote, the
last release whose policies were a catalog the function named by ID (36,864
bytes, SHA-256
`410d34c67e5f7d93a1efa3b14f49c3a86b625242577fd71701f0bedf915a70be`). It holds
two runs, each a handoff attempt with no observation and no launch failure:

| Run ID | Kind | Task identity | Candidate | Provider | Selected by |
|---|---|---|---|---|---|
| `45308255-7446-42b2-bbd7-e5f7861e46ef` | `build` | `parser-k12` | `builder` | `your-provider` | the routes table |
| `b2aec552-b5eb-4de0-b5e9-5db0d70a6d55` | `audit` | `parser-k13` | `auditor` | `your-other-provider` | `--choice auditor` |

Each launch document therefore carries what only that contract recorded: a
candidate ID, a selection form, an explicit choice, and slot objects among the
candidate's arguments. The tests copy the store into a sandbox and read these
runs with the current release, which must still show them, observe them, look
them up and resolve a review's creator from them.

The installed 21.13.0 `harness-dispatch run` produced it, with nothing written
by hand: `policy.ts` here was the personal policy under a temporary `HOME`,
with a `fake-harness` that exits 0 on PATH, and the two runs were

    harness-dispatch run --kind build --task-id parser-k12 --prompt 'Build the parser' --state-dir STATE
    harness-dispatch run --kind audit --choice auditor --task-id parser-k13 --prompt 'Audit the parser' --state-dir STATE

The paths its launch documents name are the scratch directory it was recorded
in. `policy.ts` is the version-1 policy that release evaluated, kept as the
record of what was run; the current release refuses it. Neither file is
regenerated or edited.
