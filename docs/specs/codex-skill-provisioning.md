# Codex skill provisioning at Grove startup

## Requested behavior

Bare `grove` ensures its bundled skills are available to Codex before launching
a session. Missing skills are installed and Grove-managed installations are
refreshed when the installed binary carries different bytes. A provisioning
failure prevents launch and identifies the failed path and corrective action.

Claude Code retains its marketplace installation. Homebrew caveats must show
the plugin installation commands and explicitly instruct users to enable
marketplace auto-update. Grove does not write Claude's skill directories or
plugin configuration.

The default scope is every bundled skill eligible for Codex under its existing
`harnesses:` frontmatter: Grove, Linkuistics, and Testanyware. Claude-only skills
remain excluded. Custom, independently authored session-kind skills remain
the configuration author's responsibility.

## Delivery choice

Embed a snapshot of the repository's plugin files in the human-facing `grove`
binary. Build-time discovery produces the file inventory and Codex-eligible
skill list, watches directories and files for Cargo rebuilds, rejects duplicate
skill-directory names, and preserves executable flags on supporting scripts.
The files under `plugins/` remain the authoritative sources.

This makes source builds, extracted release archives, and Homebrew installations
use the same offline provisioning path. It adds no download or repository
discovery at startup, and no new dependency is required.

Alternatives considered:

- Downloading from GitHub at startup introduces network and authentication
  failures before a local session can begin, and requires selecting a version.
- Shipping a separate plugin directory alongside the executable avoids
  embedding but requires distinct source-build, archive, and Homebrew path
  resolution and makes a moved binary incomplete.

## Runtime ownership

The binary owns a provisioning adapter, called only by bare lifecycle startup
after workspace resolution and driver-lease acquisition, before the loop can
mutate the task tree or launch its first session. Help, version, configuration
inspection/examples, viewing, agent verbs, and standalone invocations retain
their existing behavior.

An existing `$HOME/.codex` directory or explicitly configured `CODEX_HOME`
identifies a Codex installation. Claude-only installations are left alone.
Provisioning never infers the harness from a configured executable or wrapper.

Install discoverable skill links in `$HOME/.agents/skills`, the documented Codex
user skill location. Store complete plugin snapshots and a provisioning lock
under `$HOME/.agents/.grove` in a Grove-owned subdirectory, shared by all
Codex homes using that user skill directory. Include supporting
plugin files so skill references into their plugin's own resources still work.

Provisioning holds the common installation lock across inspection and
publication. It verifies the active snapshot's file inventory, bytes, and
executable flags. An unchanged snapshot and correct links require no writes.
For a new or damaged snapshot, materialize a complete staging directory before
publishing it through an atomic replacement of the managed current-snapshot
symlink. Previously published snapshots are retained so existing readers can
finish using resolved paths.

Only missing destinations and recognizably Grove-managed symlinks may be
created or replaced. Recognizable legacy links into this repository's bundled
plugin layout may be adopted; replacing a link never writes its former target.
Existing Grove-managed links in the legacy `$CODEX_HOME/skills` location are
kept working against the same snapshot. Do not create a second legacy
installation on a fresh machine. Remove obsolete links only when Grove owns
them. Leave unrelated skills, arbitrary symlinks, and real directories alone;
report a name collision as an actionable failure rather than overwriting it.

Provisioning errors must not be downgraded to launch-time warnings. Failures
after partial installation report failure and prevent the child from starting;
the next invocation can repair the managed state. Correct existing
installations are quiet; successful changes receive a short stderr report.

## Documentation and Homebrew

Update the formula template's caveats to distinguish automatic Codex delivery
from Claude marketplace setup. Keep all three Claude plugin commands and the
instruction to enable marketplace auto-update. Manual `plugins/install.sh`
remains available for developer checkouts and the other supported harnesses.

Reconcile README, plugin installation documentation, usage/configuration and
architecture descriptions with this behavior. Retain historical descriptions
as history, but remove current claims that the binary never provisions skills.
Update the source-exact system-overview walkthrough for the new adapter,
generated inventory reference, module declaration, and startup call.

## Verification

Use temporary homes and real files/processes; never test installation against
the developer's skill directories. Establish a failing startup test before
adding the adapter, then cover:

- first installation, complete supporting resources, and eligibility filtering;
- a repeat startup leaving correct files and links untouched;
- repair of a missing skill link and damaged managed snapshot;
- replacement of an older managed snapshot and removal of obsolete owned links;
- existing developer links and working legacy links;
- refusal of foreign files/directories/symlinks without modifying them;
- provisioning failure preventing a fake configured child from launching;
- concurrent startup in separate workspaces, including different Codex homes
  sharing one HOME;
- custom Codex homes and a machine with no Codex installation;
- help and inspection commands having no provisioning side effects.

Verify the formula caveats and every affected walkthrough. Run
`bash scripts/check.sh` on the integrated result, outside Codex's sandbox when
the existing macOS confinement tests require it. Preserve a linear local
`main` history when integrating the completed change; publication is a
separate release action.

## Source references

- Existing manual installer: `plugins/install.sh`.
- Lifecycle dispatch: `crates/grove/src/cli.rs`.
- Release caveats: `scripts/templates/grove.rb.tmpl`.
- [Codex local skill locations and symlink support](https://developers.openai.com/codex/skills/#where-to-save-skills),
  consulted 2026-09-21.
