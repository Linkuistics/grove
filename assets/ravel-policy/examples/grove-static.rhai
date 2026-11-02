// Grove examples/grove-static.rhai — a starter configuration for Grove's
// session kinds: a `select` that consults an exact table, each kind mapped to
// one command. Runtime: Ravel 2.x, Rhai, SCHEMA_VERSION 4. Its Ravel 1.x
// counterpart is `grove-static.ts`, with the same table.
//
// It imports `static.rhai` by its path, so keep the two files together. Use it
// whole from your own configuration script, ~/.config/ravel/config.rhai:
//
//   import "/absolute/path/to/examples/grove-static" as grove;
//   export const SCHEMA_VERSION = 4;
//   export const VERSION = "my-config 1";
//   fn select(request, context) { grove::select(request, context) }
//
// To add kinds of your own, extend `ROUTES` and select through this file's
// `select_route`:
//
//   const ROUTES = grove::ROUTES + #{ "my-implementation": grove::lead("high") };
//   fn select(request, context) { grove::select_route(global::ROUTES, request) }
//
// Call `grove::select_route`, not the `select_route` of `static.rhai`. A route
// of this file is a closure, and Rhai finds a closure's function only while a
// function of the module that made it is running.
//
// Better still, copy this file and `static.rhai` beside your configuration
// script and edit them: change `lead` and `review` to your own two harnesses
// and keep the table. It is a starting configuration, not a recommendation.
// Grove passes the kind; this file never reads a task file or a Grove filename.
//
// Two harnesses share the work, as in the sample configuration. A lead
// produces and integrates; a reviewer from another provider reviews, runs the
// second research survey, copy-edits and proofs. `my-codex-wrapper` and
// `my-claude-wrapper` are illustrative wrappers you supply, not shipped tools:
// each receives `--model`, `--effort` and the prompt, and should exec its
// harness with them. Swap the two to lead with the other provider. The models
// are placeholders for your own. A table keyed by kind cannot know which
// provider actually created the artifact a review reads, so it enforces no
// provider rule; that takes a configuration with caller-owned creator
// evidence.
//
// Each command receives `request.prompt`, the prompt that Ravel delivers.
// With skill delivery enabled, that prompt holds the skill directories; the
// wrappers you supply then grant their harness reads of
// `request.skills.sources[*].snapshotRoot` where its sandbox needs them.
//
// WHY EFFORT VARIES BY KIND. More reasoning effort pays where a mistake costs
// more to find and repair. Each route below is justified by six properties of
// the work, never by which model is better:
//
//   - abstraction: how far the output is from something that can be run or read;
//   - uncertainty: how much must be inferred rather than read from the task;
//   - consequences: what a mistake breaks, and for whom;
//   - downstream repair: how much later work a mistake forces to be redone;
//   - reversibility: whether the result can be undone cheaply once acted on;
//   - available checks: whether tests, types or a later reader catch a mistake.
//
// These are priors for a kind of session, not calibrated estimates. A task
// that is smaller or riskier than its kind suggests deserves a configuration
// of your own that reads more than the kind. Effort labels are each harness's
// own vocabulary, not quantities comparable across vendors.

import "static" as starter;

export const SCHEMA_VERSION = 4;
export const VERSION = "grove-static 3";

/// One harness's command at `effort`, with the prompt as its last argument.
fn command(program, provider, model, effort) {
    |request| #{
        program: program,
        args: ["--model", model, "--effort", effort, request.prompt],
        provider: provider,
        model: model,
        effort: effort,
    }
}

/// The harness that produces and integrates, at an effort.
fn lead(effort) { command("my-codex-wrapper", "openai", "your-codex-model", effort) }

/// A harness from another provider, for reviews, at an effort.
fn review(effort) { command("my-claude-wrapper", "anthropic", "your-claude-model", effort) }

/// Grove's session kinds to commands, exactly. A kind not listed refuses.
export const ROUTES = #{
    // Requirements are the most abstract work in the tree and the most
    // uncertain: the human's intent has to be drawn out, not read. No test
    // exists yet to catch a gap, and a missed requirement is repaired by
    // redoing the design, plan and code built on it.
    requirements: lead("max"),
    // The review is the only check requirements get before design builds on
    // them, and its triage can change the agreed contract, so both carry the
    // requirements' downstream reach.
    "review-requirements": review("xhigh"),
    "integrate-review-requirements": lead("xhigh"),

    // Design fixes abstract, coupled decisions that are costly to reverse once
    // increments build on them. Its checks are arguments and models rather
    // than tests, and an architectural defect survives local tests into every
    // increment, so its review and triage keep the same effort.
    design: lead("xhigh"),
    "review-design": review("xhigh"),
    "integrate-review-design": lead("xhigh"),

    // A plan's increment boundaries and dependencies shape every later
    // session. A wrong cut is found only when an increment cannot land, and is
    // repaired by re-planning and reworking finished leaves.
    planning: lead("xhigh"),
    "review-planning": review("xhigh"),
    "integrate-review-planning": lead("xhigh"),

    // A prototype is thrown away by design. It is cheap to redo and wholly
    // reversible, and its value is the reaction it provokes, which a person
    // checks at once.
    prototype: lead("medium"),
    "review-prototype": review("medium"),
    "integrate-review-prototype": lead("medium"),

    // Implementation is concrete and bounded by a settled design, and tests,
    // types and review check it; a defect is local and repaired in place.
    // Raise it for invariants or concurrency the compiler cannot check. The
    // review cannot run more than the tests already do, and what it misses
    // surfaces later, where repair costs more. Triage of findings against
    // checked code is bounded and reversible.
    impl: lead("high"),
    "review-impl": review("high"),
    "integrate-review-impl": lead("medium"),

    // A survey's uncertainty is the field itself, and only the second,
    // independent survey and the combiner check it, so each is thorough. The
    // second runs on the other provider to stay independent. The combiner must
    // challenge claims both surveys share; what it lets through informs design
    // with no later check.
    "research-a": lead("high"),
    "research-b": review("high"),
    "combine-research": lead("xhigh"),

    // A draft owns structure and technical truth. Copy-edit and proof follow
    // and catch much, but not wrong substance. Copy-editing and figures are
    // local, reversible work within a fixed structure, and proof checks them.
    // Proof is the final whole-document read, with nothing after it to catch
    // what it misses.
    draft: lead("high"),
    "copy-edit": review("medium"),
    art: lead("medium"),
    proof: review("high"),

    // Most finishing steps are explicit and checked by the tools that run
    // them. Raise it where a project's finish integrates and releases, which
    // cannot be undone once published.
    finish: lead("medium"),
};

/// The command that `routes` gives for the kind of `request`, or the refusal
/// of `static.rhai` for a kind that `routes` does not list. The closures of
/// this file resolve only while a function of this module runs, so an owner
/// table that holds them selects through here.
fn select_route(routes, request) {
    starter::select_route(routes, request)
}

/// Call `selection`, a closure without arguments, while a function of this
/// module runs. A route of this file then resolves inside it, at any depth.
/// `grove-review.rhai` runs its review selector this way.
fn within(selection) {
    selection.call()
}

/// The whole starter configuration: `ROUTES`, selected by kind.
fn select(request, context) {
    select_route(global::ROUTES, request)
}
