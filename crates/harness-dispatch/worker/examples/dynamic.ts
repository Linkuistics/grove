// harness-dispatch/examples/dynamic — a starter computed policy: the routes of
// harness-dispatch/examples/static, consulted by a `select` that also polices
// explicit choices.
//
// Use it whole from your personal policy, ~/.config/harness-dispatch/policy.ts:
//
//   export { policy } from "harness-dispatch/examples/dynamic";
//
// or call its `select` from a policy of your own over the same catalog. Better
// still, copy this file beside your policy and edit it. It is starting policy,
// not a recommendation. It takes its catalog and routes from the static
// example, whose comments justify each route's effort, so its wrapper, model
// and provider are that example's placeholders.
//
// WHAT IT COMPUTES. A routes table cannot say "an explicit choice may raise the
// effort for a task riskier than its kind, but never lower it below what the
// kind's route gives". That rule needs the choice and the table together, so it
// takes a `select`:
//
//   - with no choice, it applies the kind's route, and its reason names the
//     entry it applied, so inspection shows which one;
//   - with a choice, it accepts a candidate at or above the routed effort and
//     refuses one below it. A kind with no route sets no floor, so any
//     configured choice is accepted for it;
//   - a kind with no route and no choice refuses, in the policy's own words,
//     as the static table would. Nothing falls back.
//
// It is deterministic: the same request always gives the same result, with no
// clock, file, network or model involved, so a fake harness can test it.
// harness-dispatch has already checked the policy, catalog included, and that
// any choice names a candidate in it, before `select` is called.

import { definePolicy, type SelectionRequest, type SelectionResult } from "harness-dispatch/sdk";
import { catalog, routes } from "harness-dispatch/examples/static";

/** The static catalog's efforts, least to most. */
const efforts = ["low", "medium", "high", "xhigh"] as const;

/** Where a candidate's effort falls in {@link efforts}; -1 for no candidate. */
function effortRank(id: string): number {
  const candidate = catalog.find((entry) => entry.id === id);
  return candidate === undefined ? -1 : efforts.indexOf(candidate.effort);
}

/** The route for `kind`, if the table names one. */
function routeFor(kind: string): string | undefined {
  return Object.hasOwn(routes, kind) ? routes[kind as keyof typeof routes] : undefined;
}

export function select(request: SelectionRequest): SelectionResult {
  const kind = JSON.stringify(request.kind);
  const routed = routeFor(request.kind);
  const choice = request.explicitChoice;

  if (choice !== undefined) {
    if (routed === undefined) {
      return {
        status: "selected",
        candidateId: choice,
        reason: `the explicit choice "${choice}" is accepted: kind ${kind} has no route, so no effort floor applies`,
      };
    }
    if (effortRank(choice) >= effortRank(routed)) {
      return {
        status: "selected",
        candidateId: choice,
        reason: `the explicit choice "${choice}" is accepted: its effort is at least that of routes[${kind}], "${routed}"`,
      };
    }
    return {
      status: "refused",
      code: "effort_below_route",
      message: `the explicit choice "${choice}" has less effort than routes[${kind}], "${routed}", which is this kind's floor`,
      remedy: `choose "${routed}" or a candidate with more effort, or omit --choice to take the route`,
    };
  }

  if (routed === undefined) {
    return {
      status: "refused",
      code: "incomplete_mapping",
      message: `the routes name no candidate for kind ${kind}`,
      remedy: "add a route for this kind in your copy of the policy, or name one configured candidate with --choice",
    };
  }
  return { status: "selected", candidateId: routed, reason: `routes[${kind}] names candidate "${routed}"` };
}

/** The whole starter policy: the static catalog, and {@link select}. */
export const policy = definePolicy({
  schemaVersion: 1,
  version: "harness-dispatch/examples/dynamic 1",
  catalog,
  select,
});
