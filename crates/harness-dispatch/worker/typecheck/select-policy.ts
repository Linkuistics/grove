// A computed `select` policy, type-checked against the SDK's shipped
// declarations by `task dispatch:typecheck`, and evaluated through the
// compiled worker by the command-seam tests, so the declarations and the
// runtime agree on it. `request` is typed by the policy's own form, with no
// annotation, and `select` may return a promise of either result.
import { definePolicy } from "harness-dispatch/sdk";

export const policy = definePolicy({
  schemaVersion: 1,
  version: "typecheck-select-1",
  catalog: [
    {
      id: "deep",
      provider: "origin-a",
      model: "model-large",
      effort: "high",
      program: "fake-harness",
      args: ["--model", { slot: "model" }, "--effort", { slot: "effort" }, { slot: "prompt" }],
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
  async select(request) {
    if (request.explicitChoice !== undefined) {
      return {
        status: "refused",
        code: "no_explicit_choices",
        message: `this policy chooses for itself, and was asked for ${request.explicitChoice}`,
        remedy: "omit --choice",
      };
    }
    const candidateId = request.kind === "design" ? "deep" : "quick";
    return {
      status: "selected",
      candidateId,
      reason: `kind ${request.kind} within ${request.limits.selectionMs} ms`,
    };
  },
});
