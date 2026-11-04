# ignore-rules-k1

## Goal

Survey how the `git` and `jj` command-line tools on this machine describe the
files that they ignore: where each tool reads its ignore rules, and what each
tool says about a file that is already tracked.

## Done when

- The survey document exists and holds one section for each tool.
- Each claim cites the exact help command whose output supports it, such as
  `git help gitignore` or `jj help config`, with the quoted words. The
  installed help text is the only primary source. Do not search the web.
- One commit holds the survey.

## Notes

Delegate the survey of `jj` to one delegate, through your harness's agent
tool. Survey `git` yourself. Wait for the result of the delegate, then put the
two sections into the one document before you commit.
