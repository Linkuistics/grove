Start a new grove called {{NAME}} in this worktree.

First, drain the inbox: read `.grove-inboxes/inboxes/{{NAME}}.md` and triage each entry — incorporate, defer to a leaf in the new tree, or reject (and possibly seed elsewhere with `grove inbox add <other-name>`). When done, run `grove inbox drain {{NAME}}` to clear and commit the file. Never edit the inbox file or its branch directly.

Then use grove's start-a-new-grove flow: grill me on the goal (use the bundled grilling procedure), sharpening new terminology into `CONTEXT.md` inline. Propose the root `.grove/BRIEF.md` (per `BRIEF-FORMAT.md`) and a small initial decomposition — one or two leaves only. Don't over-plan.
