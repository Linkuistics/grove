// Grove examples/grove-review.rhai — Grove's review routing with the task-file
// adapter and the provider-evidence wrapper. Runtime: Ravel 2.x, Rhai,
// SCHEMA_VERSION 4. Its Ravel 1.x counterpart is `grove-review.ts`, with the
// same tables, refusals and receipts.
//
// It imports `review.rhai`, `grove-static.rhai` and `static.rhai` beside it,
// and `grove.rhai` and `provenance.rhai` above it, by their paths. Keep the
// tree together. Use it whole from your own configuration script,
// ~/.config/ravel/config.rhai:
//
//   import "/absolute/path/to/examples/grove-review" as grove_review;
//   export const SCHEMA_VERSION = 4;
//   export const VERSION = grove_review::VERSION;
//   fn load_context(request) { grove_review::load_context(request) }
//   fn select(request, context) { grove_review::select(request, context) }
//
// To compose tables of your own, pass them to `load_context_with` and
// `select_with`, and wrap the final selection once:
//
//   import "/absolute/path/to/provenance" as provenance;
//   const RULES = #{ routes: grove_review::ROUTES, reviews: grove_review::REVIEWS };
//   fn load_context(request) { grove_review::load_context_with(global::RULES, request) }
//   fn select(request, context) {
//       provenance::with_provenance(grove_review::select_with(global::RULES, request, context),
//           global::VERSION, request)
//   }
//
// `select_with` runs the review selector inside `within` of
// `grove-static.rhai`. Rhai finds the function of a closure only while a
// function of the module that made it runs, so the routes of
// `grove-static.rhai` resolve there, as do routes of your configuration script.

import "grove-static" as grove;
import "review" as review;
import "../grove" as adapter;
import "../provenance" as provenance;

export const SCHEMA_VERSION = 4;
export const VERSION = "grove/examples/grove-review 1";

// Preserve Grove's effort rationale: abstract reviews are the only check before
// downstream work; prototypes are disposable; code already has tests and types.
export const REVIEWS = #{
    "review-requirements": #{ openai: grove::review("xhigh"), anthropic: grove::lead("xhigh") },
    "review-design": #{ openai: grove::review("xhigh"), anthropic: grove::lead("xhigh") },
    "review-planning": #{ openai: grove::review("xhigh"), anthropic: grove::lead("xhigh") },
    "review-prototype": #{ openai: grove::review("medium"), anthropic: grove::lead("medium") },
    "review-impl": #{ openai: grove::review("high"), anthropic: grove::lead("high") },
};

/// The routes of `grove-static.rhai` without the review kinds.
fn without_reviews(routes, reviews) {
    for kind in reviews.keys() {
        routes.remove(kind);
    }
    routes
}

export const ROUTES = without_reviews(grove::ROUTES, REVIEWS);

/// The selection context of `request` from its task file, by `rules`.
fn load_context_with(rules, request) {
    adapter::grove_context(request, rules.reviews)
}

/// Select for `request` by `rules`, a map of `routes` and `reviews`.
fn select_with(rules, request, context) {
    grove::within(|| review::select_with(rules, request, context))
}

/// The selection context of the whole example.
fn load_context(request) {
    load_context_with(#{ routes: global::ROUTES, reviews: global::REVIEWS }, request)
}

/// The whole example, wrapped once so that finishing producers carry their
/// actual selected provider receipt. Missing receipts still require owner
/// declarations.
fn select(request, context) {
    let selection = select_with(#{ routes: global::ROUTES, reviews: global::REVIEWS }, request, context);
    provenance::with_provenance(selection, global::VERSION, request)
}
