// Shapes the SDK's declarations must refuse. Each `@ts-expect-error` fails the
// type check if its line stops being an error, so this file is the negative
// control for `routes-policy.ts`: declarations that accepted anything would
// fail here rather than pass there.
import {
  definePolicy,
  type Candidate,
  type Context,
  type Creator,
  type Policy,
  type SelectPolicy,
  type SourceRecord,
} from "harness-dispatch/sdk";

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

// @ts-expect-error a policy has exactly one of `routes` or `select`
export const bothForms: Policy = {
  schemaVersion: 1,
  version: "v",
  catalog: [candidate],
  routes: {},
  select: () => ({ status: "selected", candidateId: "c", reason: "r" }),
};

export const unknownStatus: SelectPolicy = {
  schemaVersion: 1,
  version: "v",
  catalog: [candidate],
  // @ts-expect-error a result's status is "selected" or "refused"
  select: () => ({ status: "chosen", candidateId: "c", reason: "r" }),
};

export const noReason: SelectPolicy = {
  schemaVersion: 1,
  version: "v",
  catalog: [candidate],
  // @ts-expect-error a selected result gives its reason
  select: () => ({ status: "selected", candidateId: "c" }),
};

export const noRemedy: SelectPolicy = {
  schemaVersion: 1,
  version: "v",
  catalog: [candidate],
  // @ts-expect-error a refusal says what to do about it
  select: async () => ({ status: "refused", code: "c", message: "m" }),
};

export const selectReadsNothing: SelectPolicy = {
  schemaVersion: 1,
  version: "v",
  catalog: [candidate],
  select: (_request, _context, host) => {
    // @ts-expect-error select's host has no reads: its context is the measured one
    host.readText("notes.md");
    return { status: "selected", candidateId: "c", reason: "r" };
  },
};

export const noRunLookupYet: SelectPolicy = {
  schemaVersion: 1,
  version: "v",
  catalog: [candidate],
  loadContext: (_request, host) => {
    // @ts-expect-error run lookup is not in this release
    host.run("5f0e2c41-9b7d-4a3e-8c15-2d6f7a9b0e34");
    return { schemaVersion: 1 };
  },
  select: () => ({ status: "selected", candidateId: "c", reason: "r" }),
};

export const loaderWithoutVersion: SelectPolicy = {
  schemaVersion: 1,
  version: "v",
  catalog: [candidate],
  // @ts-expect-error a loaded context is a version-1 context
  loadContext: () => ({ summary: "s" }),
  select: () => ({ status: "selected", candidateId: "c", reason: "r" }),
};

// @ts-expect-error a creator has exactly one form
export const bothCreators: Creator = { run: "5f0e2c41-9b7d-4a3e-8c15-2d6f7a9b0e34", declared: "origin-a" };

// @ts-expect-error a source record pins its evidence by sha256 or version
export const unpinnedSource: SourceRecord = { name: "notes.md" };

// @ts-expect-error a context is data, with no executable field
export const executableContext: Context = { schemaVersion: 1, program: "sh" };
