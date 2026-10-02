// A `select` that lets one checkout choose among the efforts it offers, with
// the SDK's choice-file helper. It is type-checked against the SDK's shipped
// declarations by `task dispatch:typecheck`, and evaluated through the
// compiled worker by the command-seam tests, so the declarations and the
// runtime agree on it. The helper's result is the names, nothing when the file
// is absent, or a refusal that `select` returns as it is.
import { CHOICE_FILE, definePolicy, readChoice } from "harness-dispatch/sdk";

const efforts: Readonly<Record<string, string>> = { fast: "low", careful: "high" };

export const policy = definePolicy({
  schemaVersion: 2,
  version: "typecheck-choice-1",
  select(request) {
    const choice = readChoice(request.cwd, Object.keys(efforts));
    if (choice !== undefined && "status" in choice) return choice;
    const [name = "careful", ...rest] = choice ?? [];
    const effort = efforts[name];
    if (effort === undefined || rest.length > 0) {
      return {
        status: "refused",
        code: "choice_not_one",
        message: `${CHOICE_FILE} in ${request.cwd} names ${rest.length + 1} efforts, and this policy takes one`,
        remedy: `name one of ${Object.keys(efforts).join(", ")} there`,
      };
    }
    return {
      status: "selected",
      program: "fake-harness",
      args: ["--effort", effort, request.prompt],
      provider: "origin-a",
      model: "model-large",
      effort,
      reason: `${name}, ${choice === undefined ? "the default" : `chosen by ${CHOICE_FILE}`}`,
    };
  },
});
