// A static routes policy, type-checked against the SDK's shipped declarations
// by `task dispatch:typecheck`, and evaluated through the compiled worker by
// the command-seam tests, so the declarations and the runtime agree on it.
import { definePolicy } from "harness-dispatch/sdk";

export const policy = definePolicy({
  schemaVersion: 1,
  version: "typecheck-fixture-1",
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
  routes: { design: "deep", impl: "quick" },
});
