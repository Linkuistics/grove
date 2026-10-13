// Ordinary caller-owned file imports, with Ravel's public generic interfaces.
import { definePolicy, type Policy, type SelectionRequest } from "ravel/sdk";
import { selectRoute, type Route } from "ravel/examples/static";
import { lead, review, routes, policy as staticPolicy } from "../examples/grove-static.ts";
import { policy as samplePolicy } from "../sample/policy.ts";

const policies: readonly Policy[] = [staticPolicy, samplePolicy];
const customRoutes: Readonly<Record<string, Route>> = {
  ...routes,
  "my-implementation": lead("high"),
  "my-review": review("high"),
};
export const policy = definePolicy({
  schemaVersion: 2,
  version: "ordinary-import-fixture",
  select: (request) => selectRoute(customRoutes, request),
});

// The imported route API needs the full public request; it accepts no arbitrary
// executable text in place of a route function.
declare const request: SelectionRequest;
lead("high")(request);
// @ts-expect-error a static route table holds command builders, not strings
const invalidRoute: Route = "my-agent";
void invalidRoute;
void policies;
