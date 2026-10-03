// The starter examples as an owner's policy uses them, type-checked against
// their shipped declarations by `task dispatch:typecheck`, and evaluated
// through the compiled worker by the command-seam tests: the Grove example's
// table, with one kind rerouted to a command of the owner's own.
import { definePolicy, type Policy } from "harness-dispatch/sdk";
import { routes, lead } from "harness-dispatch/examples/grove-static";
import { policy as generic, selectRoute, type Route } from "harness-dispatch/examples/static";

// The whole generic example is a policy as it stands.
export const whole: Policy = generic;

const mine: Route = (request) => ({
  program: "fake-harness",
  args: ["--label", request.params["label"] ?? "unlabelled", request.prompt],
  provider: "origin-a",
  model: "model-for-impl",
  effort: "high",
});

export const policy = definePolicy({
  schemaVersion: 2,
  version: "examples-fixture-1",
  select: (request) => selectRoute({ ...routes, impl: mine, spike: lead("low") }, request),
});

// @ts-expect-error a route returns a whole command: its labels are not optional
export const unlabelled: Route = (request) => ({ program: "fake-harness", args: [request.prompt] });
