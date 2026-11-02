// Grove examples/review.rhai — caller-owned review routing through Ravel's
// generic selection context. Runtime: Ravel 2.x, Rhai, SCHEMA_VERSION 4. Its
// Ravel 1.x counterpart is `review.ts`, with the same tables and refusals.
//
// It imports `static.rhai` beside it and `provenance.rhai` above it by their
// paths, so keep the tree together. Use it whole from your own configuration
// script, ~/.config/ravel/config.rhai:
//
//   import "/absolute/path/to/examples/review" as review;
//   export const SCHEMA_VERSION = 4;
//   export const VERSION = review::VERSION;
//   fn select(request, context) { review::select(request, context) }
//
// To use tables of your own, pass them to `select_with`:
//
//   const RULES = #{ routes: review::ROUTES, reviews: #{ "code-review": #{ ... } } };
//   fn select(request, context) { review::select_with(global::RULES, request, context) }
//
// `select_with` calls each route through the `select_route` of `static.rhai`.
// Rhai finds the function of a closure only while a function of the module
// that made it runs, so a route resolves here when your configuration script,
// this file or `static.rhai` made it. A route of another module resolves when
// a function of that module calls `select_with`. `grove-review.rhai` does so
// for the routes of `grove-static.rhai`.
//
// Review facts are caller attestations, never a lookup of an earlier task
// attempt. A caller supplies them in `--context` as `facts.reviewedArtifact`
// and `facts.creator`. A provider alone is never creator evidence.
//
// Rhai lists the origins of a review table in code point order. `review.ts`
// sorts them the same way in its refusal.

import "static" as starter;
import "../provenance" as provenance;

export const SCHEMA_VERSION = 4;
export const VERSION = "grove/examples/review 1";

const RECEIPT_MEMBERS = ["schemaVersion", "attemptId", "provider", "policyVersion", "kind", "taskId"];

/// Whether `map` has only keys from `allowed`.
fn only(map, allowed) {
    for key in map.keys() {
        if !allowed.contains(key) {
            return false;
        }
    }
    true
}

/// Validate generic caller data. Returns the creator evidence, or `()`.
fn read_creator(value) {
    if type_of(value) != "map" || !provenance::label(value.get("provider")) {
        return ();
    }
    if value.get("evidence") == "declared" && only(value, ["evidence", "provider"]) {
        return #{ evidence: "declared", provider: value.provider };
    }
    if value.get("evidence") != "recorded" || !value.contains("receipt")
        || !only(value, ["evidence", "provider", "receipt"]) {
        return ();
    }
    let record = value.receipt;
    if type_of(record) != "map" || !only(record, global::RECEIPT_MEMBERS) {
        return ();
    }
    // Generic --context is parsed JSON, whose transport can reorder object
    // keys. Rebuild the canonical member order, then share the raw receipt's
    // validation. JSON.stringify writes a number equal to 1 as `1`, and a JSON
    // number such as `1.0` arrives here as a float.
    let canonical = #{};
    for name in global::RECEIPT_MEMBERS {
        if record.contains(name) {
            canonical[name] = record[name];
        }
    }
    let version = canonical.get("schemaVersion");
    if type_of(version) == "f64" && version == 1.0 {
        canonical.schemaVersion = 1;
    }
    // Each member came from JSON, so `json_encode` writes each one.
    let receipt = provenance::parse_evidence(provenance::receipt_text(canonical));
    if receipt != () && receipt.provider == value.provider {
        #{ evidence: "recorded", provider: value.provider, receipt: receipt }
    } else {
        ()
    }
}

fn refuse(code, message, remedy) {
    #{ status: "refused", code: code, message: message, remedy: remedy }
}

/// The command of `route` for `request`. The call passes through the
/// `select_route` of `static.rhai`, so that a closure of `static.rhai` resolves
/// as well as one of this file or of the configuration script.
fn command(route, request) {
    let table = #{};
    table[request.kind] = route;
    let command = starter::select_route(table, request);
    command.remove("status");
    command.remove("reason");
    command
}

/// The selection context for `select_with`: the caller's, or an empty one.
fn load_context(request) {
    if request.context == () { #{ schemaVersion: 1 } } else { request.context }
}

/// Select for `request` by `rules`, a map of `routes` and `reviews`. A review
/// kind maps each creator origin to the route of a reviewer of another origin.
fn select_with(rules, request, context) {
    let kind = request.kind;
    let facts = if context == () { () } else { context.get("facts") };
    let reported = facts != () && (facts.contains("reviewedArtifact") || facts.contains("creator"));
    if !rules.reviews.contains(kind) {
        if reported {
            return refuse("review_kind_unlisted",
                `kind ${ravel::json_encode(kind)} is not listed as a review but carries review facts`,
                "list the kind in reviews or remove the review facts for a nonreview task");
        }
        return starter::select_route(rules.routes, request);
    }
    let entry = rules.reviews[kind];
    let artifact = if facts == () { () } else { facts.get("reviewedArtifact") };
    if type_of(artifact) != "map" || !provenance::label(artifact.get("id")) {
        return refuse("reviewed_artifact_missing", `review kind ${ravel::json_encode(kind)} has no valid reviewed artifact`,
            "supply facts.reviewedArtifact.id and facts.creator, or use Grove's task-file adapter");
    }
    let id = ravel::json_encode(artifact.id);
    if !facts.contains("creator") {
        return refuse("creator_missing", `creator of ${id} is unknown`,
            "have the owner supply facts.creator with evidence declared and the original provider");
    }
    let creator = read_creator(facts.creator);
    if creator == () {
        return refuse("creator_invalid", `creator evidence for ${id} is invalid`,
            "repair the recorded receipt and matching provider or have the owner declare the original provider");
    }
    let origin = creator.provider;
    let description = if creator.evidence == "recorded" {
        `creator attempt ${creator.receipt.attemptId} (kind ${ravel::json_encode(creator.receipt.kind)}, task ${ravel::json_encode(creator.receipt.get("taskId"))}) recorded origin ${ravel::json_encode(origin)}`
    } else {
        `creator origin ${ravel::json_encode(origin)} declared by the owner`
    };
    if !entry.contains(origin) {
        let listed = "";
        for key in entry.keys() {
            if listed != "" {
                listed += ", ";
            }
            listed += ravel::json_encode(key);
        }
        if listed == "" {
            listed = "no origins";
        }
        return refuse("creator_origin_unlisted",
            `${description}; reviews[${ravel::json_encode(kind)}] lists ${listed}. Origins match exactly`,
            "correct the declaration or list the recorded origin in the review table; a relabelled provider is still the same provider");
    }
    let reviewer = command(entry[origin], request);
    if reviewer.provider == origin {
        return refuse("same_origin",
            `${description}; ${reviewer.program} has the same origin ${ravel::json_encode(reviewer.provider)}. A gateway keeps the model's origin`,
            `map reviews[${ravel::json_encode(kind)}][${ravel::json_encode(origin)}] to a command of another origin`);
    }
    reviewer.status = "selected";
    reviewer.reason = `review kind ${ravel::json_encode(kind)} of ${id}: ${description}; reviewer origin ${ravel::json_encode(reviewer.provider)}, effort ${reviewer.effort}`;
    reviewer
}

/// A harness of another provider at `effort`, with the prompt as its last
/// argument.
fn other_agent(effort) {
    |request| #{
        program: "my-other-agent-wrapper",
        args: ["--model", "your-other-model", "--effort", effort, request.prompt],
        provider: "your-other-provider",
        model: "your-other-model",
        effort: effort,
    }
}

export const ROUTES = starter::ROUTES;
export const REVIEWS = #{
    "code-review": #{ "your-provider": other_agent("high"), "your-other-provider": starter::agent("high") },
    "architecture-review": #{ "your-provider": other_agent("xhigh"), "your-other-provider": starter::agent("xhigh") },
};

/// The whole example: `ROUTES` and `REVIEWS`, selected by `select_with`.
fn select(request, context) {
    select_with(#{ routes: global::ROUTES, reviews: global::REVIEWS }, request, context)
}
