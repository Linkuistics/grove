# configure-codex-jj-permissions-k11

## Goal

Configure a scoped Codex permission profile so Grove's unattended Codex
sessions can commit through jj without receiving unrestricted machine access.

## Context

The human requested this `impl` leaf be inserted as the next Grove step because
they are operating through an iPad screen-sharing app. The `impl` session kind
is mapped to Claude in `/Users/antony/.config/grove/config.kdl`; Claude can edit
the two personal configuration files that the current Codex `workspace-write`
sandbox cannot.

Edit these files:

- `/Users/antony/.codex/config.toml`
- `/Users/antony/.config/grove/config.kdl`

Add this inactive-by-default profile to the Codex config:

```toml
[permissions.grove-jj]
description = "Allow Grove to write its worktree and shared jj/Git repository."

[permissions.grove-jj.filesystem]
":root" = "read"
":tmpdir" = "write"
":slash_tmp" = "write"

[permissions.grove-jj.filesystem.":workspace_roots"]
"." = "write"

[permissions.grove-jj.network]
enabled = false
```

In all five `review-*` Codex templates and `research-b`, replace:

```text
--sandbox workspace-write --ask-for-approval never
```

with:

```text
-c default_permissions=grove-jj --ask-for-approval never
```

Keep `--profile sol-xhigh`, `--add-dir ${repo}`, and `${prompt}`. Removing
`--sandbox` is load-bearing: according to the official OpenAI permission-profile
documentation, passing `--sandbox` selects the legacy sandbox settings instead
of `default_permissions`.

## Done when

- `permissions.grove-jj` exists exactly once and is not the global default.
- The profile grants read-only access broadly, write access to temp and every
  runtime workspace root, and no shell network access.
- Exactly the five `review-*` Codex templates plus `research-b` select
  `default_permissions=grove-jj`; none of those six still passes `--sandbox`.
- `codex --strict-config --version` exits successfully.
- The relevant `config.kdl` lines are re-read and confirm that `impl` still
  launches Claude while the six intended entries launch Codex.

## Notes

- Official reference: <https://learn.chatgpt.com/docs/permissions>
- Do not set `default_permissions` globally; Grove selects this profile only in
  its Codex command templates.
- `codex-jj-sandbox-permission-k10` is the later Codex-run verification step.
