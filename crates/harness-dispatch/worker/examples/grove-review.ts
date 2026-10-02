// harness-dispatch/examples/grove-review — a starter policy for Grove's session
// kinds that applies the provider rule to Grove's reviews: every review it
// selects runs a command whose provider origin differs from the original
// creator's, as the review leaf's own task file names that creator.
//
// Use it whole from your personal policy, ~/.config/harness-dispatch/policy.ts:
//
//   export { policy } from "harness-dispatch/examples/grove-review";
//
// Grove passes the task file of the leaf it selected for every session it
// launches, so nothing else is needed. Or apply the rule to routes and review
// kinds of your own:
//
//   import { definePolicy } from "harness-dispatch/sdk";
//   import { groveReviewSelector } from "harness-dispatch/examples/grove-review";
//   export const policy = definePolicy({
//     schemaVersion: 2, version: "mine-1",
//     ...groveReviewSelector({ routes, reviews }),
//   });
//
// Better still, copy this file beside your policy and edit it. It is starting
// policy, not a recommendation. It builds on three shipped modules:
// harness-dispatch/examples/grove-static, whose two harnesses and routes it
// keeps and whose comments justify each kind's effort; harness-dispatch/examples/review,
// whose rule it applies; and harness-dispatch/grove, the adapter that reads the
// task file.
//
// WHAT A GROVE REVIEW NEEDS. Its task file names the producer it reviews, by its
// handle, and that producer's original creator, each on a line of its own:
//
//   **Reviews:** parser-k12
//   **Creator:** run 5f0e2c41-9b7d-4a3e-8c15-2d6f7a9b0e34
//
// `**Creator:** run` names the dispatch run of the session that finished the
// producer, its HARNESS_DISPATCH_RUN_ID, so the creator's origin is the one the
// record store holds for that run, whatever the policy returns today. That
// session writes the line itself, as Grove's methodology directs, and a
// session that finishes a producer without harness-dispatch removes it. For
// such a producer, finished before you adopted harness-dispatch or by a
// harness Grove launched directly, you write `**Creator:** declared <origin>`
// to declare its origin instead. A run line is its writer's word: one naming
// some other existing run is not detected. A review with neither form refuses,
// and so does one whose lines are duplicated or malformed;
// harness-dispatch/grove states the grammar.
//
// THE RULE. For each of Grove's five review kinds, on every invocation and
// retry, the reviewer is of another origin than the creator's: the kind's
// entry below for the creator's origin. Nothing is run in place of a reviewer
// the rule refuses.
// harness-dispatch/examples/review states the checks in full. Every other kind
// takes grove-static's routes, where the rule does not apply, and a task file
// that declares `**Reviews:**` under such a kind refuses rather than taking one.
//
// WHY EFFORT VARIES BY KIND. Each review kind keeps grove-static's effort for
// it, justified there by six properties of the work, never by which model is
// better. Only the harness follows the creator: whichever provider made the
// artifact, the other one reviews it.

import {
  definePolicy,
  type Context,
  type ContextHost,
  type Refused,
  type SelectionRequest,
} from "harness-dispatch/sdk";
import { groveContext } from "harness-dispatch/grove";
import {
  lookUpCreator,
  reviewSelector,
  type ReviewEntry,
  type ReviewRules,
  type ReviewSelector,
} from "harness-dispatch/examples/review";
import { lead, review, routes as groveRoutes } from "harness-dispatch/examples/grove-static";
import type { Route } from "harness-dispatch/examples/static";

/** The two halves of a policy that applies the provider rule to Grove task files. */
export interface GroveReviewSelector {
  /**
   * The context the task file declares, with its creator run looked up, or
   * the adapter's refusal.
   */
  loadContext(request: SelectionRequest, host: ContextHost): Context | Refused;
  /** The review's reviewer under the rule, a route for any other kind, or a refusal. */
  readonly select: ReviewSelector["select"];
}

/**
 * The provider rule over `rules`, with each review kind's reviewed artifact
 * and creator read from the Grove task file the caller supplies.
 */
export function groveReviewSelector(rules: ReviewRules): GroveReviewSelector {
  const { select } = reviewSelector(rules);
  return {
    loadContext(request, host) {
      const context = groveContext(request, host, rules.reviews);
      return "status" in context ? context : lookUpCreator(context, host);
    },
    select,
  };
}

/**
 * Each of Grove's review kinds, exactly, from its creator's origin to a
 * reviewer of the other, at grove-static's effort for that kind.
 */
export const reviews = {
  // The only check requirements, a design or a plan gets before later work
  // builds on it, with arguments rather than tests to catch a defect.
  "review-requirements": { openai: review("xhigh"), anthropic: lead("xhigh") },
  "review-design": { openai: review("xhigh"), anthropic: lead("xhigh") },
  "review-planning": { openai: review("xhigh"), anthropic: lead("xhigh") },
  // A prototype is thrown away by design, and a person checks it at once.
  "review-prototype": { openai: review("medium"), anthropic: lead("medium") },
  // Tests and types already check the code; what the review misses surfaces
  // later, where repair costs more.
  "review-impl": { openai: review("high"), anthropic: lead("high") },
} as const satisfies Readonly<Record<string, ReviewEntry>>;

/**
 * Every other kind: grove-static's routes, exactly, without its review kinds,
 * which the rule selects for instead.
 */
export const routes: Readonly<Record<string, Route>> = Object.fromEntries(
  Object.entries(groveRoutes).filter(([kind]) => !Object.hasOwn(reviews, kind)),
);

/** The whole starter policy: {@link routes} and {@link reviews}, under the rule. */
export const policy = definePolicy({
  schemaVersion: 2,
  version: "harness-dispatch/examples/grove-review 2",
  ...groveReviewSelector({ routes, reviews }),
});
