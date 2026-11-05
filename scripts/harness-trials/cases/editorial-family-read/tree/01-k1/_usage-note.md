# usage-note-k1 — brief

## Goal

A usage note of one section for `wordtally`. A reader who has never run the
tool can count the words of a directory after reading it.

## Prose contract

- British spelling, with the -ise forms.
- One term for one concept. The count of one file is its "word count". Do not
  write "word total" or "number of words".
- A sentence has 25 words at most.
- No semicolons.
- No figure of speech. Name the mechanism instead.
- The project has no glossary, and the contract names no precedent.

## The document

The draft stage wrote this text. Later stages change it here, in place.

```markdown
# Using wordtally

Run `wordtally <dir>` to recognize every Markdown file directly in `<dir>`;
the tool prints one line for each file. Each line holds the file name, a tab
and the word total. The tool is lightning fast, so a large directory is no
problem. Text inside fenced code blocks does not add to the number of words.
A directory with no Markdown file prints nothing and exits 0.
```
