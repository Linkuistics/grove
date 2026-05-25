# grove

Hierarchical, self-extending workstream tool for AI agents. See `content/SKILL.md` for the methodology; this README covers the CLI.

## Install

```
brew tap Linkuistics/taps
brew install grove
```

## Use

```
grove install [<repo>]              # materialise grove into <repo>
grove update  [<repo>]              # refresh grove in <repo>
grove start <name>                  # bootstrap a new grove
grove continue <name>               # resume an existing grove
grove status                        # show grove state in cwd
grove --help                        # full surface
```

See `grove --help` for the complete command surface.
