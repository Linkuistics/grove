First, drain the inbox: read `.grove-inboxes/inboxes/{{NAME}}.md` and triage each entry — incorporate into the current task, defer to a later leaf, or reject (and possibly seed elsewhere with `grove inbox add <other-name>`). When done, run `grove inbox drain {{NAME}}` to clear and commit the file. Never edit the inbox file or its branch directly.

Then do the next task in `.grove/`. Follow grove's loop: pick → bootstrap → execute → commit → judge retirement.
