// harness-dispatch/examples/grove-static — a starter routes policy for Grove's
// session kinds, each one mapped exactly.
//
// Use it whole from your personal policy, ~/.config/harness-dispatch/policy.ts:
//
//   export { policy } from "harness-dispatch/examples/grove-static";
//
// or keep its routes and supply your own catalog with the same candidate IDs:
//
//   import { definePolicy } from "harness-dispatch/sdk";
//   import { routes } from "harness-dispatch/examples/grove-static";
//   export const policy = definePolicy({ schemaVersion: 1, version: "mine-1", catalog: [...], routes });
//
// Better still, copy this file beside your policy and edit it. It is starting
// policy, not a recommendation. Grove passes the kind; this file never reads a
// task file or a Grove filename.
//
// Two harnesses share the work, as in Grove's own configuration examples. A
// lead produces and integrates; a reviewer from another provider reviews, runs
// the second research survey, copy-edits and proofs. `my-codex-wrapper` and
// `my-claude-wrapper` are illustrative wrappers you supply, not shipped tools:
// each receives `--model`, `--effort` and the prompt, and should exec its
// harness with them. Swap the two to lead with the other provider. The models
// are placeholders for your own. A static table cannot know which provider
// actually created the artifact a review reads, so it enforces no provider
// rule; that takes a policy that looks up the creator.
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
//   - available checks: whether tests, types or a later reader catch a mistake.
//
// These are priors for a kind of session, not calibrated estimates. A task
// that is smaller or riskier than its kind suggests deserves `--choice` in the
// command Grove runs, or a policy of your own that computes the choice. Effort
// labels are each harness's own vocabulary, not quantities comparable across
// vendors.

import { definePolicy, type Candidate } from "harness-dispatch/sdk";

const args = ["--model", { slot: "model" }, "--effort", { slot: "effort" }, { slot: "prompt" }] as const;
const lead = { provider: "openai", model: "your-codex-model", program: "my-codex-wrapper", args } as const;
const review = { provider: "anthropic", model: "your-claude-model", program: "my-claude-wrapper", args } as const;

/** Each harness at the efforts the routes use. Each ID names a role and an effort. */
export const catalog = [
  { id: "lead-max", effort: "max", ...lead },
  { id: "lead-xhigh", effort: "xhigh", ...lead },
  { id: "lead-high", effort: "high", ...lead },
  { id: "lead-medium", effort: "medium", ...lead },
  { id: "review-xhigh", effort: "xhigh", ...review },
  { id: "review-high", effort: "high", ...review },
  { id: "review-medium", effort: "medium", ...review },
] as const satisfies readonly Candidate[];

/** A candidate ID from {@link catalog}. */
export type CandidateId = (typeof catalog)[number]["id"];

/** Grove's session kinds to candidates, exactly. A kind not listed refuses. */
export const routes = {
  // Requirements are the most abstract work in the tree and the most
  // uncertain: the human's intent has to be drawn out, not read. No test exists
  // yet to catch a gap, and a missed requirement is repaired by redoing the
  // design, plan and code built on it.
  requirements: "lead-max",
  // The review is the only check requirements get before design builds on
  // them, and its triage can change the agreed contract, so both carry the
  // requirements' downstream reach.
  "review-requirements": "review-xhigh",
  "integrate-review-requirements": "lead-xhigh",

  // Design fixes abstract, coupled decisions that are costly to reverse once
  // increments build on them. Its checks are arguments and models rather than
  // tests, and an architectural defect survives local tests into every
  // increment, so its review and triage keep the same effort.
  design: "lead-xhigh",
  "review-design": "review-xhigh",
  "integrate-review-design": "lead-xhigh",

  // A plan's increment boundaries and dependencies shape every later session.
  // A wrong cut is found only when an increment cannot land, and is repaired
  // by re-planning and reworking finished leaves.
  planning: "lead-xhigh",
  "review-planning": "review-xhigh",
  "integrate-review-planning": "lead-xhigh",

  // A prototype is thrown away by design. It is cheap to redo and wholly
  // reversible, and its value is the reaction it provokes, which a person
  // checks at once.
  prototype: "lead-medium",
  "review-prototype": "review-medium",
  "integrate-review-prototype": "lead-medium",

  // Implementation is concrete and bounded by a settled design, and tests,
  // types and review check it; a defect is local and repaired in place. Raise
  // it for invariants or concurrency the compiler cannot check. The review
  // cannot run more than the tests already do, and what it misses surfaces
  // later, where repair costs more. Triage of findings against checked code is
  // bounded and reversible.
  impl: "lead-high",
  "review-impl": "review-high",
  "integrate-review-impl": "lead-medium",

  // A survey's uncertainty is the field itself, and only the second,
  // independent survey and the combiner check it, so each is thorough. The
  // second runs on the other provider to stay independent. The combiner must
  // challenge claims both surveys share; what it lets through informs design
  // with no later check.
  "research-a": "lead-high",
  "research-b": "review-high",
  "combine-research": "lead-xhigh",

  // A draft owns structure and technical truth. Copy-edit and proof follow and
  // catch much, but not wrong substance. Copy-editing and figures are local,
  // reversible work within a fixed structure, and proof checks them. Proof is
  // the final whole-document read, with nothing after it to catch what it
  // misses.
  draft: "lead-high",
  "copy-edit": "review-medium",
  art: "lead-medium",
  proof: "review-high",

  // Most finishing steps are explicit and checked by the tools that run them.
  // Raise it where a project's finish integrates and releases, which cannot be
  // undone once published.
  finish: "lead-medium",
} as const satisfies Readonly<Record<string, CandidateId>>;

/** The whole starter policy: {@link catalog} and {@link routes}. */
export const policy = definePolicy({
  schemaVersion: 1,
  version: "harness-dispatch/examples/grove-static 1",
  catalog,
  routes,
});
