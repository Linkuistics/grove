// Public assets compose against only the independent installed SDK.
import { definePolicy, type Context, type LoadContext } from "ravel/sdk";
import { agent } from "ravel/examples/static";
import { groveContext } from "../grove.ts";
import { withProvenance } from "../provenance.ts";
import { reviewSelector, type ReviewRules } from "../examples/review.ts";
import { groveReviewSelector, policy } from "../examples/grove-review.ts";

const rules = { routes: { impl: agent("high") }, reviews: { "review-impl": { openai: agent("high") } } } satisfies ReviewRules;
const generic = withProvenance(definePolicy({ schemaVersion: 2, version: "types", ...reviewSelector(rules) }));
const grove = definePolicy({ schemaVersion: 2, version: "types", ...groveReviewSelector(rules) });
const loader: LoadContext = (request, host) => groveContext(request, host, rules.reviews);
const caller: Context = { schemaVersion: 1, facts: { reviewedArtifact: { id: "external" }, creator: { evidence: "declared", provider: "openai" } } };
void [generic, grove, policy, loader, caller];
