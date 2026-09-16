# Proving a negative
<!-- book-page id="proving-a-negative" slice="closure-proved" order="4" -->
[Previous: Three steps](03-three-steps.md) | [Contents](README.md) | [Next: What the call reaches](05-what-the-call-reaches.md)

<a id="closure-proved"></a>
## The closure tests

The binary tests two different contracts: exactly the top-level subcommands `config` and `view`, with `show` beneath
`config`, and
no bare lifecycle selector arguments; and a description for every listed
command and argument. The first keeps launch policy out of the parser. The
second exercises nested inspection help and the kind filter as well as viewer help.

<a id="the-block"></a>
## The test module as a source partition

The composite keeps source order while the sections below explain the two
properties separately. These are binary unit tests: they inspect the same clap
model that production parsing builds.

<!-- fragment «surface-closure-tests» owner="closure-proved" source="crates/grove/src/cli.rs" lines="140-231" parent="source-command-surface" -->
<!-- insert «tests-module-opening» -->
<!-- insert «undescribed-doc-purpose» -->
<!-- insert «undescribed-doc-twice» -->
<!-- insert «undescribed-doc-empty» -->
<!-- insert «undescribed-arguments» -->
<!-- insert «undescribed-subcommands» -->
<!-- insert «describes-test-doc» -->
<!-- insert «describes-test» -->
<!-- insert «closure-test-doc» -->
<!-- insert «closure-test-subcommands» -->
<!-- insert «closure-test-arguments» -->
<!-- /fragment -->

<a id="the-module"></a>
## A test module inside the binary

The binary has no library target to import from an integration test. Its
private `Cli` is reachable by the nested test module, which imports
`CommandFactory` to obtain clap metadata without running the application.

<!-- fragment «tests-module-opening» owner="closure-proved" source="crates/grove/src/cli.rs" lines="140-143" parent="surface-closure-tests" -->
````rust
#[cfg(test)]
mod tests {
    use super::Cli;
    use clap::CommandFactory;
````
<!-- /fragment -->

<a id="a-closure-property"></a>
## A closed command set

A rejection list can miss a new flag nobody remembered to forbid. This test
collects every subcommand and requires exactly `["config", "view"]`, then checks `["show"]` beneath `config`. It separately
collects the outer arguments, excludes clap help/version, and requires none.
The viewer path belongs to the subcommand and does not become a lifecycle
selector. Adding an unlisted command or bare selector fails this test.

<!-- fragment «closure-test-doc» owner="closure-proved" source="crates/grove/src/cli.rs" lines="198-202" parent="surface-closure-tests" -->
````rust

    /// Stated as a closure property rather than as a list of rejected verbs: the
    /// bare lifecycle has no launch-policy selectors. `view` observes a path;
    /// `config show` explains the configured policy. A new command or argument fails
    /// this closed-set assertion without being named in a rejection list.
````
<!-- /fragment -->

The assertion obtains every subcommand name from the model and compares the
result with the permitted observation commands.

<!-- fragment «closure-test-subcommands» owner="closure-proved" source="crates/grove/src/cli.rs" lines="203-213" parent="surface-closure-tests" -->
````rust
    #[test]
    fn the_human_command_surface_has_nothing_left_to_select() {
        let command = Cli::command();
        let subcommands: Vec<&str> = command.get_subcommands().map(|s| s.get_name()).collect();
        assert!(
            subcommands == ["config", "view"],
            "only observation complements the bare lifecycle: {subcommands:?}"
        );
        let config = command.find_subcommand("config").unwrap();
        let config_commands: Vec<_> = config.get_subcommands().map(|s| s.get_name()).collect();
        assert_eq!(config_commands, ["show"]);
````
<!-- /fragment -->

The second assertion inspects the outer argument identifiers, removes the two
clap metadata options and refuses any remaining lifecycle selector.

<!-- fragment «closure-test-arguments» owner="closure-proved" source="crates/grove/src/cli.rs" lines="214-231" parent="surface-closure-tests" -->
````rust
        let show = config.find_subcommand("show").unwrap();
        let options: Vec<_> = show
            .get_arguments()
            .map(|arg| arg.get_id().as_str())
            .collect();
        assert_eq!(options, ["kind", "json"]);
        let arguments: Vec<String> = command
            .get_arguments()
            .map(|argument| argument.get_id().to_string())
            .filter(|id| id != "help" && id != "version")
            .collect();
        assert!(
            arguments.is_empty(),
            "launch policy has one home and it is not the command line; `grove` \
             accepts: {arguments:?}"
        );
    }
}
````
<!-- /fragment -->

<a id="the-model-not-the-text"></a>
## Inspect the parser model

The test reads argument identifiers and subcommand names rather than scraping
formatted help text. Width and layout changes cannot disguise an extra
selector. Process tests independently check the displayed help and dispatch.

<a id="every-option-described"></a>
## Every listed option has a description

The convention test walks the model and reports each missing description with
its full command path. It protects nested configuration help, the kind filter, the viewer command and WORKTREE argument;
it does not establish whether an existing description is accurate.

<!-- fragment «describes-test-doc» owner="closure-proved" source="crates/grove/src/cli.rs" lines="182-187" parent="surface-closure-tests" -->
````rust

    /// `grove retire --no-launch` shipped with **no doc comment at all** and
    /// rendered as a padded blank row beside two described options
    /// (retire-no-launch-help-k21). Asserted against clap's own model rather than
    /// the rendered text, for the reasons
    /// `crates/grove-llm/tests/help_surfaces.rs` sets out at length.
````
<!-- /fragment -->

The convention test passes the real parser model to the helper, then fails
with every undescribed command path in its diagnostic.

<!-- fragment «describes-test» owner="closure-proved" source="crates/grove/src/cli.rs" lines="188-197" parent="surface-closure-tests" -->
````rust
    #[test]
    fn the_human_facing_binary_describes_every_option_it_lists() {
        let mut out = Vec::new();
        undescribed(&Cli::command(), "grove", &mut out);
        assert!(
            out.is_empty(),
            "these render as blank rows in a generated help surface:\n  {}",
            out.join("\n  ")
        );
    }
````
<!-- /fragment -->

<a id="the-walk"></a>
## Walk arguments and subcommands recursively

The helper first checks each argument, then each subcommand and its children.
It accepts either short or long help, after trimming. Recursion lets the same
check cover deeper command trees without being taught a separate inventory.

<!-- fragment «undescribed-doc-purpose» owner="closure-proved" source="crates/grove/src/cli.rs" lines="144-147" parent="surface-closure-tests" -->
````rust

    /// Collect every `<command path> :: <thing>` in `cmd`'s subtree that appears
    /// in a help listing with no description behind it.
    ///
````
<!-- /fragment -->

The argument loop accepts either help form only when its trimmed text is
nonempty, and records the argument identifier otherwise.

<!-- fragment «undescribed-arguments» owner="closure-proved" source="crates/grove/src/cli.rs" lines="161-170" parent="surface-closure-tests" -->
````rust
    fn undescribed(cmd: &clap::Command, path: &str, out: &mut Vec<String>) {
        for arg in cmd.get_arguments() {
            let described = [arg.get_help(), arg.get_long_help()]
                .into_iter()
                .flatten()
                .any(|help| !help.to_string().trim().is_empty());
            if !described {
                out.push(format!("{path} :: argument `{}`", arg.get_id()));
            }
        }
````
<!-- /fragment -->

The command loop applies the same rule to descriptions and descends into each
subcommand, preserving the full path for actionable diagnostics.

<!-- fragment «undescribed-subcommands» owner="closure-proved" source="crates/grove/src/cli.rs" lines="171-181" parent="surface-closure-tests" -->
````rust
        for sub in cmd.get_subcommands() {
            let described = [sub.get_about(), sub.get_long_about()]
                .into_iter()
                .flatten()
                .any(|about| !about.to_string().trim().is_empty());
            if !described {
                out.push(format!("{path} :: subcommand `{}`", sub.get_name()));
            }
            undescribed(sub, &format!("{path} {}", sub.get_name()), out);
        }
    }
````
<!-- /fragment -->

<a id="twice"></a>
## Why both binaries carry the helper

Each binary owns its own parser. Sharing the short check would require a
separate test crate or a new library surface solely for tests. The comment
records that trade-off; it is not a claim that future edits cannot diverge.

<!-- fragment «undescribed-doc-twice» owner="closure-proved" source="crates/grove/src/cli.rs" lines="148-157" parent="surface-closure-tests" -->
````rust
    /// **This walk exists twice**, here and in
    /// `crates/grove-llm/tests/help_surfaces.rs`, and that is the cost of the two
    /// binaries being two packages: a clap model is reachable only from the
    /// package that declares it, and this one is a binary target with no library
    /// to import from an integration test. The alternative was a shared test
    /// crate for thirty lines, or a `[lib]` on this package that exists only so a
    /// test can reach it — which would give the binary a library to reach into,
    /// and that is the property `docs/specs/module-decomposition.md`'s decision 1
    /// made this a crate to keep.
    ///
````
<!-- /fragment -->

<a id="an-empty-description"></a>
## Whitespace does not describe an option

Clap can retain an explicitly empty help string as metadata. Testing presence
alone would accept a visually blank help row. Trimming and checking for a
nonempty value closes that gap.

<!-- fragment «undescribed-doc-empty» owner="closure-proved" source="crates/grove/src/cli.rs" lines="158-160" parent="surface-closure-tests" -->
````rust
    /// A description counts only if it is non-empty after trimming: clap treats
    /// `#[arg(help = "")]` as present, and an empty string renders exactly like
    /// the missing doc comment this check exists to reject.
````
<!-- /fragment -->

<a id="worked-assertion"></a>
## Worked example: what a new selector breaks

Add a hypothetical `--harness` field to the outer `Cli`: the closure test
collects its identifier and rejects the nonempty argument set. If it also lacks
help, the description test reports `grove :: argument harness`. Adding a
properly described WORKTREE argument to the existing viewer variant satisfies
the description check and leaves the outer argument set unchanged. The
observable view behavior is checked separately by the application fixtures.

<a id="three-mechanisms-complete"></a>
## Three different guarantees

The compiler enforces public library access. The closed-set test constrains
the declared CLI surface. The description test enforces help presence. None
establishes all runtime behavior: the viewer fixtures and terminal smoke
exercise navigation, refresh and cleanup through the actual application.

[Previous: Three steps](03-three-steps.md) | [Contents](README.md) | [Next: What the call reaches](05-what-the-call-reaches.md)
