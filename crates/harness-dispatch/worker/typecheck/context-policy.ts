// A policy with a `loadContext`, type-checked against the SDK's shipped
// declarations by `task dispatch:typecheck`, and evaluated through the
// compiled worker by the command-seam tests, so the declarations and the
// runtime agree on it. The loader reads a JSON file and a text file relative
// to the caller's directory, attributes both, and keeps the caller's own
// context; `select` reads the delivered context and its measured sources.
import { definePolicy, type Context, type Json } from "harness-dispatch/sdk";

export const policy = definePolicy({
  schemaVersion: 2,
  version: "typecheck-context-1",
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
    const deep = risk === "high";
    return {
      status: "selected",
      program: "fake-harness",
      args: [request.prompt],
      provider: deep ? "origin-a" : "origin-b",
      model: deep ? "model-large" : "model-small",
      effort: deep ? "high" : "low",
      reason: `kind ${request.kind}, risk ${JSON.stringify(risk)}, measured ${measured}`,
    };
  },
});
