// The Grove adapter and the Grove review example as an owner's policy uses
// them, type-checked against their shipped declarations by
// `task dispatch:typecheck`, and evaluated through the compiled worker by the
// command-seam tests: an owner's own routes and review kind, with each
// review's artifact and creator read from its Grove task file. Each
// `@ts-expect-error` fails the type check if its line stops being an error, so
// the declarations are seen to refuse what they must not accept.
import { definePolicy, type SelectionRequest } from "harness-dispatch/sdk";
import { groveContext, version } from "harness-dispatch/grove";
import { lookUpCreator, reviewSelector } from "harness-dispatch/examples/review";
import { groveReviewSelector } from "harness-dispatch/examples/grove-review";
import type { Route } from "harness-dispatch/examples/static";

function harness(provider: string, model: string): Route {
  return (request) => ({ program: "fake-harness", args: [request.prompt], provider, model, effort: "high" });
}

const builder = harness("your-provider", "model-a");
const auditor = harness("your-other-provider", "model-b");

const reviews = { audit: { "your-provider": auditor, "your-other-provider": builder } } as const;

export const policy = definePolicy({
  schemaVersion: 2,
  version: `typecheck-grove-${version}`,
  ...groveReviewSelector({ routes: { build: builder }, reviews }),
});

// The adapter alone, in a loader of the owner's own: a refusal it returns is
// the loader's to return.
export const composed = definePolicy({
  schemaVersion: 2,
  version: "typecheck-grove-composed-1",
  loadContext(request, host) {
    const context = groveContext(request, host, reviews);
    return "status" in context ? context : lookUpCreator(context, host);
  },
  select: reviewSelector({ routes: {}, reviews }).select,
});

export const namedReviewer = groveReviewSelector({
  routes: {},
  // @ts-expect-error an entry gives each origin a route, not the name of one
  reviews: { audit: { "your-provider": "auditor" } },
});

export const selectingLoader = definePolicy({
  schemaVersion: 2,
  version: "typecheck-grove-selecting-1",
  // @ts-expect-error a loader returns a context or a refusal, never a selection
  loadContext: () => ({
    status: "selected",
    program: "fake-harness",
    args: [],
    provider: "p",
    model: "m",
    effort: "e",
    reason: "r",
  }),
  select: reviewSelector({ routes: {}, reviews }).select,
});

// @ts-expect-error the adapter reads through the host a loader receives
export const hostless = (request: SelectionRequest) => groveContext(request, reviews);
