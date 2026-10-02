// Shapes the SDK's declarations must refuse. Each `@ts-expect-error` fails the
// type check if its line stops being an error, so this file is the negative
// control for `routes-policy.ts`: declarations that accepted anything would
// fail here rather than pass there.
import {
  definePolicy,
  type Context,
  type Creator,
  type Policy,
  type Selected,
  type SelectionRequest,
  type SourceRecord,
} from "harness-dispatch/sdk";

const selected: Selected = {
  status: "selected",
  program: "p",
  args: ["a"],
  provider: "origin-a",
  model: "m",
  effort: "e",
  reason: "r",
};

// @ts-expect-error schemaVersion is exactly 2
export const wrongVersion: Policy = { schemaVersion: 1, version: "v", select: () => selected };

// @ts-expect-error a policy has a select
export const noSelect: Policy = { schemaVersion: 2, version: "v" };

export const catalogued = definePolicy({
  schemaVersion: 2,
  version: "v",
  // @ts-expect-error a policy has no catalog: select returns the command
  catalog: [],
  select: () => selected,
});

export const routed = definePolicy({
  schemaVersion: 2,
  version: "v",
  // @ts-expect-error a policy has no routes form: a table is something select consults
  routes: {},
  select: () => selected,
});

// @ts-expect-error a selected result declares its provider origin
export const noProvider: Selected = { status: "selected", program: "p", args: [], model: "m", effort: "e", reason: "r" };

// @ts-expect-error an argument is a string, one whole word
export const slotted: Selected = { ...selected, args: [{ slot: "prompt" }] };

export const unknownStatus: Policy = {
  schemaVersion: 2,
  version: "v",
  // @ts-expect-error a result's status is "selected" or "refused"
  select: () => ({ ...selected, status: "chosen" }),
};

export const namedCandidate: Policy = {
  schemaVersion: 2,
  version: "v",
  // @ts-expect-error a selected result is the command, not the name of one
  select: () => ({ status: "selected", candidateId: "c", reason: "r" }),
};

export const noRemedy: Policy = {
  schemaVersion: 2,
  version: "v",
  // @ts-expect-error a refusal says what to do about it
  select: async () => ({ status: "refused", code: "c", message: "m" }),
};

// @ts-expect-error a request carries no explicit choice: a caller steers with a parameter the policy reads
export const noChoice = (request: SelectionRequest) => request.explicitChoice;

// @ts-expect-error a parameter the caller did not pass is undefined, never a string
export const absentParam = (request: SelectionRequest): string => request.params["repo"];

export const selectReadsNothing: Policy = {
  schemaVersion: 2,
  version: "v",
  select: (_request, _context, host) => {
    // @ts-expect-error select's host has no reads: its context is the measured one
    host.readText("notes.md");
    return selected;
  },
};

export const selectLooksUpNothing: Policy = {
  schemaVersion: 2,
  version: "v",
  select: (_request, _context, host) => {
    // @ts-expect-error select's host has no run lookup: its context carries the runs looked up
    host.run("5f0e2c41-9b7d-4a3e-8c15-2d6f7a9b0e34");
    return selected;
  },
};

export const lookupNeedsItsStatus: Policy = {
  schemaVersion: 2,
  version: "v",
  loadContext: (_request, host) => {
    const lookup = host.run("5f0e2c41-9b7d-4a3e-8c15-2d6f7a9b0e34");
    // @ts-expect-error a missing run has no provider: check its status first
    host.diagnostic(lookup.provider);
    return { schemaVersion: 1 };
  },
  select: () => selected,
};

export const loaderWithoutVersion: Policy = {
  schemaVersion: 2,
  version: "v",
  // @ts-expect-error a loaded context is a version-1 context
  loadContext: () => ({ summary: "s" }),
  select: () => selected,
};

// @ts-expect-error a creator has exactly one form
export const bothCreators: Creator = { run: "5f0e2c41-9b7d-4a3e-8c15-2d6f7a9b0e34", declared: "origin-a" };

// @ts-expect-error a source record pins its evidence by sha256 or version
export const unpinnedSource: SourceRecord = { name: "notes.md" };

// @ts-expect-error a context is data, with no executable field
export const executableContext: Context = { schemaVersion: 1, program: "sh" };
