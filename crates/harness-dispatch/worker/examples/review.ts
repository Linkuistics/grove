// harness-dispatch/examples/review — a starter review policy that applies the
// provider rule: every review it selects runs on a candidate whose provider
// origin differs from the original creator's.
//
// Use it whole from your personal policy, ~/.config/harness-dispatch/policy.ts:
//
//   export { policy } from "harness-dispatch/examples/review";
//
// or apply its rule to a catalog, routes and review kinds of your own:
//
//   import { definePolicy } from "harness-dispatch/sdk";
//   import { reviewSelector } from "harness-dispatch/examples/review";
//   export const policy = definePolicy({
//     schemaVersion: 1, version: "mine-1", catalog,
//     ...reviewSelector({ catalog, routes, reviews }),
//   });
//
// Better still, copy this file beside your policy and edit it. It is starting
// policy, not a recommendation. It builds on harness-dispatch/examples/static:
// that example's candidates and routes, whose comments justify each route's
// effort, and a second harness, from another origin, for reviews.
// `my-other-agent-wrapper` and `my-gateway-wrapper` are illustrative wrappers
// you supply, as `my-agent-wrapper` is, and the models and providers are
// placeholders for your own.
//
// WHAT A REVIEW NEEDS. A review is of an artifact, and its caller says which,
// with a `reviewedArtifact` in its `--context` document: the artifact's ID and
// its original creator, in one of two forms. `{ "run": <run ID> }` names the
// harness-dispatch run that created it, whose provider the record store holds;
// `{ "declared": <origin> }` is the owner's word for an artifact made without
// such a run. Nothing else supplies the creator, and a review without one
// refuses. No task file and no Grove convention is involved.
//
// THE RULE. For a kind listed in `reviews`, on every invocation, retry and
// explicit choice:
//
//   - a run reference is looked up, and a run the store does not hold, or one
//     whose harness never executed, refuses: it names no creator's origin;
//   - the creator's origin, recorded or declared, must be one of the current
//     catalog's origins, exactly, with no normalisation, so a relabelled
//     origin or a misspelt declaration refuses rather than passing as a
//     different provider;
//   - the reviewer is the explicit choice, or else the review kind's entry for
//     the creator's origin, and it must be of another origin. A gateway to a
//     model does not change the model's origin, so a candidate behind one
//     keeps its origin's label and cannot pass as another provider.
//
// Every failure refuses, naming its remedy. Nothing is ever chosen in place of
// the reviewer the rule refused. A kind `reviews` does not list takes the
// static routes, where the rule does not apply; so a context that names a
// reviewed artifact under such a kind refuses rather than taking a route.
//
// WHY EFFORT VARIES BY KIND. More reasoning effort pays where a mistake costs
// more to find and repair. The static example justifies its routes by six
// properties of the work: abstraction, uncertainty, consequences, downstream
// repair, reversibility and available checks. The review entries below are
// justified the same way, never by which model is better. These are priors for
// a kind of review, not calibrated estimates.

import {
  definePolicy,
  type Candidate,
  type Context,
  type ContextHost,
  type DeliveredContext,
  type Refused,
  type SelectionRequest,
  type SelectionResult,
} from "harness-dispatch/sdk";
import { catalog as staticCatalog, routes as staticRoutes } from "harness-dispatch/examples/static";

/**
 * A review kind's entry: for each provider origin a creator can have, the
 * candidate that reviews its work, which must be of another origin. An origin
 * the entry leaves out refuses as an incomplete mapping.
 */
export type ReviewEntry<Origin extends string = string, Id extends string = string> = {
  readonly [O in Origin]?: Id;
};

/** What the provider rule applies to: a catalog, its routes and its review kinds. */
export interface ReviewRules<C extends readonly Candidate[] = readonly Candidate[]> {
  /** The catalog whose origins a creator's must be among: the policy's own. */
  readonly catalog: C;
  /** Every kind that is not a review, to a candidate ID, exactly. */
  readonly routes: Readonly<Record<string, C[number]["id"]>>;
  /** Each review kind, exactly, to its entry. */
  readonly reviews: Readonly<Record<string, ReviewEntry<C[number]["provider"], C[number]["id"]>>>;
}

/** The two halves of a policy that applies the provider rule. */
export interface ReviewSelector {
  /**
   * The caller's context, or an empty one, with its creator run looked up
   * ({@link lookUpCreator}).
   */
  loadContext(request: SelectionRequest, host: ContextHost): Context;
  /** The review's reviewer under the rule, a route for any other kind, or a refusal. */
  select(request: SelectionRequest, context: DeliveredContext | undefined): SelectionResult;
}

/**
 * Look up the creator run `context`'s reviewed artifact names, if it names
 * one, and return `context` unchanged. harness-dispatch delivers the answer to
 * `select` in the context's `runs`, where no loader can write it, so the
 * provider `select` reads there is the record store's. Call it from a loader
 * that assembles its own context, with the context it will return.
 */
export function lookUpCreator(context: Context, host: ContextHost): Context {
  const run = context.reviewedArtifact?.creator?.run;
  if (run !== undefined) host.run(run);
  return context;
}

/** The provider rule over `rules`, as a `loadContext` and a `select`. */
export function reviewSelector<const C extends readonly Candidate[]>(rules: ReviewRules<C>): ReviewSelector {
  const catalog: readonly Candidate[] = rules.catalog;
  const routes: Readonly<Record<string, string>> = rules.routes;
  const reviews: Readonly<Record<string, Readonly<Record<string, string | undefined>>>> = rules.reviews;
  const byId = new Map(catalog.map((candidate) => [candidate.id, candidate] as const));
  const origins = [...new Set(catalog.map((candidate) => candidate.provider))].sort();
  const originList = origins.map(quote).join(", ");

  /** The candidates of any origin but `origin`, as a list to choose from. */
  function otherThan(origin: string): string {
    const others = catalog.filter((candidate) => candidate.provider !== origin).map((candidate) => quote(candidate.id));
    return others.length > 0 ? others.join(", ") : "none in the catalog; add one";
  }

  const declare = `declare the origin that made it instead: a creator { "declared": <origin> }, one of ${originList}`;
  const exactly = `the current catalog: ${originList}. Origins match exactly, with no normalisation`;

  function select(request: SelectionRequest, context: DeliveredContext | undefined): SelectionResult {
    const kind = request.kind;
    const artifact = context?.reviewedArtifact;
    const choice = request.explicitChoice;

    if (!Object.hasOwn(reviews, kind)) {
      if (artifact !== undefined) {
        return refuse(
          "review_kind_unlisted",
          `kind ${quote(kind)} is not a review kind this policy lists, ` +
            `but its context names reviewed artifact ${quote(artifact.id)}`,
          `list ${quote(kind)} in reviews in your copy of the policy, so the provider rule applies to it, ` +
            "or leave reviewedArtifact out if this is not a review",
        );
      }
      if (choice !== undefined) {
        return selected(choice, `the explicit choice ${quote(choice)} is taken for kind ${quote(kind)}, not a review`);
      }
      const routed = Object.hasOwn(routes, kind) ? routes[kind] : undefined;
      if (routed === undefined) {
        return refuse(
          "incomplete_mapping",
          `the routes name no candidate for kind ${quote(kind)}`,
          "add a route for this kind in your copy of the policy, or name one configured candidate with --choice",
        );
      }
      return selected(routed, `routes[${quote(kind)}] names candidate ${quote(routed)}`);
    }

    const entry = reviews[kind] ?? {};
    if (artifact === undefined) {
      return refuse(
        "reviewed_artifact_missing",
        `kind ${quote(kind)} is a review, but its context names no reviewed artifact, ` +
          "so its creator's origin is unknown",
        "supply a context whose reviewedArtifact names the artifact and its creator: " +
          `{ "id": <artifact ID>, "creator": { "run": <run ID> } }, or with { "declared": <origin> } as its creator`,
      );
    }
    const of = `reviewed artifact ${quote(artifact.id)}`;
    const reference = artifact.creator;
    if (reference === undefined) {
      return refuse(
        "creator_missing",
        `${of} names no creator, so its creator's origin is unknown`,
        `name the run that created it, a creator { "run": <run ID> }, or ${declare}`,
      );
    }

    // The creator's origin, and how it is known.
    let origin: string;
    let creator: string;
    if (reference.run !== undefined) {
      const run = reference.run;
      const lookup = context?.runs?.find((answer) => answer.runId === run);
      if (lookup === undefined) {
        return refuse(
          "creator_not_looked_up",
          `creator run ${run} of ${of} was not looked up, so its recorded origin is unknown`,
          "look it up in loadContext with lookUpCreator from harness-dispatch/examples/review, " +
            "as the example's own loader does",
        );
      }
      if (lookup.status === "missing") {
        return refuse(
          "creator_run_missing",
          `creator run ${run} of ${of} is not in this record store, so its origin is unknown`,
          "review with the record store the creator ran with (--state-dir), " +
            `or, if it was made without a run this store holds, ${declare}`,
        );
      }
      if (lookup.launchFailure !== null) {
        return refuse(
          "creator_not_executed",
          `creator run ${run} of ${of} never executed: harness-dispatch recorded a launch failure ` +
            `(${lookup.launchFailure.cause}), so it created nothing`,
          `name the run that did create the artifact, or ${declare}`,
        );
      }
      origin = lookup.candidate.provider;
      const task = lookup.taskId === null ? "no task" : `task ${quote(lookup.taskId)}`;
      creator = `creator run ${run} (${task}, kind ${quote(lookup.kind)}) recorded origin ${quote(origin)}`;
      if (!origins.includes(origin)) {
        return refuse(
          "creator_origin_unknown",
          `${creator}, which is not an origin in ${exactly}`,
          `if ${quote(origin)} was relabelled, restore that label in your catalog, ` +
            `since a new label is not a new provider; otherwise ${declare}`,
        );
      }
    } else {
      origin = reference.declared;
      creator = `creator origin ${quote(origin)} declared by the owner`;
      if (!origins.includes(origin)) {
        return refuse(
          "creator_origin_unknown",
          `declared creator origin ${quote(origin)} of ${of} is not an origin in ${exactly}`,
          `correct the declaration to one of ${originList}`,
        );
      }
    }

    // The reviewer: the explicit choice, or the entry's for this origin.
    const slot = `reviews[${quote(kind)}][${quote(origin)}]`;
    let id: string;
    let named: string;
    if (choice !== undefined) {
      id = choice;
      named = `the explicit choice ${quote(choice)}`;
    } else {
      const mapped = Object.hasOwn(entry, origin) ? entry[origin] : undefined;
      if (mapped === undefined) {
        return refuse(
          "incomplete_mapping",
          `reviews[${quote(kind)}] names no reviewer for creator origin ${quote(origin)}: ${creator}`,
          `add ${slot}, naming a candidate of another origin, in your copy of the policy, ` +
            `or name one with --choice: ${otherThan(origin)}`,
        );
      }
      id = mapped;
      named = `${slot} names ${quote(mapped)}`;
    }
    const reviewer = byId.get(id);
    if (reviewer === undefined) {
      return refuse(
        "reviewer_unknown",
        `${named}, which is not in the catalog`,
        `name a catalog candidate of another origin there: ${otherThan(origin)}`,
      );
    }
    if (reviewer.provider === origin) {
      return refuse(
        "same_origin",
        `${named}, of origin ${quote(reviewer.provider)}, the same as the creator's: ${creator}. ` +
          "A review must run on another origin, and a gateway to a model keeps the model's origin",
        choice !== undefined
          ? `choose a candidate of another origin: ${otherThan(origin)}`
          : `map ${slot} to a candidate of another origin in your copy of the policy, ` +
              `or name one with --choice: ${otherThan(origin)}`,
      );
    }
    return selected(
      id,
      `review kind ${quote(kind)} of ${quote(artifact.id)}: ${creator}; ` +
        `${named}, of origin ${quote(reviewer.provider)}`,
    );
  }

  return {
    loadContext: (request, host) => lookUpCreator(request.context ?? { schemaVersion: 1 }, host),
    select,
  };
}

function quote(text: string): string {
  return JSON.stringify(text);
}

function selected(candidateId: string, reason: string): SelectionResult {
  return { status: "selected", candidateId, reason };
}

function refuse(code: string, message: string, remedy: string): Refused {
  return { status: "refused", code, message, remedy };
}

const args = ["--model", { slot: "model" }, "--effort", { slot: "effort" }, { slot: "prompt" }] as const;
/** A second harness, from another origin, for reviews. */
const other = {
  provider: "your-other-provider",
  model: "your-other-model",
  program: "my-other-agent-wrapper",
  args,
} as const;
/**
 * The static example's model, reached through a gateway. The gateway changes
 * the program and the model's name, but not its origin, so its provider is the
 * static example's.
 */
const gateway = {
  provider: "your-provider",
  model: "your-gateway/your-model",
  program: "my-gateway-wrapper",
  args,
} as const;

/** The static example's candidates, and the others the reviews use. */
export const catalog = [
  ...staticCatalog,
  { id: "other-careful", effort: "high", ...other },
  { id: "other-deliberate", effort: "xhigh", ...other },
  // Listed so that a review can name it with --choice, and be refused: it is
  // the static example's origin, however it is reached.
  { id: "gateway-careful", effort: "high", ...gateway },
] as const satisfies readonly Candidate[];

/** A candidate ID from {@link catalog}. */
export type CandidateId = (typeof catalog)[number]["id"];

/** A provider origin in {@link catalog}. */
export type Origin = (typeof catalog)[number]["provider"];

/** Every kind that is not a review: the static example's routes, exactly. */
export const routes = staticRoutes;

/** Each review kind, exactly, from its creator's origin to a reviewer of the other. */
export const reviews = {
  // A code review reads a bounded change that tests and types already check.
  // It cannot run more than they do, and what it misses surfaces later, in
  // downstream repair that costs more than the review, so it gets the effort
  // of the feature work it reads.
  "code-review": { "your-provider": "other-careful", "your-other-provider": "careful" },
  // An architecture review is the only check abstract, uncertain work gets
  // before every later task builds on it. Nothing runs yet to catch a defect,
  // and one it misses is repaired by redoing that work, so it gets the most
  // effort.
  "architecture-review": { "your-provider": "other-deliberate", "your-other-provider": "deliberate" },
} as const satisfies Readonly<Record<string, ReviewEntry<Origin, CandidateId>>>;

/** The whole starter policy: {@link catalog}, {@link routes} and {@link reviews}, under the rule. */
export const policy = definePolicy({
  schemaVersion: 1,
  version: "harness-dispatch/examples/review 1",
  catalog,
  ...reviewSelector({ catalog, routes, reviews }),
});
