// Copy this file beside an owner policy and edit its routes normally.
// Grove owns the task adapter and examples; Ravel embeds only its generic SDK.
import { definePolicy, type LoadContext } from "ravel/sdk";
import type { Route } from "ravel/examples/static";
import { groveContext } from "../grove.ts";
import { withProvenance } from "../provenance.ts";
import { reviewSelector, type ReviewEntry, type ReviewRules, type ReviewSelector } from "./review.ts";
import { lead, review, routes as groveRoutes } from "./grove-static.ts";

export interface GroveReviewSelector {
  readonly loadContext: LoadContext;
  readonly select: ReviewSelector["select"];
}
export function groveReviewSelector(rules: ReviewRules): GroveReviewSelector {
  return { loadContext: (request, host) => groveContext(request, host, rules.reviews), select: reviewSelector(rules).select };
}

// Preserve Grove's effort rationale: abstract reviews are the only check before
// downstream work; prototypes are disposable; code already has tests and types.
export const reviews = {
  "review-requirements": { openai: review("xhigh"), anthropic: lead("xhigh") },
  "review-design": { openai: review("xhigh"), anthropic: lead("xhigh") },
  "review-planning": { openai: review("xhigh"), anthropic: lead("xhigh") },
  "review-prototype": { openai: review("medium"), anthropic: lead("medium") },
  "review-impl": { openai: review("high"), anthropic: lead("high") },
} as const satisfies Readonly<Record<string, ReviewEntry>>;
export const routes: Readonly<Record<string, Route>> = Object.fromEntries(
  Object.entries(groveRoutes).filter(([kind]) => !Object.hasOwn(reviews, kind)),
);
// Opt in around the final policy so finishing producers carry their actual
// selected provider receipt; missing receipts still require owner declarations.
export const policy = withProvenance(definePolicy({ schemaVersion: 2, version: "grove/examples/grove-review 1", ...groveReviewSelector({ routes, reviews }) }));
