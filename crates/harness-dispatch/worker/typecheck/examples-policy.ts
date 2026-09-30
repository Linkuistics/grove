// The starter examples as an owner's policy uses them, type-checked against
// their shipped declarations by `task dispatch:typecheck`, and evaluated
// through the compiled worker by the command-seam tests: an owner's own
// catalog under the Grove example's routes.
import { definePolicy, type Candidate } from "harness-dispatch/sdk";
import { routes, type CandidateId } from "harness-dispatch/examples/grove-static";
import { policy as generic } from "harness-dispatch/examples/static";

// The whole generic example is a policy as it stands.
export const whole: typeof generic & { readonly schemaVersion: 1 } = generic;

function mine(id: CandidateId, provider: string, effort: string): Candidate {
  return { id, provider, model: `model-for-${id}`, effort, program: "fake-harness", args: [{ slot: "prompt" }] };
}

export const policy = definePolicy({
  schemaVersion: 1,
  version: "examples-fixture-1",
  catalog: [
    mine("lead-max", "origin-a", "max"),
    mine("lead-xhigh", "origin-a", "xhigh"),
    mine("lead-high", "origin-a", "high"),
    mine("lead-medium", "origin-a", "medium"),
    mine("review-xhigh", "origin-b", "xhigh"),
    mine("review-high", "origin-b", "high"),
    mine("review-medium", "origin-b", "medium"),
  ],
  routes,
});

// @ts-expect-error a route's target is one of the example's candidate IDs
export const typo: CandidateId = "lead-maximum";
