# trial — brief

## Goal

A small command-line tool, `wordtally`, that counts the words in the Markdown
files of one directory.

## Done when

- `wordtally <dir>` prints one line for each `.md` file directly in `<dir>`:
  the file name, a tab and the word count.
- A word is a run of characters between whitespace. Text inside fenced code
  blocks does not count.
- The output is sorted by file name.
- A directory with no Markdown file prints nothing and exits 0.
- The tool is fast.

## Notes

This grove is disposable. It exists for one harness trial, and its leaves are
small, so that each session measures the loading route rather than the work.
