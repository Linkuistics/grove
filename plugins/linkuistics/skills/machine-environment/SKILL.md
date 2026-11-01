---
name: machine-environment
description: Antony's macOS development and configuration contract — mise tool selection, Taskfile workflows, mise bootstrap ownership of machine setup, and the Obsidian operating manual. Use when setting up projects, running development commands, adding toolchains, or changing persistent configuration on this machine.
harnesses: [any]
assumes-personal-setup: true
---

# Machine environment

This is Antony's personal setup. On another machine, follow its owner's project
and environment configuration. The paths below identify the maintained local
contract; they are not directories to recreate on unrelated machines.

## Find the maintained setup

The operating manual starts at
`$HOME/Library/Mobile Documents/iCloud~md~obsidian/Documents/Brain/Configuration/Machine Configuration.md`.
Read it when working on persistent machine configuration or recovery. Its linked
Development Environment, Maintenance and Recovery, Application Recovery, and
Machine Status notes provide the relevant detail.

`$HOME/.config/mise/config.toml` declares the machine setup. It holds the global
tool defaults, the Homebrew and App Store packages, the macOS defaults, and the
dotfiles that mise tracks. Read its current contents and the project's
requirements. Do not copy a version from this skill.

## Run project tools through mise

Read the repository's instructions, Taskfile, manifests, lockfiles, and existing
version files before setup or checks. Project pins take precedence over machine
defaults. Preserve existing idiomatic files such as `rust-toolchain.toml`,
`.python-version`, `.zig-version`, and `.tool-versions`.

A noninteractive agent shell may not load the interactive mise hook. After
reviewing configuration, use `mise install` for required tools and run the
repository's existing task inside its environment:

```sh
mise exec -- task check
```

Use the repository's actual task name and entry point. The Taskfile owns reusable
recipes; mise supplies tool selection. Keep project libraries with their language
package manager and lockfile. Verify selected versions when diagnosing a mismatch.
Use `mise use <tool>@<version>` for an intentional project requirement change,
then preserve that change in the project repository.

This applies across languages. Retain a documented native manager or SDK when
mise cannot provide the required tool; language servers, coupled runtimes, and
native libraries need compatible versions too. Follow an explicit project or
user-selected environment instead of replacing it with the personal default.

## Preserve persistent changes

Project requirements and tasks belong in the project. Machine packages belong in
`[bootstrap.packages]` of the global mise config. Selected shell, editor, and
application settings are dotfiles that mise tracks in place. Keep credentials and
application data with their private recovery owners.

- Edit a tracked file where it is. The history watcher saves each edit.
- Run `mise dot status` to see the tracked state.
- Track a new file with `mise dot track <path>`.
- Add a package with `mise bootstrap packages use <manager:package>`.
- Preview a change with `mise bootstrap plan`.
- Do not run `mise bootstrap packages prune`. It uninstalls undeclared packages.
- Leave `mise dot sync` to the owner. It publishes the history.

After a machine change, update the affected Obsidian note and run `mac-check`.
It is read-only: exit 10 includes manual review, especially CCC backup verification
with the separate disk connected. It does not authorize installations or establish
that a backup is current. `mac-bootstrap` is an explicit, broader setup action.

The local manual and repository instructions govern exceptions. Mise command
behaviour is documented in [getting started](https://mise.jdx.dev/getting-started)
and [configuration](https://mise.jdx.dev/configuration).
