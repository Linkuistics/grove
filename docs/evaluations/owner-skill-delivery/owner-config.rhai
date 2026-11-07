// Your Ravel configuration script: which harness, model and effort each of
// Grove's session kinds launches, and the standalone `release-notes` kind that
// `grove run release-notes` launches. A kind not listed refuses.
//
// IT RUNS CODEX WITH APPROVALS OFF AND FULL ACCESS. The `codex` command below
// passes `--ask-for-approval never` and `default_permissions=:danger-full-access`,
// so a session it launches can change anything your account can.
//
// Two harnesses share the work by role: a lead produces and integrates, and the
// other provider reviews, runs the second research survey and makes the
// whole-document reads.
//
// WHERE IT RUNS. It reads no parameter and names no session. Grove runs it in
// the working-tree root, which the prompt assumes too. A secondary jj workspace
// keeps its store in the main repository, and its `.jj/repo` is a file naming
// that store; there both harnesses are given the main repository with
// `--add-dir`. In a primary workspace, or anywhere else, nothing more is
// granted. Validate routing with temporary fake agents on a private PATH;
// start-session launches the selected program, so it is not an inspection verb.
//
// This file converts `policy.ts` (policy schema 2) to Rhai for Ravel 2.0. Each
// routed kind gets the argument vector that `policy.ts` gives under Ravel 1.1.

export const SCHEMA_VERSION = 4;
export const VERSION = "harness-dispatch 3";

/// A harness: the program, and the labels its command is recorded under.
fn codex(model, effort) {
    #{ provider: "openai", model: model, effort: effort, program: "codex" }
}

fn claude(model, effort) {
    #{ provider: "anthropic", model: model, effort: effort, program: "claude" }
}

/// The command line of `harness`. `grants` are the directories beyond the cwd
/// that the harness may write.
fn harness_args(harness, prompt, grants) {
    let add_dirs = [];
    for grant in grants {
        add_dirs.push("--add-dir");
        add_dirs.push(grant);
    }
    if harness.program == "codex" {
        let args = [
            "--no-alt-screen",
            "--model",
            harness.model,
            "-c",
            `model_reasoning_effort=${harness.effort}`,
            "-c",
            "default_permissions=:danger-full-access",
            "--ask-for-approval",
            "never",
        ];
        args += add_dirs;
        args.push(prompt);
        args
    } else {
        let args = add_dirs;
        args += ["--model", harness.model, "--effort", harness.effort, prompt];
        args
    }
}

fn astra(effort) { codex("gpt-6-astra", effort) }
fn sol(effort) { codex("gpt-6.1-sol", effort) }
fn terra(effort) { codex("gpt-6.1-terra", effort) }
fn luna(effort) { codex("gpt-6.1-luna", effort) }

fn fable(effort) { claude("claude-fable-5-1", effort) }
fn opus(effort) { claude("claude-opus-5-5", effort) }
fn sonnet(effort) { claude("claude-sonnet-5-5", effort) }
fn haiku(effort) { claude("claude-haiku-5-5", effort) }

fn design_lead(effort) { sol(effort) }
fn design_review(effort) { opus(effort) }
fn lead(effort) { opus(effort) }
fn review(effort) { sol(effort) }

/// The harness of `kind`, or `()` for a kind that is not listed.
fn select_harness(kind) {
    switch kind {
        "requirements" => design_lead("max"),
        "review-requirements" => design_review("max"),
        "integrate-review-requirements" => design_lead("max"),

        "design" => design_lead("xhigh"),
        "review-design" => design_review("xhigh"),
        "integrate-review-design" => design_lead("xhigh"),

        "planning" => lead("high"),
        "review-planning" => review("high"),
        "integrate-review-planning" => lead("high"),

        "prototype" => lead("medium"),
        "review-prototype" => review("medium"),
        "integrate-review-prototype" => lead("medium"),

        "impl" => lead("high"),
        "review-impl" => review("high"),
        "integrate-review-impl" => lead("high"),

        "research-a" => design_lead("xhigh"),
        "research-b" => design_review("xhigh"),
        "combine-research" => design_lead("xhigh"),

        "draft" => opus("high"),
        "copy-edit" => terra("high"),
        "art" => terra("high"),
        "proof" => sol("medium"),
        "finish" => opus("medium"),

        "release-notes" => sonnet("medium"),

        _ => (),
    }
}

fn refused(code, message, remedy) {
    #{ status: "refused", code: code, message: message, remedy: remedy }
}

/// The main repository a secondary jj workspace at `cwd` keeps its store in,
/// as a list of zero or one directories, or a refusal map. A secondary
/// workspace's `.jj/repo` is a file holding the store's path, relative to
/// `.jj/`, and the store is `<main>/.jj/repo`. The grant is `<main>` rather
/// than the store alone because a colocated repository's git objects sit beside
/// the store, in `<main>/.git`. A primary workspace's `.jj/repo` is the store
/// itself, a directory inside the cwd, and a directory with no `.jj` is no
/// workspace: neither needs a grant.
fn main_repository(cwd) {
    let jj = ravel::path_resolve(cwd, ".jj");
    let file = ravel::path_resolve(jj, "repo");
    let store = "";
    try {
        store = ravel::read_text(file).text;
    } catch (error) {
        if error.code == "source_missing" || error.code == "source_not_regular" {
            return [];
        }
        return store_refusal(file, error.message);
    }
    store.trim();
    if store == "" {
        return store_refusal(file, "it is empty");
    }
    [ravel::path_parent(ravel::path_parent(ravel::path_resolve(jj, store)))]
}

fn store_refusal(file, why) {
    refused(
        "jj_store_unreadable",
        `${file} names no jj store: ${why}`,
        `run from a jj workspace whose ${file} names its store, or from a directory with no .jj`,
    )
}

fn select(request, context) {
    let kind = ravel::json_encode(request.kind);
    let harness = select_harness(request.kind);
    if type_of(harness) == "()" {
        return refused(
            "incomplete_mapping",
            `no mapping for kind ${kind}`,
            "add a mapping for this kind in your policy",
        );
    }

    let grants = main_repository(request.cwd);
    if type_of(grants) == "map" {
        return grants;
    }

    #{
        status: "selected",
        program: harness.program,
        args: harness_args(harness, request.prompt, grants),
        provider: harness.provider,
        model: harness.model,
        effort: harness.effort,
        reason: `routes ${kind} to ${harness.program} with model ${harness.model} at effort ${harness.effort}`,
    }
}
