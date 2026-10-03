// The sample policy, which `harness-dispatch init` installs as your own
// ~/.config/harness-dispatch/policy.ts. It is one owner's launch policy for
// Grove, with the real `codex` and `claude` command lines. Read it before the
// first launch, and edit it: it is yours.
//
// IT RUNS CODEX WITH APPROVALS OFF AND FULL ACCESS. The `codex` command below
// passes `--ask-for-approval never` and `default_permissions=:danger-full-access`,
// so a session it launches can change anything your account can. Remove those
// arguments if that is not what you want.
//
// WHAT IT ROUTES. Each of Grove's session kinds, and the standalone
// `release-notes` kind that `grove run release-notes` launches. A kind it does
// not list refuses. Two harnesses share the work by role: a lead produces and
// integrates, and the other provider reviews, runs the second research survey
// and makes the whole-document reads.
//
// WHAT IT OFFERS. Four arrangements say which harness takes which role, and
// two modifiers change model and effort across an arrangement. With no choice
// file it selects `DEFAULT`. One checkout selects differently by holding a
// `.harness-dispatch-choice` file in the directory Grove runs in, naming
// exactly one arrangement and any modifiers, separated by whitespace:
//
//     codex-design-claude-impl high-effort
//
// That file replaces the default whole, so a modifier the default applies
// must be named again to keep it. It can name only what this file offers.
//
// WHERE IT RUNS. Nothing but the kind and the prompt: it reads no parameter
// and names no session. Grove runs it in the working-tree root, which the
// prompt assumes too. A secondary jj workspace keeps its store in the main
// repository, and its `.jj/repo` is a file naming that store; there the sample
// gives both harnesses the main repository with `--add-dir`. In a primary
// workspace, or anywhere else, it grants nothing more. Inspect from the
// directory Grove would run in:
//
//     cd /work/parser && harness-dispatch inspect --kind impl

import { readFileSync, statSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { CHOICE_FILE, definePolicy, readChoice, type Refused, type SelectionResult } from "harness-dispatch/sdk";

/** A command line, and the model and effort it runs at unless a route says otherwise. */
interface Harness {
  readonly provider: string;
  readonly model: string;
  readonly effort: string;
  readonly program: string;
  /** `grants` are the directories beyond the cwd the harness may write. */
  args(model: string, effort: string, prompt: string, grants: readonly string[]): string[];
}

const HARNESSES = {
  codex: {
    provider: "openai",
    model: "gpt-6.1-sol",
    effort: "medium",
    program: "codex",
    args: (model, effort, prompt, grants) => [
      "--model",
      model,
      "-c",
      `model_reasoning_effort=${effort}`,
      "-c",
      "default_permissions=:danger-full-access",
      "--ask-for-approval",
      "never",
      ...grants.flatMap((grant) => ["--add-dir", grant]),
      prompt,
    ],
  },
  claude: {
    provider: "anthropic",
    model: "claude-opus-5-5",
    effort: "medium",
    program: "claude",
    args: (model, effort, prompt, grants) => [
      ...grants.flatMap((grant) => ["--add-dir", grant]),
      "--model",
      model,
      "--effort",
      effort,
      prompt,
    ],
  },
  // The headless one-shot writer for `grove run release-notes`. The release
  // task stages codex-headless.sh beside its inputs, in the directory it runs,
  // and the writer needs no grant beyond it.
  notes: {
    provider: "openai",
    model: "gpt-6.1-sol",
    effort: "medium",
    program: "/bin/bash",
    args: (model, effort, prompt) => ["codex-headless.sh", model, effort, prompt],
  },
} as const satisfies Readonly<Record<string, Harness>>;

/** The two interactive harnesses an arrangement shares the roles between. */
type Agent = "codex" | "claude";

/** Who does a kind's work: producers and integrations lead, adversarial reads review. */
type Role = "design-lead" | "design-review" | "lead" | "review";

/** A kind's role, or the notes writer, and the model and effort it sets. */
interface Route {
  readonly by: Role | "notes";
  readonly model?: string;
  readonly effort?: string;
}

/** What an arrangement or a modifier changes about a kind's route. */
type Patches = Readonly<Record<string, Partial<Route>>>;

/** Kind to route, exactly. A kind not listed refuses; nothing falls back. */
const ROUTES: Readonly<Record<string, Route>> = {
  requirements: { by: "design-lead", effort: "max" },
  "review-requirements": { by: "design-review", effort: "xhigh" },
  "integrate-review-requirements": { by: "design-lead", effort: "xhigh" },

  design: { by: "design-lead", effort: "xhigh" },
  "review-design": { by: "design-review", effort: "xhigh" },
  "integrate-review-design": { by: "design-lead", effort: "xhigh" },

  planning: { by: "lead", effort: "xhigh" },
  "review-planning": { by: "review", effort: "xhigh" },
  "integrate-review-planning": { by: "lead", effort: "xhigh" },

  prototype: { by: "lead" },
  "review-prototype": { by: "review" },
  "integrate-review-prototype": { by: "lead" },

  impl: { by: "lead", effort: "high" },
  "review-impl": { by: "review", effort: "high" },
  "integrate-review-impl": { by: "lead" },

  // The research pair spans both providers.
  "research-a": { by: "lead", effort: "high" },
  "research-b": { by: "review", effort: "high" },
  "combine-research": { by: "lead", effort: "xhigh" },

  // Writing and drawing lead; the whole-document reads review.
  draft: { by: "design-lead", effort: "high" },
  "copy-edit": { by: "review" },
  art: { by: "design-lead", effort: "high" },
  proof: { by: "review" },
  finish: { by: "design-lead", effort: "medium" },

  // Standalone: `grove run release-notes`.
  "release-notes": { by: "notes" },
};

/** Which harness takes each role, and the kinds an arrangement routes its own way. */
interface Arrangement {
  readonly roles: Readonly<Record<Role, Agent>>;
  readonly routes?: Patches;
}

const ARRANGEMENTS: Readonly<Record<string, Arrangement>> = {
  "codex-led": {
    roles: { "design-lead": "codex", "design-review": "claude", lead: "codex", review: "claude" },
  },
  "claude-led": {
    roles: { "design-lead": "claude", "design-review": "codex", lead: "claude", review: "codex" },
  },
  // Codex leads requirements, design and writing; Claude leads planning,
  // prototypes and implementation. Reviews stay on the other provider.
  "codex-design-claude-impl": {
    roles: { "design-lead": "codex", "design-review": "claude", lead: "claude", review: "codex" },
    routes: {
      prototype: { model: "claude-sonnet-5-5" },
      "review-prototype": { model: "gpt-6.1-sol" },
      impl: { model: "claude-sonnet-5-5" },
      "copy-edit": { by: "design-review", model: "claude-sonnet-5-5", effort: "medium" },
      art: { by: "design-review", model: "claude-sonnet-5-5", effort: "medium" },
      proof: { by: "design-review", effort: "high" },
    },
  },
  // Its mirror across providers.
  "claude-design-codex-impl": {
    roles: { "design-lead": "claude", "design-review": "codex", lead: "codex", review: "claude" },
    routes: {
      prototype: { model: "gpt-6.1-sol" },
      "review-prototype": { model: "claude-sonnet-5-5" },
      impl: { model: "gpt-6.1-sol" },
      "copy-edit": { by: "design-review", model: "gpt-6.1-sol", effort: "medium" },
      art: { by: "design-review", model: "gpt-6.1-sol", effort: "medium" },
      proof: { by: "design-review", effort: "high" },
    },
  },
};

/** A change across an arrangement: to a harness's own model or effort, and to routes. */
interface Modifier {
  readonly harnesses?: Readonly<Partial<Record<Agent, { readonly model?: string; readonly effort?: string }>>>;
  readonly routes?: Patches;
}

const MODIFIERS: Readonly<Record<string, Modifier>> = {
  // Codex conservation: pins the Sol model and runs the three early reviews at
  // high rather than xhigh. It assumes those reviews are Codex's, as they are
  // under claude-led.
  "codex-sol": {
    harnesses: { codex: { model: "gpt-6.1-sol" } },
    routes: {
      "review-requirements": { effort: "high" },
      "review-design": { effort: "high" },
      "review-planning": { effort: "high" },
    },
  },
  // Raises both harnesses' own effort, which a kind runs at when its route
  // sets none. The notes writer is neither harness, and stays as it is.
  "high-effort": {
    harnesses: { codex: { effort: "high" }, claude: { effort: "high" } },
  },
};

/** What is selected where no choice file says otherwise. */
const DEFAULT: readonly string[] = ["claude-led", "codex-sol"];

function refused(code: string, message: string, remedy: string): Refused {
  return { status: "refused", code, message, remedy };
}

/**
 * The main repository a secondary jj workspace at `cwd` keeps its store in, or
 * nothing. A secondary workspace's `.jj/repo` is a file holding the store's
 * path, relative to `.jj/`, and the store is `<main>/.jj/repo`. The grant is
 * `<main>` rather than the store alone because a colocated repository's git
 * objects sit beside the store, in `<main>/.git`. A primary workspace's
 * `.jj/repo` is the store itself, a directory inside the cwd, and a directory
 * with no `.jj` is no workspace: neither needs a grant.
 */
function mainRepository(cwd: string): string[] | Refused {
  const file = join(cwd, ".jj", "repo");
  try {
    if (statSync(file, { throwIfNoEntry: false })?.isFile() !== true) return [];
    const store = readFileSync(file, "utf8").trim();
    if (store === "") throw new Error("it is empty");
    return [dirname(dirname(resolve(join(cwd, ".jj"), store)))];
  } catch (error) {
    return refused(
      "jj_store_unreadable",
      `${file} names no jj store: ${error instanceof Error ? error.message : String(error)}`,
      `run from a jj workspace whose ${file} names its store, or from a directory with no .jj`,
    );
  }
}

export const policy = definePolicy({
  schemaVersion: 2,
  version: "harness-dispatch sample 2",
  select(request): SelectionResult {
    const kind = JSON.stringify(request.kind);
    const base = Object.hasOwn(ROUTES, request.kind) ? ROUTES[request.kind] : undefined;
    if (base === undefined) {
      return refused(
        "incomplete_mapping",
        `ROUTES names no route for kind ${kind}`,
        "add a route for this kind to ROUTES in your policy",
      );
    }

    const arrangementNames = Object.keys(ARRANGEMENTS);
    const modifierNames = Object.keys(MODIFIERS);
    const choice = readChoice(request.cwd, [...arrangementNames, ...modifierNames]);
    if (choice !== undefined && "status" in choice) return choice;
    const names = choice ?? DEFAULT;
    const from = choice === undefined ? "the default" : `chosen by ${CHOICE_FILE} in ${request.cwd}`;
    const arrangements = names.flatMap((name) => ARRANGEMENTS[name] ?? []);
    const [arrangement] = arrangements;
    if (arrangement === undefined || arrangements.length > 1) {
      return refused(
        "choice_arrangement",
        `${CHOICE_FILE} in ${request.cwd} names ${arrangements.length} arrangements, and a selection has exactly one`,
        `name exactly one of ${arrangementNames.join(", ")} there, with any of ${modifierNames.join(", ")}`,
      );
    }
    const modifiers = names.flatMap((name) => MODIFIERS[name] ?? []);

    // A later patch wins: the arrangement's over the route, then each
    // modifier's. A route's own model or effort wins over its harness's.
    const route: Route = Object.assign(
      {},
      base,
      arrangement.routes?.[request.kind],
      ...modifiers.map((modifier) => modifier.routes?.[request.kind]),
    );
    const name = route.by === "notes" ? "notes" : arrangement.roles[route.by];
    const harness: Harness = Object.assign(
      {},
      HARNESSES[name],
      ...modifiers.map((modifier) => (name === "notes" ? undefined : modifier.harnesses?.[name])),
    );
    const model = route.model ?? harness.model;
    const effort = route.effort ?? harness.effort;

    const grants = mainRepository(request.cwd);
    if ("status" in grants) return grants;
    return {
      status: "selected",
      program: harness.program,
      args: harness.args(model, effort, request.prompt, grants),
      provider: harness.provider,
      model,
      effort,
      reason: `${names.join(" ")} (${from}) routes ${kind} to ${harness.program} with model ${model} at effort ${effort}`,
    };
  },
});
