// harness-dispatch/examples/review — a starter review policy that applies the
// provider rule: every review it selects runs a command whose provider origin
// differs from the original creator's.
//
// Use it whole from your personal policy, ~/.config/harness-dispatch/policy.ts:
//
//   export { policy } from "harness-dispatch/examples/review";
//
// or apply its rule to routes and review kinds of your own:
//
//   import { definePolicy } from "harness-dispatch/sdk";
//   import { reviewSelector } from "harness-dispatch/examples/review";
//   export const policy = definePolicy({
//     schemaVersion: 2, version: "mine-1",
//     ...reviewSelector({ routes, reviews }),
//   });
//
// Better still, copy this file beside your policy and edit it. It is starting
// policy, not a recommendation. It builds on harness-dispatch/examples/static:
// that example's routes, whose comments justify each route's effort, and a
// second harness, from another origin, for reviews. `my-other-agent-wrapper`
// is an illustrative wrapper you supply, as `my-agent-wrapper` is, and the
// models and providers are placeholders for your own.
//
// WHAT A REVIEW NEEDS. A review is of an artifact, and its caller says which,
// with a `reviewedArtifact` in its `--context` document: the artifact's ID and
// its original creator, in one of two forms. `{ "run": <run ID> }` names the
// harness-dispatch run that created it, whose provider the record store holds;
// `{ "declared": <origin> }` is the owner's word for an artifact made without
// such a run. Nothing else supplies the creator, and a review without one
// refuses. No task file and no Grove convention is involved.
//
// THE RULE. A review kind's entry in `reviews` maps each provider origin a
// creator can have to the command that reviews its work. For a kind listed
// there, on every invocation and retry:
//
//   - a run reference is looked up, and a run the store does not hold, or one
//     whose harness never executed, refuses: it names no creator's origin;
//   - the creator's origin, recorded or declared, must be one of the origins
//     the entry lists, exactly, with no normalisation, so a relabelled origin
//     or a misspelt declaration refuses rather than passing as a different
//     provider;
//   - the reviewer is the entry's command for that origin, and its provider
//     label must be another origin. A gateway to a model does not change the
//     model's origin, so a command that reaches the creator's model through
//     one keeps that origin's label and cannot pass as another provider.
//
// Every failure refuses, naming its remedy. Nothing is ever run in place of
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
  type Context,
  type ContextHost,
  type DeliveredContext,
  type Refused,
  type SelectionRequest,
  type SelectionResult,
} from "harness-dispatch/sdk";
import { agent, routes as staticRoutes, selectRoute, type Route } from "harness-dispatch/examples/static";

/**
 * A review kind's entry: for each provider origin a creator can have, the
 * command that reviews its work, which must carry another origin's label. An
 * origin the entry leaves out refuses, naming the origins it lists.
 */
export type ReviewEntry = Readonly<Record<string, Route>>;

/** What the provider rule applies to: the routes and the review kinds. */
export interface ReviewRules {
  /** Every kind that is not a review, to its command, exactly. */
  readonly routes: Readonly<Record<string, Route>>;
  /** Each review kind, exactly, to its entry. */
  readonly reviews: Readonly<Record<string, ReviewEntry>>;
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
export function reviewSelector(rules: ReviewRules): ReviewSelector {
  const { routes, reviews } = rules;

  function select(request: SelectionRequest, context: DeliveredContext | undefined): SelectionResult {
    const kind = request.kind;
    const artifact = context?.reviewedArtifact;
    const entry = Object.hasOwn(reviews, kind) ? reviews[kind] : undefined;

    if (entry === undefined) {
      if (artifact !== undefined) {
        return refuse(
          "review_kind_unlisted",
          `kind ${quote(kind)} is not a review kind this policy lists, ` +
            `but its context names reviewed artifact ${quote(artifact.id)}`,
          `list ${quote(kind)} in reviews in your copy of the policy, so the provider rule applies to it, ` +
            "or leave reviewedArtifact out if this is not a review",
        );
      }
      return selectRoute(routes, request);
    }

    const origins = Object.keys(entry).sort();
    const originList = origins.map(quote).join(", ");
    const declare = `declare the origin that made it instead: a creator { "declared": <origin> }, one of ${originList}`;
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
    let correct: string;
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
      origin = lookup.provider;
      const task = lookup.taskId === null ? "no task" : `task ${quote(lookup.taskId)}`;
      creator = `creator run ${run} (${task}, kind ${quote(lookup.kind)}) recorded origin ${quote(origin)}`;
      correct =
        `if ${quote(origin)} was relabelled, list that label in reviews[${quote(kind)}] in your copy of the policy, ` +
        `since a new label is not a new provider; otherwise ${declare}`;
    } else {
      origin = reference.declared;
      creator = `creator origin ${quote(origin)} declared by the owner`;
      correct = `correct the declaration to one of ${originList}`;
    }

    // The reviewer: the entry's command for this origin, of another origin.
    const at = `reviews[${quote(kind)}][${quote(origin)}]`;
    const route = Object.hasOwn(entry, origin) ? entry[origin] : undefined;
    if (route === undefined) {
      return refuse(
        "creator_origin_unlisted",
        `${creator}, which is not an origin reviews[${quote(kind)}] lists for ${of}: ${originList}. ` +
          "Origins match exactly, with no normalisation",
        correct,
      );
    }
    const reviewer = route(request);
    if (reviewer.provider === origin) {
      return refuse(
        "same_origin",
        `${at} gives ${reviewer.program}, of origin ${quote(reviewer.provider)}, the same as the creator's: ` +
          `${creator}. A review must run on another origin, and a gateway to a model keeps the model's origin`,
        `map ${at} to a command of another origin in your copy of the policy`,
      );
    }
    return {
      status: "selected",
      ...reviewer,
      reason:
        `review kind ${quote(kind)} of ${quote(artifact.id)}: ${creator}; ` +
        `${at} gives ${reviewer.program}, of origin ${quote(reviewer.provider)}, at effort ${reviewer.effort}`,
    };
  }

  return {
    loadContext: (request, host) => lookUpCreator(request.context ?? { schemaVersion: 1 }, host),
    select,
  };
}

function quote(text: string): string {
  return JSON.stringify(text);
}

function refuse(code: string, message: string, remedy: string): Refused {
  return { status: "refused", code, message, remedy };
}

/** A second harness, from another origin, for reviews, at `effort`. */
export function otherAgent(effort: string): Route {
  const model = "your-other-model";
  return (request) => ({
    program: "my-other-agent-wrapper",
    args: ["--model", model, "--effort", effort, request.prompt],
    provider: "your-other-provider",
    model,
    effort,
  });
}

/** Every kind that is not a review: the static example's routes, exactly. */
export const routes = staticRoutes;

/** Each review kind, exactly, from its creator's origin to a reviewer of the other. */
export const reviews = {
  // A code review reads a bounded change that tests and types already check.
  // It cannot run more than they do, and what it misses surfaces later, in
  // downstream repair that costs more than the review, so it gets the effort
  // of the feature work it reads.
  "code-review": { "your-provider": otherAgent("high"), "your-other-provider": agent("high") },
  // An architecture review is the only check abstract, uncertain work gets
  // before every later task builds on it. Nothing runs yet to catch a defect,
  // and one it misses is repaired by redoing that work, so it gets the most
  // effort.
  "architecture-review": { "your-provider": otherAgent("xhigh"), "your-other-provider": agent("xhigh") },
} as const satisfies Readonly<Record<string, ReviewEntry>>;

/** The whole starter policy: {@link routes} and {@link reviews}, under the rule. */
export const policy = definePolicy({
  schemaVersion: 2,
  version: "harness-dispatch/examples/review 2",
  ...reviewSelector({ routes, reviews }),
});
