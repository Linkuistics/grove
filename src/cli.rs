use clap::Parser;

/// The complete human command surface: bare `grove`, plus clap's own `--help`
/// and `--version`. There are no subcommands and no flags — the driver reads
/// the task tree for what to do and `~/.config/grove/config.kdl` for how to
/// launch it, so there is nothing left for an argument to select.
#[derive(Parser)]
#[command(
    name = "grove",
    version,
    about = "Grove: hierarchical workstream tool for AI agents"
)]
pub struct Cli {}

pub fn run() -> anyhow::Result<()> {
    let _cli = Cli::parse();
    crate::launch::bare_grove()
}
