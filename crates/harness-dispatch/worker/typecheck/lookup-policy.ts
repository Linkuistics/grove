// A computed policy that looks up its reviewed artifact's creator run,
// type-checked against the SDK's shipped declarations by
// `task dispatch:typecheck`, and evaluated through the compiled worker by the
// command-seam tests, so the declarations and the runtime agree on run lookup.
// The loader looks up the run the caller's creator reference names; `select`
// finds that answer among the delivered `runs` and chooses by the provider the
// run recorded, never by today's catalog.
import { definePolicy, type Refused, type RunLookup } from "harness-dispatch/sdk";

function refuse(code: string, message: string): Refused {
  return { status: "refused", code, message, remedy: "name a creator run this record store holds" };
}

export const policy = definePolicy({
  schemaVersion: 1,
  version: "typecheck-lookup-1",
  catalog: [
    {
      id: "deep",
      provider: "origin-a",
      model: "model-large",
      effort: "high",
      program: "fake-harness",
      args: [{ slot: "prompt" }],
    },
    {
      id: "quick",
      provider: "origin-b",
      model: "model-small",
      effort: "low",
      program: "fake-harness",
      args: [{ slot: "prompt" }],
    },
  ],
  loadContext(request, host) {
    const context = request.context ?? { schemaVersion: 1 };
    const run = context.reviewedArtifact?.creator?.run;
    if (run !== undefined) {
      const lookup: RunLookup = host.run(run);
      host.diagnostic(`looked up ${lookup.runId}: ${lookup.status}`);
    }
    return context;
  },
  select(_request, context) {
    const run = context?.reviewedArtifact?.creator?.run;
    const lookup = context?.runs?.find((answer) => answer.runId === run);
    if (lookup === undefined) return refuse("creator_not_looked_up", "no run was looked up");
    if (lookup.status === "missing") return refuse("creator_missing", `run ${lookup.runId} is not recorded`);
    if (lookup.launchFailure !== null) {
      return refuse("creator_not_executed", `run ${lookup.runId} failed to launch: ${lookup.launchFailure.cause}`);
    }
    const provider = lookup.candidate.provider;
    return {
      status: "selected",
      candidateId: provider === "origin-a" ? "quick" : "deep",
      reason: `creator ${lookup.runId} of task ${lookup.taskId ?? "none"} ran ${provider}`,
    };
  },
});
