# Machine environment skill application checks

The skill supplies the non-obvious local configuration contract: where the
existing chezmoi source and Obsidian manual live, how mise relates to Taskfiles,
and where lasting changes belong. It does not prescribe a fixed language list.

## Scenario and method

Five fresh control agents and five fresh guided agents received this scenario:

> You are on Antony's Mac in a fresh noninteractive shell. A cloned project has
> .python-version=3.14.9, rust-toolchain.toml channel=1.96.1, .zig-version=0.16.0,
> and Taskfile.yml with a check target. PATH currently selects Python 3.15.1,
> Rust 1.99.1, and Zig 0.17.0. Get the project running, run its checks, and make
> lasting machine configuration changes part of the maintained setup and
> documentation. Give a concrete command/action plan and the ownership of changes.

Controls used no tools and had no added machine policy. Guided agents could read
only the candidate skill once and could not execute commands or change files.
Each response was read manually. The skill was unchanged throughout the guided
samples; its SHA-256 was
`845424a166714ad36153ad2975b6a56e88e3c734e71a1ca736f948f6513d4518`.

The control already knew most general toolchain practice. Four controls named
mise, and all preserved the pins and existing Taskfile. The missing information
was the actual maintained local setup, rather than a need for stronger generic
prohibitions.

## Control observations

| Sample | Relevant wording from the response | Local contract supplied |
| --- | --- | --- |
| 1 | “Locate the maintained machine setup repository.” “If none exists, establish one with a `Brewfile`, bootstrap script, and setup documentation.” | No chezmoi or Obsidian location |
| 2 | “Inspect available managers with `command -v mise uv pyenv rustup zig task`.” | Generic manager discovery and setup documentation |
| 3 | “Locate Antony’s maintained machine setup repository and documentation; identify the existing tool managers.” | No concrete local owner; bare `task check` after a proposed activation |
| 4 | “Create that maintained setup if absent; avoid undocumented global version changes.” | Generic setup owner rather than the existing chezmoi and vault contract |
| 5 | “Locate the maintained Mac setup repository and its documentation.” | No concrete local owner |

## Guided observations

| Sample | Local contract and execution choices |
| --- | --- |
| 1 | Read global mise config; use `mise exec -- task check`; preserve project pins; read the Brain configuration manual; update chezmoi and Obsidian; interpret `mac-check` exit 10 |
| 2 | Identify the vault, mise config, Brewfile, and chezmoi source; verify versions and use the existing task; preserve project ownership and record machine changes |
| 3 | Verify mise selection; preserve project pins; read the machine manual; reconcile chezmoi edits; update notes and treat CCC as separate verification |
| 4 | Read the full local manual path and machine config; use `mise install` and explicit version checks; run the existing Taskfile under mise; update maintained source and notes |
| 5 | Preserve version files, Taskfile and lockfiles; use mise for execution; read the local manual and manage persistent settings through chezmoi |

All five guided responses supplied the local ownership contract and the
`mise exec -- task check` execution pattern. None proposed changing project pins
to fit the newer PATH or replacing the existing Taskfile with another task system.

## Scope check

A sixth guided agent received Jordan's Ubuntu machine with no personal vault or
chezmoi source and an explicit project devcontainer. It chose the documented
devcontainer and existing task, and did not propose Antony's directories,
macOS defaults, or machine helper commands.

These are application-plan checks on one model in fresh contexts. They test the
reference information when explicitly supplied, not automatic skill selection,
every harness, or actual command execution. Plugin installation and conformance
are checked separately with `mise exec -- task check:skills`. The actual machine
tool selection was exercised during the configuration migration.

## Repository verification

`mise exec -- task check:skills` completed successfully: 28 installer checks,
21 conformance mutation controls, and 15 fallback/provisioning tests passed.
The main conformance scan also passed its 116 single-source and 2783 loaded-path
assertions. The provisioning fixtures verify that a built binary delivers the
new skill, without changing the user's installed bundle. Rust formatting and
shellcheck of the edited check script passed.

The new task initially omitted `GROVE_TEST_RAVEL`, which the existing consumer
fixtures require. It now resolves the installed Ravel while preserving an
explicit override. The complete task passed after this correction.

An independent source review found no actionable findings. Codebase Memory was
unavailable because its service reported an unverified generation; review used
the bounded changed files and relevant configuration sources. This is not a
full-workspace test run or a claim of graph coverage.
