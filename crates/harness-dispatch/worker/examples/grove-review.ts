// harness-dispatch/examples/grove-review — a starter policy for Grove's session
// kinds that applies the provider rule to Grove's reviews: every review it
// selects runs on a candidate whose provider origin differs from the original
// creator's, as the review leaf's own task file names that creator.
//
// Use it whole from your personal policy, ~/.config/harness-dispatch/policy.ts:
//
//   export { policy } from "harness-dispatch/examples/grove-review";
//
// with Grove's dispatch command, which passes the task file it selected:
//
//   harness-dispatch run --kind ${kind} --task-file ${task_file} --task-id ${task_id} --prompt ${prompt}
//
// or apply the rule to a catalog, routes and review kinds of your own:
//
//   import { definePolicy } from "harness-dispatch/sdk";
//   import { groveReviewSelector } from "harness-dispatch/examples/grove-review";
//   export const policy = definePolicy({
//     schemaVersion: 1, version: "mine-1", catalog,
//     ...groveReviewSelector({ catalog, routes, reviews }),
//   });
//
// Better still, copy this file beside your policy and edit it. It is starting
// policy, not a recommendation. It builds on three shipped modules:
// harness-dispatch/examples/grove-static, whose catalog and routes it keeps and
// whose comments justify each kind's effort; harness-dispatch/examples/review,
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
// record store holds for that run, whatever the catalog says today. For a
// producer finished without harness-dispatch, before you adopted it or by a
// harness Grove launched directly, `**Creator:** declared <origin>` declares
// its origin instead. Grove's sessions do not yet write either line, so you
// do. A review with neither refuses, and so does one whose lines are
// duplicated or malformed; harness-dispatch/grove states the grammar.
//
// THE RULE. For each of Grove's five review kinds, on every invocation, retry
// and explicit choice, the reviewer is of another origin than the creator's:
// the explicit choice, or else the kind's entry below for the creator's origin.
// Nothing is chosen in place of a reviewer the rule refuses.
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
  type Candidate,
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
import { catalog as groveCatalog, routes as groveRoutes } from "harness-dispatch/examples/grove-static";

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
export function groveReviewSelector<const C extends readonly Candidate[]>(rules: ReviewRules<C>): GroveReviewSelector {
  const { select } = reviewSelector(rules);
  return {
    loadContext(request, host) {
      const context = groveContext(request, host, rules.reviews);
      return "status" in context ? context : lookUpCreator(context, host);
    },
    select,
  };
}

/** grove-static's candidates: a lead harness, and a reviewer from another provider. */
export const catalog = groveCatalog;

/** A candidate ID from {@link catalog}. */
export type CandidateId = (typeof catalog)[number]["id"];

/** A provider origin in {@link catalog}. */
export type Origin = (typeof catalog)[number]["provider"];

/**
 * Each of Grove's review kinds, exactly, from its creator's origin to a
 * reviewer of the other, at grove-static's effort for that kind.
 */
export const reviews = {
  // The only check requirements, a design or a plan gets before later work
  // builds on it, with arguments rather than tests to catch a defect.
  "review-requirements": { openai: "review-xhigh", anthropic: "lead-xhigh" },
  "review-design": { openai: "review-xhigh", anthropic: "lead-xhigh" },
  "review-planning": { openai: "review-xhigh", anthropic: "lead-xhigh" },
  // A prototype is thrown away by design, and a person checks it at once.
  "review-prototype": { openai: "review-medium", anthropic: "lead-medium" },
  // Tests and types already check the code; what the review misses surfaces
  // later, where repair costs more.
  "review-impl": { openai: "review-high", anthropic: "lead-high" },
} as const satisfies Readonly<Record<string, ReviewEntry<Origin, CandidateId>>>;

/**
 * Every other kind: grove-static's routes, exactly, without its review kinds,
 * which the rule selects for instead.
 */
export const routes: Readonly<Record<string, CandidateId>> = Object.fromEntries(
  Object.entries(groveRoutes).filter(([kind]) => !Object.hasOwn(reviews, kind)),
);

/** The whole starter policy: {@link catalog}, {@link routes} and {@link reviews}, under the rule. */
export const policy = definePolicy({
  schemaVersion: 1,
  version: "harness-dispatch/examples/grove-review 1",
  catalog,
  ...groveReviewSelector({ catalog, routes, reviews }),
});
