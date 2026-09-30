// A computed policy with a `loadContext`, type-checked against the SDK's
// shipped declarations by `task dispatch:typecheck`, and evaluated through the
// compiled worker by the command-seam tests, so the declarations and the
// runtime agree on it. The loader reads a JSON file and a text file relative
// to the caller's directory, attributes both, and keeps the caller's own
// context; `select` reads the delivered context and its measured sources.
import { definePolicy, type Context, type Json } from "harness-dispatch/sdk";

export const policy = definePolicy({
  schemaVersion: 1,
  version: "typecheck-context-1",
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
  loadContext(request, host): Context {
    const risk = host.readJson("risk.json");
    const notes = host.readText("notes.md", 1024);
    host.diagnostic(`read ${notes.source.bytes} bytes of notes`);
    const level = (risk.value as { readonly level?: Json } | null)?.level;
    return {
      ...request.context,
      schemaVersion: 1,
      summary: notes.text.trim(),
      assessments: { risk: { by: "risk.json", value: level ?? null } },
      sources: [risk.source, notes.source],
    };
  },
  select(request, context) {
    const risk = context?.assessments?.["risk"]?.value;
    const measured = context?.measured.map((source) => `${source.via}:${source.bytes}`).join(",") ?? "none";
    return {
      status: "selected",
      candidateId: risk === "high" ? "deep" : "quick",
      reason: `kind ${request.kind}, risk ${JSON.stringify(risk)}, measured ${measured}`,
    };
  },
});
