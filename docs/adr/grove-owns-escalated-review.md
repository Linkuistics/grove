# Grove owns escalated review

Once the current session has run Grove's Bootstrap, invoked `grove-llm pick`
itself, and adopted the returned leaf, doubt-driven development may spend at
most one in-session fresh-context reviewer across that leaf. Each independently
materialised reviewer counts — a diverse-lens pass with N subagents spends N —
and a second review need, including re-review after a substantive,
non-mechanical fix, is represented as a Grove review chain. A producer that
already has a scheduled Grove review invokes no duplicate doubt reviewer;
review leaves and research-pair leaves invoke none.

The ownership predicate is the session's own Bootstrap-and-pick procedure, not
checkout state or an inherited environment variable. Outside a picked Grove
leaf, doubt-driven development keeps its standalone bounded-cycle behavior.
This division binds because letting both orchestrators own the same review
multiplies reviews and bypasses Grove's fresh-session routing, while disabling
in-session doubt entirely makes a narrow unexpected check pay for three Grove
sessions.

## Considered options

- **Keep doubt's multi-cycle loop inside Grove.** Rejected because the child
  reviewers do not pass through Grove's task-kind routing and a substantive fix
  can recursively create more hidden review work. Reopen only if in-session
  reviewers become Grove-visible, Grove-routed work without adding state outside
  the task tree.
- **Disable doubt whenever a checkout contains `.grove/`.** Rejected because
  checkout presence does not mean the current session is executing a picked
  leaf, and a plain producer still benefits from one cheap, narrow disproof
  attempt. Reopen only if every non-trivial producer decision receives an
  equivalent in-flight check without forcing a review chain for small work.
