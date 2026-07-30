//! Every generated help surface in both binaries describes everything it lists.
//!
//! `grove retire --no-launch` shipped with **no doc comment at all** and rendered
//! as a padded blank row beside two described options
//! (retire-no-launch-help-k21). It was found by a human executing every help
//! surface and reading them (retire-help-node-path-k20), which is the only reason
//! it was found at all: a missing doc comment compiles, and the defect exists
//! solely on a surface no test looked at.
//!
//! Asserted against **clap's own model** rather than the rendered text. The scan
//! that found the defect was done by eye, and the obvious way to automate it —
//! scraping `--help` output for a token followed by nothing — has to reproduce
//! clap's two layouts (a short row, and the long-help layout a multi-paragraph
//! description switches the whole command into) and its wrapping. That parser is
//! more likely to be wrong than the thing it checks, and wrong in the direction
//! that manufactures a false clean. Walking `Command` asks the same question
//! where it is a fact rather than a rendering.
//!
//! Subcommands are covered alongside arguments because a subcommand with no
//! `about` renders the identical blank row one surface up. The property is *this
//! help lists nothing it does not describe*; restricting it to arguments would
//! leave it passing on the same defect wearing a different hat.

use clap::CommandFactory;

/// Collect every `<command path> :: <thing>` in `cmd`'s subtree that appears in
/// a help listing with no description behind it.
///
/// A description counts only if it is non-empty after trimming: clap treats
/// `#[arg(help = "")]` as present, and an empty string renders exactly like the
/// missing doc comment this file exists to reject.
fn undescribed(cmd: &clap::Command, path: &str, out: &mut Vec<String>) {
    for arg in cmd.get_arguments() {
        let described = [arg.get_help(), arg.get_long_help()]
            .into_iter()
            .flatten()
            .any(|h| !h.to_string().trim().is_empty());
        if !described {
            out.push(format!("{path} :: argument `{}`", arg.get_id()));
        }
    }
    for sub in cmd.get_subcommands() {
        let described = [sub.get_about(), sub.get_long_about()]
            .into_iter()
            .flatten()
            .any(|a| !a.to_string().trim().is_empty());
        if !described {
            out.push(format!("{path} :: subcommand `{}`", sub.get_name()));
        }
        undescribed(sub, &format!("{path} {}", sub.get_name()), out);
    }
}

fn assert_fully_described<C: CommandFactory>(binary: &str) {
    let cmd = C::command();
    let mut out = Vec::new();
    undescribed(&cmd, binary, &mut out);
    assert!(
        out.is_empty(),
        "these render as blank rows in a generated help surface:\n  {}",
        out.join("\n  ")
    );
}

#[test]
fn the_human_facing_binary_describes_every_option_it_lists() {
    assert_fully_described::<grove::cli::Cli>("grove");
}

#[test]
fn the_llm_facing_binary_describes_every_option_it_lists() {
    // Held to the same bar despite `cli-binary-split` marking it not-for-humans:
    // its own long_about says a session reads `grove-llm --help` to recover the
    // verb set after dropping context, so an undescribed row here costs a
    // *session* its bearings rather than a human a second look.
    assert_fully_described::<grove::llm_cli::Cli>("grove-llm");
}
