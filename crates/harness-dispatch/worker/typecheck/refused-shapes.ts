// Shapes the SDK's declarations must refuse. Each `@ts-expect-error` fails the
// type check if its line stops being an error, so this file is the negative
// control for `routes-policy.ts`: declarations that accepted anything would
// fail here rather than pass there.
import { definePolicy, type Candidate, type Policy } from "harness-dispatch/sdk";

const candidate: Candidate = {
  id: "c",
  provider: "origin-a",
  model: "m",
  effort: "e",
  program: "p",
  args: [{ slot: "prompt" }],
};

// @ts-expect-error schemaVersion is exactly 1
export const wrongVersion: Policy = { schemaVersion: 2, version: "v", catalog: [candidate], routes: {} };

// @ts-expect-error the routes form needs its table
export const noRoutes: Policy = { schemaVersion: 1, version: "v", catalog: [candidate] };

export const badSlot = definePolicy({
  schemaVersion: 1,
  version: "v",
  // @ts-expect-error a slot names one of the documented caller inputs
  catalog: [{ ...candidate, args: [{ slot: "cwd" }] }],
  routes: {},
});

// @ts-expect-error a candidate declares its provider origin
export const noProvider: Candidate = { id: "c", model: "m", effort: "e", program: "p", args: [] };
