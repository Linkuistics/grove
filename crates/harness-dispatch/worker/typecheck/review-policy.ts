// The review example's selector as an owner's policy uses it, type-checked
// against its shipped declarations by `task dispatch:typecheck`, and evaluated
// through the compiled worker by the command-seam tests: an owner's own
// catalog, routes and review kind, under the example's provider rule. Each
// `@ts-expect-error` fails the type check if its line stops being an error, so
// the declarations are seen to refuse what the table must not say.
import { definePolicy, type Candidate } from "harness-dispatch/sdk";
import { lookUpCreator, reviewSelector } from "harness-dispatch/examples/review";

const args = [{ slot: "prompt" }] as const;

const catalog = [
  { id: "builder", provider: "your-provider", model: "model-a", effort: "high", program: "fake-harness", args },
  { id: "auditor", provider: "your-other-provider", model: "model-b", effort: "high", program: "fake-harness", args },
] as const satisfies readonly Candidate[];

export const policy = definePolicy({
  schemaVersion: 1,
  version: "typecheck-review-1",
  catalog,
  ...reviewSelector({
    catalog,
    routes: { feature: "builder" },
    reviews: { audit: { "your-provider": "auditor", "your-other-provider": "builder" } },
  }),
});

// The loader step alone, for a policy that assembles its own context first.
export const composed = definePolicy({
  schemaVersion: 1,
  version: "typecheck-review-composed-1",
  catalog,
  loadContext: (request, host) => lookUpCreator(request.context ?? { schemaVersion: 1 }, host),
  select: reviewSelector({ catalog, routes: {}, reviews: {} }).select,
});

export const unknownReviewer = reviewSelector({
  catalog,
  routes: {},
  // @ts-expect-error a reviewer is one of the catalog's candidate IDs
  reviews: { audit: { "your-provider": "auditer" } },
});

export const unknownOrigin = reviewSelector({
  catalog,
  routes: {},
  // @ts-expect-error an entry is keyed by the catalog's provider origins
  reviews: { audit: { "your-provdier": "auditor" } },
});

export const unknownRoute = reviewSelector({
  catalog,
  // @ts-expect-error a route names one of the catalog's candidate IDs
  routes: { feature: "biulder" },
  reviews: {},
});
