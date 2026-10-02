// A policy whose `select` consults a table by kind, type-checked against the
// SDK's shipped declarations by `task dispatch:typecheck`, and evaluated
// through the compiled worker by the command-seam tests, so the declarations
// and the runtime agree on it.
import { definePolicy, type Selected } from "harness-dispatch/sdk";

type Command = Omit<Selected, "status" | "reason">;

const table: Readonly<Record<string, (prompt: string) => Command>> = {
  design: (prompt) => ({
    program: "fake-harness",
    args: ["--model", "model-large", "--effort", "high", prompt],
    provider: "origin-a",
    model: "model-large",
    effort: "high",
  }),
  impl: (prompt) => ({
    program: "fake-harness",
    args: [prompt],
    provider: "origin-b",
    model: "model-small",
    effort: "low",
  }),
};

export const policy = definePolicy({
  schemaVersion: 2,
  version: "typecheck-fixture-1",
  select(request) {
    const kind = JSON.stringify(request.kind);
    const route = Object.hasOwn(table, request.kind) ? table[request.kind] : undefined;
    if (route === undefined) {
      return {
        status: "refused",
        code: "incomplete_mapping",
        message: `the table names no command for kind ${kind}`,
        remedy: "add an entry for this kind to the table",
      };
    }
    return { status: "selected", ...route(request.prompt), reason: `table[${kind}]` };
  },
});
