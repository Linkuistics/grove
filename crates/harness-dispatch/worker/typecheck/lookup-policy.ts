// A policy that looks up its reviewed artifact's creator run, type-checked
// against the SDK's shipped declarations by `task dispatch:typecheck`, and
// evaluated through the compiled worker by the command-seam tests, so the
// declarations and the runtime agree on run lookup. The loader looks up the
// run the caller's creator reference names; `select` finds that answer among
// the delivered `runs` and chooses by the provider the run recorded, never by
// what the policy would return today.
import { definePolicy, type Refused, type RunLookup } from "harness-dispatch/sdk";

function refuse(code: string, message: string): Refused {
  return { status: "refused", code, message, remedy: "name a creator run this record store holds" };
}

export const policy = definePolicy({
  schemaVersion: 2,
  version: "typecheck-lookup-1",
  loadContext(request, host) {
    const context = request.context ?? { schemaVersion: 1 };
    const run = context.reviewedArtifact?.creator?.run;
    if (run !== undefined) {
      const lookup: RunLookup = host.run(run);
      host.diagnostic(`looked up ${lookup.runId}: ${lookup.status}`);
    }
    return context;
  },
  select(request, context) {
    const run = context?.reviewedArtifact?.creator?.run;
    const lookup = context?.runs?.find((answer) => answer.runId === run);
    if (lookup === undefined) return refuse("creator_not_looked_up", "no run was looked up");
    if (lookup.status === "missing") return refuse("creator_missing", `run ${lookup.runId} is not recorded`);
    if (lookup.launchFailure !== null) {
      return refuse("creator_not_executed", `run ${lookup.runId} failed to launch: ${lookup.launchFailure.cause}`);
    }
    const other = lookup.provider === "origin-a";
    return {
      status: "selected",
      program: "fake-harness",
      args: [request.prompt],
      provider: other ? "origin-b" : "origin-a",
      model: other ? "model-small" : "model-large",
      effort: other ? "low" : "high",
      reason: `creator ${lookup.runId} of task ${lookup.taskId ?? "none"} ran ${lookup.provider}, model ${lookup.model}`,
    };
  },
});
