// The Grove adapter and the Grove review example as an owner's policy uses
// them, type-checked against their shipped declarations by
// `task dispatch:typecheck`, and evaluated through the compiled worker by the
// command-seam tests: an owner's own catalog, routes and review kind, with
// each review's artifact and creator read from its Grove task file. Each
// `@ts-expect-error` fails the type check if its line stops being an error, so
// the declarations are seen to refuse what they must not accept.
import { definePolicy, type Candidate, type SelectionRequest } from "harness-dispatch/sdk";
import { groveContext, version } from "harness-dispatch/grove";
import { lookUpCreator, reviewSelector } from "harness-dispatch/examples/review";
import { groveReviewSelector } from "harness-dispatch/examples/grove-review";

const args = [{ slot: "prompt" }] as const;

const catalog = [
  { id: "builder", provider: "your-provider", model: "model-a", effort: "high", program: "fake-harness", args },
  { id: "auditor", provider: "your-other-provider", model: "model-b", effort: "high", program: "fake-harness", args },
] as const satisfies readonly Candidate[];

const reviews = { audit: { "your-provider": "auditor", "your-other-provider": "builder" } } as const;

export const policy = definePolicy({
  schemaVersion: 1,
  version: `typecheck-grove-${version}`,
  catalog,
  ...groveReviewSelector({ catalog, routes: { build: "builder" }, reviews }),
});

// The adapter alone, in a loader of the owner's own: a refusal it returns is
// the loader's to return.
export const composed = definePolicy({
  schemaVersion: 1,
  version: "typecheck-grove-composed-1",
  catalog,
  loadContext(request, host) {
    const context = groveContext(request, host, reviews);
    return "status" in context ? context : lookUpCreator(context, host);
  },
  select: reviewSelector({ catalog, routes: {}, reviews }).select,
});

export const unknownReviewer = groveReviewSelector({
  catalog,
  routes: {},
  // @ts-expect-error a reviewer is one of the catalog's candidate IDs
  reviews: { audit: { "your-provider": "auditer" } },
});

export const selectingLoader = definePolicy({
  schemaVersion: 1,
  version: "typecheck-grove-selecting-1",
  catalog,
  routes: {},
  // @ts-expect-error a loader returns a context or a refusal, never a selection
  loadContext: () => ({ status: "selected", candidateId: "builder", reason: "r" }),
});

// @ts-expect-error the adapter reads through the host a loader receives
export const hostless = (request: SelectionRequest) => groveContext(request, reviews);
