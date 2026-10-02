// A `select` that reads the caller's parameters and prompt, type-checked
// against the SDK's shipped declarations by `task dispatch:typecheck`, and
// evaluated through the compiled worker by the command-seam tests, so the
// declarations and the runtime agree on it. `request` is typed with no
// annotation, a parameter the caller did not pass is `undefined`, and `select`
// may return a promise of either result.
import { definePolicy, PROMPT_NOT_SUPPLIED } from "harness-dispatch/sdk";

export const policy = definePolicy({
  schemaVersion: 2,
  version: "typecheck-select-1",
  async select(request) {
    const repo = request.params["repo"];
    if (repo === undefined) {
      return {
        status: "refused",
        code: "repo_missing",
        message: "this policy runs its harness in the caller's repository, and was given none",
        remedy: "pass --param repo=PATH",
      };
    }
    const deep = request.kind === "design";
    const prompted = request.prompt === PROMPT_NOT_SUPPLIED ? "no prompt" : `a ${request.prompt.length}-character prompt`;
    return {
      status: "selected",
      program: "fake-harness",
      args: ["-C", repo, `--kind=${request.kind}`, request.prompt],
      provider: deep ? "origin-a" : "origin-b",
      model: deep ? "model-large" : "model-small",
      effort: deep ? "high" : "low",
      reason: `kind ${request.kind} with ${prompted} within ${request.limits.selectionMs} ms`,
    };
  },
});
