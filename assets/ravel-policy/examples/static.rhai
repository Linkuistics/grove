// Grove's copy of Ravel's `examples/static.rhai`. Runtime: Ravel 2.x, Rhai,
// SCHEMA_VERSION 4.
//
// Ravel 2.0.0 embeds no module, so a script imports this starter by its path.
// `grove-static.rhai` beside it imports it as `import "static" as starter;`.
// The 1.x TypeScript assets import the same starter as `ravel/examples/static`
// from the installed worker instead.
//
// Copied from Ravel commit 4f831feb4d0f with one change: the remedy of an
// unlisted kind keeps the words of the 1.x starter, "your copy of the policy",
// so the Rhai and TypeScript assets of Grove refuse alike.
//
// The starter route table: a kind maps to one command, exactly.
export const SCHEMA_VERSION = 4;
export const VERSION = "ravel/examples/static 3";

/// One harness and model at `effort`, with the prompt as its last argument.
fn agent(effort) {
    let model = "your-model";
    |request| #{
        program: "my-agent-wrapper",
        args: ["--model", model, "--effort", effort, request.prompt],
        provider: "your-provider",
        model: model,
        effort: effort,
    }
}

export const ROUTES = #{
    question: agent("low"),
    bugfix: agent("medium"),
    feature: agent("high"),
    migration: agent("xhigh"),
    architecture: agent("xhigh"),
};

fn select_route(routes, request) {
    let kind = ravel::json_encode(request.kind);
    if !routes.contains(request.kind) {
        return #{
            status: "refused",
            code: "incomplete_mapping",
            message: `the routes name no command for kind ${kind}`,
            remedy: "add a route for this kind in your copy of the policy",
        };
    }
    let command = routes[request.kind].call(request);
    command.status = "selected";
    command.reason = `routes[${kind}] gives ${command.program} with model ${command.model} at effort ${command.effort}`;
    command
}

fn select(request, context) {
    select_route(global::ROUTES, request)
}
