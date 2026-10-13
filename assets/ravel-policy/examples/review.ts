// Caller-owned review routing through Ravel 1.0.0's generic context API.
// https://github.com/Linkuistics/Ravel/blob/v1.0.0/crates/ravel/worker/sdk/index.ts
import { definePolicy, type Context, type DeliveredContext, type Json, type Refused, type SelectionRequest, type SelectionResult } from "ravel/sdk";
import { agent, routes as staticRoutes, selectRoute, type Route } from "ravel/examples/static";
import { parseEvidence, type ProviderEvidence } from "../provenance.ts";

export type ReviewEntry = Readonly<Record<string, Route>>;
export interface ReviewRules {
  readonly routes: Readonly<Record<string, Route>>;
  readonly reviews: Readonly<Record<string, ReviewEntry>>;
}
export type Creator =
  | { readonly evidence: "declared"; readonly provider: string }
  | { readonly evidence: "recorded"; readonly provider: string; readonly receipt: ProviderEvidence };

function object(value: Json | undefined): { readonly [key: string]: Json } | undefined {
  return typeof value === "object" && value !== null && !Array.isArray(value)
    ? value as { readonly [key: string]: Json } : undefined;
}
function label(value: Json | undefined): value is string {
  return typeof value === "string" && value.trim() !== "" && !/[\r\n\0]/.test(value);
}

/** Validate generic caller data; a provider alone is never creator evidence. */
export function readCreator(value: Json | undefined): Creator | undefined {
  const fields = object(value);
  if (fields === undefined || !label(fields.provider)) return undefined;
  if (fields.evidence === "declared" && Object.keys(fields).every((key) => ["evidence", "provider"].includes(key))) {
    return { evidence: "declared", provider: fields.provider };
  }
  if (fields.evidence !== "recorded" || fields.receipt === undefined
    || Object.keys(fields).some((key) => !["evidence", "provider", "receipt"].includes(key))) return undefined;
  const record = object(fields.receipt);
  if (record === undefined || Object.keys(record).some((key) =>
    !["schemaVersion", "attemptId", "provider", "policyVersion", "kind", "taskId"].includes(key))) return undefined;
  // Generic --context is parsed JSON, whose transport can reorder object keys.
  // Reconstruct canonical field order, then share the raw receipt's validation.
  const receipt = parseEvidence(JSON.stringify({ schemaVersion: record.schemaVersion,
    attemptId: record.attemptId, provider: record.provider, policyVersion: record.policyVersion, kind: record.kind,
    ...(Object.hasOwn(record, "taskId") ? { taskId: record.taskId } : {}) }));
  return receipt !== undefined && receipt.provider === fields.provider
    ? { evidence: "recorded", provider: fields.provider, receipt } : undefined;
}

export interface ReviewSelector {
  loadContext(request: SelectionRequest): Context;
  select(request: SelectionRequest, context: DeliveredContext | undefined): SelectionResult;
}

/** Review facts are caller attestations, never a lookup of an earlier task attempt. */
export function reviewSelector(rules: ReviewRules): ReviewSelector {
  return {
    loadContext: (request) => request.context ?? { schemaVersion: 1 },
    select(request, context) {
      const { kind } = request;
      const facts = context?.facts;
      const entry = Object.hasOwn(rules.reviews, kind) ? rules.reviews[kind] : undefined;
      if (entry === undefined) {
        if (facts?.reviewedArtifact !== undefined || facts?.creator !== undefined) return refuse(
          "review_kind_unlisted", `kind ${JSON.stringify(kind)} is not listed as a review but carries review facts`,
          "list the kind in reviews or remove the review facts for a nonreview task");
        return selectRoute(rules.routes, request);
      }
      const artifact = object(facts?.reviewedArtifact);
      if (artifact === undefined || !label(artifact.id)) return refuse(
        "reviewed_artifact_missing", `review kind ${JSON.stringify(kind)} has no valid reviewed artifact`,
        "supply facts.reviewedArtifact.id and facts.creator, or use Grove's task-file adapter");
      if (facts?.creator === undefined) return refuse("creator_missing", `creator of ${JSON.stringify(artifact.id)} is unknown`,
        "have the owner supply facts.creator with evidence declared and the original provider");
      const creator = readCreator(facts.creator);
      if (creator === undefined) return refuse("creator_invalid", `creator evidence for ${JSON.stringify(artifact.id)} is invalid`,
        "repair the recorded receipt and matching provider or have the owner declare the original provider");
      const origin = creator.provider;
      const description = creator.evidence === "recorded"
        ? `creator attempt ${creator.receipt.attemptId} (kind ${JSON.stringify(creator.receipt.kind)}, task ${JSON.stringify(creator.receipt.taskId ?? null)}) recorded origin ${JSON.stringify(origin)}`
        : `creator origin ${JSON.stringify(origin)} declared by the owner`;
      const route = Object.hasOwn(entry, origin) ? entry[origin] : undefined;
      if (route === undefined) return refuse("creator_origin_unlisted",
        `${description}; reviews[${JSON.stringify(kind)}] lists ${Object.keys(entry).map((key) => JSON.stringify(key)).join(", ") || "no origins"}. Origins match exactly`,
        "correct the declaration or list the recorded origin in the review table; a relabelled provider is still the same provider");
      const reviewer = route(request);
      if (reviewer.provider === origin) return refuse("same_origin",
        `${description}; ${reviewer.program} has the same origin ${JSON.stringify(reviewer.provider)}. A gateway keeps the model's origin`,
        `map reviews[${JSON.stringify(kind)}][${JSON.stringify(origin)}] to a command of another origin`);
      return { ...reviewer, status: "selected", reason: `review kind ${JSON.stringify(kind)} of ${JSON.stringify(artifact.id)}: ${description}; reviewer origin ${JSON.stringify(reviewer.provider)}, effort ${reviewer.effort}` };
    },
  };
}

export function otherAgent(effort: string): Route {
  return (request) => ({ program: "my-other-agent-wrapper", args: ["--model", "your-other-model", "--effort", effort, request.prompt],
    provider: "your-other-provider", model: "your-other-model", effort });
}
export const routes = staticRoutes;
export const reviews = {
  "code-review": { "your-provider": otherAgent("high"), "your-other-provider": agent("high") },
  "architecture-review": { "your-provider": otherAgent("xhigh"), "your-other-provider": agent("xhigh") },
} as const satisfies Readonly<Record<string, ReviewEntry>>;
export const policy = definePolicy({ schemaVersion: 2, version: "grove/examples/review 1", ...reviewSelector({ routes, reviews }) });

function refuse(code: string, message: string, remedy: string): Refused {
  return { status: "refused", code, message, remedy };
}
