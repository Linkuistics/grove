# skill-review-k3

**Kind:** review-impl

## Goal

Try to **disprove** `plugins/linkuistics/skills/using-codebase-memory/SKILL.md`.
Read it as a hostile reproducer: run every command it contains, and for each
sentence that makes a factual claim, ask what output would falsify it and go get
that output. Produce findings; change nothing.

## Context

Read the skill file **first and on its own**, before any grove document, and
form your expectations from it alone. It is going to be trusted by agents in
four harnesses who have no access to this grove — so the only fair test is the
one they will run.

Only afterwards, read `.grove/BRIEF.md` and
`docs/superpowers/specs/2026-07-29-portable-codebase-memory-skill-design.md` to
check coverage, and
`plugins/linkuistics/skills/authoring-conventions/SKILL.md` to check the
frontmatter against the house rule (capability + "Use when …", never a workflow
summary, frontmatter under 1024 chars).

**Fixture:** `Users-antony-Development-herdr`. Binary `codebase-memory-mcp`
0.8.1.

## Done when

Findings are written into this file under a `## Findings` heading, each one
naming the exact claim, the command run, the actual output, and whether it
**confirms** or **refutes**. Then this leaf is committed and retired.

Nothing in `plugins/` is edited by this leaf — corrections belong to
`skill-integrate-k4`. If there are no findings, say so explicitly; an empty
review is a real result.

## Notes

**The specific failure mode this chain exists to catch.** During scoping, the
plan's Global Constraints were found to assert that passing
`relationship`/`direction` alongside `min_degree` "produces byte-identical
results". The commands had been run. The projection the author checked
(`.results[]`) *did* match. The prose was still false — `.total` differs (2460 vs
2458). So: **a passing command is not a verified claim.** Check the claim's
actual scope, not the narrowed view that made it look true.

Highest-yield checks, in rough order:

- Every claim of *equality* or *identity* between two invocations — compare the
  whole payload, not a projection.
- Every exact number, name, or count. The fixture drifts (23,641/97,504 when the
  plan was written; 23,681/97,906 at scoping time).
- The exit-code claims — run them **without** a pipeline, then with one.
- Whether the stated stream split (JSON→stdout, logs→stderr) survives the exact
  pipe form the file documents.
- Coverage: all eight of the spec's "Skill content" items, and the fourteen tool
  names spelled correctly.
- Anything asserted about a harness other than this one, which cannot be tested
  here and therefore must be sourced or marked `UNVERIFIED`.

## Findings

Review environment: `codebase-memory-mcp --version` returned
`codebase-memory-mcp 0.8.1`. `index_status` reported the fixture
`Users-antony-Development-herdr` ready at 23,681 nodes / 97,906 edges.

### Refutes — `--json` does not duplicate the envelope to stderr

**Exact claim:** “It wraps the payload in an MCP envelope
(`.content[0].text`, a JSON *string* needing a second parse), writes that
envelope to **both** stdout and stderr, and returns exit `0` on failure.”

**Command run:** both success and failure were captured to separate files:

```bash
codebase-memory-mcp cli list_projects --json >success.out 2>success.err
codebase-memory-mcp cli search_graph '{"project":"nope"}' --json >failure.out 2>failure.err
```

**Actual output:** both calls exited 0 and stdout held the MCP envelope. In
both cases stderr contained only:

```text
level=info msg=mem.init budget_mb=65536 total_ram_mb=131072
```

The full stdout/stderr byte comparison returned `streams_identical:false` for
both success and failure. The wrapping and dishonest failure status are
confirmed; the duplicated-envelope claim is refuted.

### Refutes — the three `search_graph` match modes are not all combinable

**Exact claim:** “`search_graph` has three independent, combinable match modes:
`query`, `name_pattern` / `qn_pattern`, and `semantic_query`.”

**Commands run:** compare a BM25 query with the same query plus a regex that
cannot match anything:

```bash
codebase-memory-mcp cli search_graph \
  '{"project":"Users-antony-Development-herdr","query":"wait socket","limit":5}'
codebase-memory-mcp cli search_graph \
  '{"project":"Users-antony-Development-herdr","query":"wait socket","name_pattern":"^NO_SUCH_SYMBOL$","limit":5}'
```

**Actual output:** both exited 0; the whole stdout payloads were byte-identical,
both reported `total:257`, and both returned five `wait_for_socket` rows. The
impossible `name_pattern` was ignored when `query` was present, so these two
modes are not independent/composed. Verdict: **refutes**.

### Refutes — `ingest_traces` does not fold traces into the graph

**Exact claim:** the fourteen-tool table describes `ingest_traces` as “fold
runtime traces into the graph.”

**Command run:**

```bash
codebase-memory-mcp cli ingest_traces \
  '{"project":"Users-antony-Development-herdr","traces":[]}'
```

**Actual output:** exit 0 with:

```json
{"status":"accepted","traces_received":0,"note":"Runtime edge creation from traces not yet implemented"}
```

The endpoint accepts the payload but explicitly says graph mutation is not
implemented. Verdict: **refutes**.

### Refutes — `index_repository` has a fourth mode

**Exact claim:** the fourteen-tool table documents the optional modes as
“`full`/`moderate`/`fast`”.

**Command run:**

```bash
codebase-memory-mcp cli index_repository \
  '{"repo_path":"/private/tmp/codebase-memory-review.8n4LUT","mode":"cross-repo-intelligence"}'
```

**Actual output:** exit 1 with:

```json
{"error":"target_projects is required for cross-repo-intelligence mode. Use [\"*\"] for all projects. Run list_projects to see available."}
```

The binary recognizes `cross-repo-intelligence`; its required companion
argument is `target_projects`. Verdict: **refutes the exhaustive mode list**.

### Refutes — the worked pipelines reproduce the silent-failure trap

**Exact claims in tension:** the skill correctly says a pipeline “replaces the
binary's exit `1`” and prescribes “Guard without a pipeline”, but later presents
the two composition pipelines and the final `index_repository | jq` pipeline as
worked commands without that guard.

**Commands run:** both composition examples were first run unchanged and
succeeded. Each was then run unchanged except for `P=nope`. The indexing
example was run unchanged in the sandbox (where graph persistence was denied),
then unchanged with the required permission.

**Actual output:** with `P=nope`, both composition pipelines printed the
`project not found or not indexed` error on stderr, emitted no result rows, and
reported `pipeline_exit:0`. The sandboxed indexing call logged
`pipeline.err phase=dump`, emitted its error only on stderr, and the trailing
`jq` again made the pipeline exit 0. With graph-write permission the same
indexing command succeeded with:

```json
{"project":"private-tmp-codebase-memory-review.8n4LUT","status":"indexed","nodes":4,"edges":3,"excluded":null}
```

The disposable index was then removed with `delete_project`, which returned
`status:"deleted"`; no pre-existing index was changed. Verdict: **refutes the
worked examples as safe composition recipes**, even though their happy paths
are valid.

### Refutes house citation conformance — the Claude Code claim is unsourced

**Exact claim:** “In Claude Code these MCP tools are deferred: they are absent
from the base tool list and must be loaded before they can be called at all,”
followed by a `ToolSearch("select:…")` invocation.

**Check performed:** `ToolSearch` is a Claude Code harness primitive and cannot
be executed in this Codex session. The skill supplies neither an authority link
nor the house marker `UNVERIFIED`.

**Actual result:** the claim remains untestable here. The house convention says
an external, harness-specific fact must carry a source or literal `UNVERIFIED`.
Verdict: **refutes authoring-convention compliance**, not the underlying Claude
Code behaviour.

## Confirmed coverage

All remaining fenced shell commands were executed against the live fixture (or,
for indexing, the disposable fixture above):

- `cli --help` was treated as a tool and failed with `unknown tool: --help`;
  top-level `--help` listed the same fourteen tool names as the MCP surface.
- Running `search_graph '{}'` from this indexed working tree still exited 1
  with `project not found or not indexed`; `list_projects` and the
  `repo_path`-based indexing call confirmed the two stated exceptions.
- Success put JSON on stdout, the `mem.init` log on stderr, and exited 0.
  Unknown-tool and unknown-project failures left stdout empty and exited 1;
  the documented `| head` form masked that as 0. The guarded capture preserved
  exit 1, and `grep '^{'` extracted the structured error and hint.
- The malformed-JSON example exited 1 and produced a byte-identical structured
  error to the genuine unknown-project call. The `jq -nc` argument builder
  produced valid JSON and a successful trace.
- Default `search_graph` truncation remained
  `{"total":2460,"rows":200,"has_more":true}`. Fetching all rows found
  1,230 with `in_degree:0`; every row had total degree at least 10. Adding
  `relationship:"CALLS",direction:"inbound"` returned 2,458 rows, 1,229 with
  zero in-degree.
- Both 123- and 104-fan-in symbols returned exactly 100 callers with no total
  or `has_more` key. The shared-name checks returned eight `wait_for_socket`
  symbols, zero callers for the bare name, and 67 callers for the documented
  qualified name at depth 1 with tests included. Outbound tracing returned a
  `callees` key.
- The high-fan-in Cypher command returned the documented leaders (123 and 121
  first), in `{columns,rows,total}` form. Grouping `wait_for_socket` by bare
  name merged 119 calls into one row; grouping by qualified name returned six
  distinct rows. Both happy-path composition scripts ran: the first emitted six
  symbol/count rows and the second emitted five source summaries.
- Non-mutating smoke calls covered `index_status`, `detect_changes`,
  `search_code`, `get_graph_schema`, `get_architecture`, and
  `manage_adr(mode:"get")`; the successful composition covered
  `get_code_snippet`. Together with the disposable `index_repository` /
  `delete_project` pair and the other commands above, all fourteen CLI tool
  names were exercised.
- The skill covers all eight approved spec content areas. Its frontmatter is
  477 bytes, its description is one 436-byte capability + `Use when …`
  sentence with no workflow sequence, and its 287-line body is below the
  house's ~500-line disclosure threshold. The unavailable upstream
  `writing-skills` / `anthropic-best-practices.md` materials could not be
  checked; the concrete house rules in `authoring-conventions` were checked
  directly.
