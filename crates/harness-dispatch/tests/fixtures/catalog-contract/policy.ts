export const policy = {
  schemaVersion: 1,
  version: "catalog-contract-fixture-1",
  catalog: [
    { id: "builder", provider: "your-provider", model: "model-a", effort: "high", program: "fake-harness", args: ["--model", { slot: "model" }, "--effort", { slot: "effort" }, { slot: "prompt" }] },
    { id: "auditor", provider: "your-other-provider", model: "model-b", effort: "medium", program: "fake-harness", args: [{ slot: "taskId" }, { slot: "prompt" }] },
  ],
  routes: { build: "builder" },
};
