// A Grove-owned sample: copy this file to ~/.config/ravel/config.rhai and edit
// it before use. Runtime: Ravel 2.x, Rhai, SCHEMA_VERSION 4. Its Ravel 1.x
// counterpart is `policy.ts`, with the same routes, choices and refusals. It is
// one owner's configuration for Grove, with the real `codex` and `claude`
// command lines. Read it before the first launch, and edit it: it is yours.
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
// `.ravel-choice` file in the directory Grove runs in, naming
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
// workspace, or anywhere else, it grants nothing more. Validate routing with
// fake agents on a private PATH; ravel start-session launches its selected
// command. It has no inspection verb. The caller supplies ending instructions.
//
// SKILL DIRECTORIES. Each harness receives `request.prompt`, the prompt that
// Ravel delivers. When the settings enable skill delivery, that prompt names
// the skill directories, and `request.skills` names their retained snapshots.
// Claude reads only its cwd and the directories that `--add-dir` names, so it
// is given each snapshot root. Codex runs with full access and reads every
// path, so it needs no grant, and neither does the notes writer, which runs
// Codex with full access. With delivery disabled, `request.skills` is `()`,
// and the sample grants nothing for skills.
//
// HOW IT DIFFERS FROM `policy.ts`. Ravel measures each read. A choice file
// beyond 4,096 bytes, or a jj store file beyond Ravel's source bound, ends the
// evaluation as Ravel's `source_too_large` instead of `choice_unreadable`. The
// names in the choice file and the jj store path are split and trimmed by
// Rhai, which classifies U+FEFF and U+0085 differently from JavaScript. A jj
// store file that is not UTF-8 refuses as `jj_store_unreadable`. Grove's
// delivery specification accepts these differences, in "Runtime parity".

export const SCHEMA_VERSION = 4;
export const VERSION = "grove sample 3";

/// A command line, and the model and effort it runs at unless a route says
/// otherwise. `harness_args` builds the arguments of each.
const HARNESSES = #{
    codex: #{ provider: "openai", model: "gpt-6.1-sol", effort: "medium", program: "codex" },
    claude: #{ provider: "anthropic", model: "claude-opus-5-5", effort: "medium", program: "claude" },
    // The headless one-shot writer for `grove run release-notes`. The release
    // task stages codex-headless.sh beside its inputs, in the directory it
    // runs, and the writer needs no grant beyond it.
    notes: #{ provider: "openai", model: "gpt-6.1-sol", effort: "medium", program: "/bin/bash" },
};

/// The arguments of the harness `name` for `prompt`. `grants` are the
/// directories beyond the cwd the harness may write, and `reads` the skill
/// snapshots it must be able to read.
fn harness_args(name, model, effort, prompt, grants, reads) {
    let args = [];
    if name == "codex" {
        args += ["--no-alt-screen", "--model", model, "-c", `model_reasoning_effort=${effort}`,
                 "-c", "default_permissions=:danger-full-access", "--ask-for-approval", "never"];
        for grant in grants { args += ["--add-dir", grant]; }
    } else if name == "claude" {
        for grant in grants { args += ["--add-dir", grant]; }
        for read in reads { args += ["--add-dir", read]; }
        args += ["--model", model, "--effort", effort];
    } else {
        args += ["codex-headless.sh", model, effort];
    }
    args.push(prompt);
    args
}

/// Kind to route, exactly. A kind not listed refuses; nothing falls back. A
/// route names its role in `by`, or the notes writer, and may set `model` and
/// `effort`.
const ROUTES = #{
    requirements: #{ by: "design-lead", effort: "max" },
    "review-requirements": #{ by: "design-review", effort: "xhigh" },
    "integrate-review-requirements": #{ by: "design-lead", effort: "xhigh" },

    design: #{ by: "design-lead", effort: "xhigh" },
    "review-design": #{ by: "design-review", effort: "xhigh" },
    "integrate-review-design": #{ by: "design-lead", effort: "xhigh" },

    planning: #{ by: "lead", effort: "xhigh" },
    "review-planning": #{ by: "review", effort: "xhigh" },
    "integrate-review-planning": #{ by: "lead", effort: "xhigh" },

    prototype: #{ by: "lead" },
    "review-prototype": #{ by: "review" },
    "integrate-review-prototype": #{ by: "lead" },

    impl: #{ by: "lead", effort: "high" },
    "review-impl": #{ by: "review", effort: "high" },
    "integrate-review-impl": #{ by: "lead" },

    // The research pair spans both providers.
    "research-a": #{ by: "lead", effort: "high" },
    "research-b": #{ by: "review", effort: "high" },
    "combine-research": #{ by: "lead", effort: "xhigh" },

    // Writing and drawing lead; the whole-document reads review.
    draft: #{ by: "design-lead", effort: "high" },
    "copy-edit": #{ by: "review" },
    art: #{ by: "design-lead", effort: "high" },
    proof: #{ by: "review" },
    finish: #{ by: "design-lead", effort: "medium" },

    // Standalone: `grove run release-notes`.
    "release-notes": #{ by: "notes" },
};

/// Which harness takes each role, and the kinds an arrangement routes its own
/// way. A Rhai map keeps its keys in sorted order, so `ARRANGEMENT_NAMES`
/// holds the order that the refusals name them in.
const ARRANGEMENT_NAMES = ["codex-led", "claude-led", "codex-design-claude-impl", "claude-design-codex-impl"];

const ARRANGEMENTS = #{
    "codex-led": #{
        roles: #{ "design-lead": "codex", "design-review": "claude", lead: "codex", review: "claude" },
        routes: #{},
    },
    "claude-led": #{
        roles: #{ "design-lead": "claude", "design-review": "codex", lead: "claude", review: "codex" },
        routes: #{},
    },
    // Codex leads requirements, design and writing; Claude leads planning,
    // prototypes and implementation. Reviews stay on the other provider.
    "codex-design-claude-impl": #{
        roles: #{ "design-lead": "codex", "design-review": "claude", lead: "claude", review: "codex" },
        routes: #{
            prototype: #{ model: "claude-sonnet-5-5" },
            "review-prototype": #{ model: "gpt-6.1-sol" },
            impl: #{ model: "claude-sonnet-5-5" },
            "copy-edit": #{ by: "design-review", model: "claude-sonnet-5-5", effort: "medium" },
            art: #{ by: "design-review", model: "claude-sonnet-5-5", effort: "medium" },
            proof: #{ by: "design-review", effort: "high" },
        },
    },
    // Its mirror across providers.
    "claude-design-codex-impl": #{
        roles: #{ "design-lead": "claude", "design-review": "codex", lead: "codex", review: "claude" },
        routes: #{
            prototype: #{ model: "gpt-6.1-sol" },
            "review-prototype": #{ model: "claude-sonnet-5-5" },
            impl: #{ model: "gpt-6.1-sol" },
            "copy-edit": #{ by: "design-review", model: "gpt-6.1-sol", effort: "medium" },
            art: #{ by: "design-review", model: "gpt-6.1-sol", effort: "medium" },
            proof: #{ by: "design-review", effort: "high" },
        },
    },
};

/// A change across an arrangement: to a harness's own model or effort, and to
/// routes. `MODIFIER_NAMES` holds their order, as `ARRANGEMENT_NAMES` does.
const MODIFIER_NAMES = ["codex-sol", "high-effort"];

const MODIFIERS = #{
    // Codex conservation: pins the Sol model and runs the three early reviews
    // at high rather than xhigh. It assumes those reviews are Codex's, as they
    // are under claude-led.
    "codex-sol": #{
        harnesses: #{ codex: #{ model: "gpt-6.1-sol" } },
        routes: #{
            "review-requirements": #{ effort: "high" },
            "review-design": #{ effort: "high" },
            "review-planning": #{ effort: "high" },
        },
    },
    // Raises both harnesses' own effort, which a kind runs at when its route
    // sets none. The notes writer is neither harness, and stays as it is.
    "high-effort": #{
        harnesses: #{ codex: #{ effort: "high" }, claude: #{ effort: "high" } },
        routes: #{},
    },
};

/// What is selected where no choice file says otherwise.
const DEFAULT = ["claude-led", "codex-sol"];

const CHOICE_FILE = ".ravel-choice";

/// The bound of the choice file, which is the bound of Ravel 1.1's `readChoice`.
const CHOICE_BYTES = 4096;

fn refused(code, message, remedy) {
    #{ status: "refused", code: code, message: message, remedy: remedy }
}

/// The names of `names`, joined by `separator`.
fn joined(names, separator) {
    let text = "";
    for name in names {
        if text != "" { text += separator; }
        text += name;
    }
    text
}

/// The member `key` of `map`, or `fallback` when `map` has none. Ravel's strict
/// mode fails on an absent property, so an optional member is read through
/// this.
fn member(map, key, fallback) {
    if map.contains(key) { map[key] } else { fallback }
}

/// The names in the choice file of `directory`, separated by whitespace, or
/// `()` when there is no such file. A file that cannot be read, and a name
/// that `offered` does not hold, give a refusal map. The read is measured, so
/// a file beyond `CHOICE_BYTES` ends the evaluation as `source_too_large`.
fn read_choice(directory, offered) {
    let file = ravel::path_resolve(directory, global::CHOICE_FILE);
    let text = "";
    try {
        text = ravel::read_text(file, global::CHOICE_BYTES).text;
    } catch (error) {
        if error.code == "source_missing" { return (); }
        let why = if error.code == "source_not_regular" {
            "is not a regular file"
        } else {
            `cannot be read: ${error.message}`
        };
        return choice_refusal(file, offered, "choice_unreadable", why);
    }
    let names = text.split().filter(|name| name != "");
    for name in names {
        if !offered.contains(name) {
            return choice_refusal(
                file,
                offered,
                "choice_unoffered",
                `names ${ravel::json_encode(name)}, which this policy does not offer; it offers ${joined(offered, ", ")}`
            );
        }
    }
    names
}

fn choice_refusal(file, offered, code, message) {
    refused(
        code,
        `the choice file ${file} ${message}`,
        `write names from ${joined(offered, ", ")} in ${file}, separated by whitespace, or delete it`
    )
}

/// The main repository a secondary jj workspace at `cwd` keeps its store in,
/// as a list of one grant, or an empty list. A secondary workspace's
/// `.jj/repo` is a file holding the store's path, relative to `.jj/`, and the
/// store is `<main>/.jj/repo`. The grant is `<main>` rather than the store
/// alone because a colocated repository's git objects sit beside the store, in
/// `<main>/.git`. A primary workspace's `.jj/repo` is the store itself, a
/// directory inside the cwd, and a directory with no `.jj` is no workspace:
/// neither needs a grant. A file that cannot be read, or that is empty, gives
/// a refusal map.
fn main_repository(cwd) {
    let file = ravel::path_resolve(cwd, ".jj/repo");
    let store = "";
    try {
        store = ravel::read_text(file).text;
    } catch (error) {
        if error.code == "source_missing" || error.code == "source_not_regular" { return []; }
        return unreadable_store(file, error.message);
    }
    store.trim();
    if store == "" { return unreadable_store(file, "it is empty"); }
    let jj_dir = ravel::path_resolve(cwd, ".jj");
    [ravel::path_parent(ravel::path_parent(ravel::path_resolve(jj_dir, store)))]
}

fn unreadable_store(file, why) {
    refused(
        "jj_store_unreadable",
        `${file} names no jj store: ${why}`,
        `run from a jj workspace whose ${file} names its store, or from a directory with no .jj`
    )
}

/// The root of each retained skill snapshot, or an empty list when skill
/// delivery is disabled and `skills` is `()`.
fn skill_reads(skills) {
    if skills == () { return []; }
    let reads = [];
    for source in skills.sources { reads.push(source.snapshotRoot); }
    reads
}

fn select(request, context) {
    let kind = ravel::json_encode(request.kind);
    if !global::ROUTES.contains(request.kind) {
        return refused(
            "incomplete_mapping",
            `ROUTES names no route for kind ${kind}`,
            "add a route for this kind to ROUTES in your policy"
        );
    }
    let base = global::ROUTES[request.kind];

    let offered = global::ARRANGEMENT_NAMES + global::MODIFIER_NAMES;
    let choice = read_choice(request.cwd, offered);
    if type_of(choice) == "map" { return choice; }
    let names = if choice == () { global::DEFAULT } else { choice };
    let from = if choice == () { "the default" } else { `chosen by ${global::CHOICE_FILE} in ${request.cwd}` };
    let arrangements = names.filter(|name| global::ARRANGEMENTS.contains(name));
    if arrangements.len() != 1 {
        return refused(
            "choice_arrangement",
            `${global::CHOICE_FILE} in ${request.cwd} names ${arrangements.len()} arrangements, and a selection has exactly one`,
            `name exactly one of ${joined(global::ARRANGEMENT_NAMES, ", ")} there, with any of ${joined(global::MODIFIER_NAMES, ", ")}`
        );
    }
    let arrangement = global::ARRANGEMENTS[arrangements[0]];
    let modifiers = names.filter(|name| global::MODIFIERS.contains(name)).map(|name| global::MODIFIERS[name]);

    // A later patch wins: the arrangement's over the route, then each
    // modifier's. A route's own model or effort wins over its harness's.
    let route = base;
    route.mixin(member(arrangement.routes, request.kind, #{}));
    for modifier in modifiers { route.mixin(member(modifier.routes, request.kind, #{})); }
    let name = if route.by == "notes" { "notes" } else { arrangement.roles[route.by] };
    let harness = global::HARNESSES[name];
    if name != "notes" {
        for modifier in modifiers { harness.mixin(member(modifier.harnesses, name, #{})); }
    }
    let model = member(route, "model", harness.model);
    let effort = member(route, "effort", harness.effort);

    let grants = main_repository(request.cwd);
    if type_of(grants) == "map" { return grants; }
    #{
        status: "selected",
        program: harness.program,
        args: harness_args(name, model, effort, request.prompt, grants, skill_reads(request.skills)),
        provider: harness.provider,
        model: model,
        effort: effort,
        reason: `${joined(names, " ")} (${from}) routes ${kind} to ${harness.program} with model ${model} at effort ${effort}`,
    }
}
