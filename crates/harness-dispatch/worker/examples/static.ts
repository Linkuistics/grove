// harness-dispatch/examples/static — a starter routes policy for a caller of
// your own, with no Grove involved.
//
// Use it whole from your personal policy, ~/.config/harness-dispatch/policy.ts:
//
//   export { policy } from "harness-dispatch/examples/static";
//
// or keep its routes and supply your own catalog with the same candidate IDs:
//
//   import { definePolicy } from "harness-dispatch/sdk";
//   import { routes } from "harness-dispatch/examples/static";
//   export const policy = definePolicy({ schemaVersion: 1, version: "mine-1", catalog: [...], routes });
//
// Better still, copy this file beside your policy and edit it. It is starting
// policy, not a recommendation. `my-agent-wrapper` is an illustrative wrapper
// you supply, not a shipped tool: it receives `--model`, `--effort` and the
// prompt, and should exec your harness with them. The model and provider are
// placeholders for your own.
//
// WHY EFFORT VARIES BY KIND. More reasoning effort pays where a mistake costs
// more to find and repair. Each route below is justified by six properties of
// the work, never by which model is better:
//
//   - abstraction: how far the output is from something that can be run or read;
//   - uncertainty: how much must be inferred rather than read from the task;
//   - consequences: what a mistake breaks, and for whom;
//   - downstream repair: how much later work a mistake forces to be redone;
//   - reversibility: whether the result can be undone cheaply once acted on;
//   - available checks: whether tests, types or a reader catch a mistake soon.
//
// These are priors for a kind of work, not calibrated estimates. A task that
// is smaller or riskier than its kind suggests deserves `--choice`, or a policy
// of your own that computes the choice. Effort labels are each harness's own
// vocabulary, not quantities comparable across vendors.

import { definePolicy, type Candidate } from "harness-dispatch/sdk";

const agent = {
  provider: "your-provider",
  model: "your-model",
  program: "my-agent-wrapper",
  args: ["--model", { slot: "model" }, "--effort", { slot: "effort" }, { slot: "prompt" }],
} as const;

/** One harness and model at four efforts. Each ID names what the effort is for. */
export const catalog = [
  { id: "quick", effort: "low", ...agent },
  { id: "standard", effort: "medium", ...agent },
  { id: "careful", effort: "high", ...agent },
  { id: "deliberate", effort: "xhigh", ...agent },
] as const satisfies readonly Candidate[];

/** A candidate ID from {@link catalog}. */
export type CandidateId = (typeof catalog)[number]["id"];

/** Kind to candidate, exactly. A kind not listed refuses; nothing falls back. */
export const routes = {
  // Answering a question about existing code changes nothing, so it is wholly
  // reversible. The reader checks the answer at once, and a wrong one costs a
  // follow-up question rather than rework.
  question: "quick",
  // A bug fix is concrete and local. The failing test states the goal and
  // checks the result, and a bad fix is reverted in one step.
  bugfix: "standard",
  // A feature adds behaviour across modules. Tests check what it does, but the
  // interfaces it chooses bind later work, so a poor choice is repaired
  // downstream.
  feature: "careful",
  // A migration changes data or published interfaces in place. It is hard to
  // reverse once it has run, its consequences reach every consumer, and
  // checks often show the damage only afterwards.
  migration: "deliberate",
  // Architecture is the most abstract and most uncertain work here. Nothing
  // runs yet to check it, and every later task builds on it, so a mistake is
  // repaired by redoing that work.
  architecture: "deliberate",
} as const satisfies Readonly<Record<string, CandidateId>>;

/** The whole starter policy: {@link catalog} and {@link routes}. */
export const policy = definePolicy({
  schemaVersion: 1,
  version: "harness-dispatch/examples/static 1",
  catalog,
  routes,
});
