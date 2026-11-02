---
name: coding-style-bash
description: Bash script house style — a fixed shebang, long-form shell options, a main function, readonly constants and a usage header. Use when writing or refactoring Bash scripts.
---

# Bash script house style

## Header

- **Shebang:** `#!/bin/bash`. Do not use `env` to find the interpreter.
- **Usage header:** the second line is `# Usage:`, followed by one comment
  line for each argument.
- **Shell options:** `set -o errexit -o nounset -o pipefail`, in this long
  form. Do not use the short form `set -euo pipefail`.

## Layout

- Declare each constant with `readonly`, in upper case, after the options.
- Put all logic in functions. The last line of the script is `main "$@"`.
- Indent with four spaces.

## Style

- Quote every variable expansion.
- Use `printf` instead of `echo`.
- Write errors to stderr, and exit with a non-zero code.
