// The review example's selector as an owner's policy uses it, type-checked
// against its shipped declarations by `task dispatch:typecheck`, and evaluated
// through the compiled worker by the command-seam tests: an owner's own routes
// and review kind, under the example's provider rule. Each `@ts-expect-error`
// fails the type check if its line stops being an error, so the declarations
// are seen to refuse what the table must not say.
import { definePolicy } from "harness-dispatch/sdk";
import { lookUpCreator, reviewSelector } from "harness-dispatch/examples/review";
import type { Route } from "harness-dispatch/examples/static";

function harness(provider: string, model: string): Route {
  return (request) => ({ program: "fake-harness", args: [request.prompt], provider, model, effort: "high" });
}

const builder = harness("your-provider", "model-a");
const auditor = harness("your-other-provider", "model-b");

export const policy = definePolicy({
  schemaVersion: 2,
  version: "typecheck-review-1",
  ...reviewSelector({
    routes: { feature: builder },
    reviews: { audit: { "your-provider": auditor, "your-other-provider": builder } },
  }),
});

// The loader step alone, for a policy that assembles its own context first.
export const composed = definePolicy({
  schemaVersion: 2,
  version: "typecheck-review-composed-1",
  loadContext: (request, host) => lookUpCreator(request.context ?? { schemaVersion: 1 }, host),
  select: reviewSelector({ routes: {}, reviews: {} }).select,
});

export const namedReviewer = reviewSelector({
  routes: {},
  // @ts-expect-error an entry gives each origin a route, not the name of one
  reviews: { audit: { "your-provider": "auditor" } },
});

export const namedRoute = reviewSelector({
  // @ts-expect-error a route is a function of the request, not the name of one
  routes: { feature: "builder" },
  reviews: {},
});
