# How EditorConfig finds the pairs for a file

This page describes version 0.17.2 of the EditorConfig specification. It
explains how a core computes the pairs for one path. First it gives the lookup
and precedence rules in terms of files, sections and pairs. Then it gives the
file format and the glob forms that express those sections.

The terms "pair", "section" and "core" are defined on the
[orientation page](README.md#which-component-does-what). Each section names
the part of the specification that it comes from.

## Lookup and precedence

A core receives the path of one file, and reports the pairs that apply to
that path. Think of each EditorConfig file as a list of sections. Each
section has a glob and a list of pairs. The core does three things:

1. It finds the EditorConfig files that can apply to the path.
2. In each of those files, it finds the sections whose glob matches the path.
3. It reads the pairs of those sections in a fixed order. When two pairs set
   the same key, the pair that it reads later wins.

The rules on this page come from the "File Processing" and "Supported Pairs"
sections of the specification.

### Which files are searched

The core looks for a file named `.editorconfig` in the directory of the given
file. Then it looks in each parent directory in turn, towards the filesystem
root directory. The specification says that `.editorconfig` is the name "by
default". It does not say how another name is chosen.

The search stops at the first of these two events:

- The core finds an EditorConfig file whose preamble sets the `root` key to
  `true`. The core uses this file, and does not search above it.
- The core has searched the filesystem root directory.

The **preamble** is the lines of a file before its first section. The `root`
key has effect only there. Do not confuse the `root` key with the filesystem
root directory. The `root` key marks the top EditorConfig file of a project.
The filesystem root directory is where the search ends when no file sets the
`root` key.

A core treats a directory that does not exist as if it exists and is empty.
Thus the core can compute the pairs for a path whose file or directories do
not exist yet. An example is a new file that the editor has not saved.

### Which sections match

In each file that the search found, the core compares the path with the glob
of each section. A section applies when its glob matches the path. Where a
glob matches depends on the directory of the EditorConfig file that holds it.
The [glob forms](#the-glob-forms) section gives the matching rules.

A pair in the preamble has no effect, with one exception: the `root` key. The
specification says: "With the exception of the `root` key, all pairs MUST be
located under a section to take effect."

### The precedence rule

<a id="precedence-rule"></a>

The core reads the pairs of the matching sections in this order:

1. It reads the files from the one farthest from the path to the one closest
   to it. The closest file is the one in the directory nearest to the file.
2. In each file, it reads the sections from top to bottom.
3. In each section, it reads the pairs from top to bottom.

When the core reads a pair whose key it has already read, the new value
replaces the old value. Thus the last pair that the core reads for a key wins.
A pair in a closer file wins over a pair in a farther file. In one file, a
pair lower in the file wins over a pair higher in the file.

A key is case insensitive, and the core lowercases each key after it parses
the file. Thus `Indent_Style` and `indent_style` are the same key.

The value `unset` removes the effect of a pair, even if an earlier pair set
it. For example, `indent_size = unset` in a closer file cancels an
`indent_size` pair from a farther file, and the editor uses its own default.

The specification does not say whether a core reports a pair whose value is
`unset`, or leaves the key out of its report. It also does not say in which
order a core reports the pairs. A plugin that must work with any core cannot
rely on either behaviour.

This rule comes from the "File Processing" section of the specification:
"Files are read top to bottom and the most recent rules found take
precedence. If multiple EditorConfig files have matching sections, the rules
from the closer EditorConfig file are read last, so pairs in closer files take
precedence." In this quotation, "rules" means pairs. The `unset` value and the case of keys come from the "Supported
Pairs" section.

### Worked example

This example applies the three steps to one path, in a tree of three
EditorConfig files. The path is `/work/project/src/lib/io.c`.

Fragment: the file `/work/.editorconfig`. The other two files of the tree are
omitted.

```editorconfig
[*]
charset = latin1
```

Fragment: the file `/work/project/.editorconfig`. The other two files of the tree are
omitted.

```editorconfig
root = true

[*]
indent_style = space
indent_size = 4
end_of_line = lf

[src/*.c]
insert_final_newline = true
```

Fragment: the file `/work/project/src/.editorconfig`. The other two files of the tree are
omitted.

```editorconfig
[*.{c,h}]
indent_style = tab

[lib/*.c]
indent_size = 2
```

**Step 1: find the files.** The core looks in `/work/project/src/lib/`, which
has no EditorConfig file. Then it finds `/work/project/src/.editorconfig`.
Then it finds `/work/project/.editorconfig`, whose preamble sets `root` to
`true`, so the search stops there. The core never reads
`/work/.editorconfig`.

**Step 2: find the matching sections.**

- In `/work/project/.editorconfig`, the glob `*` has no `/`, so it can match
  at any level below `/work/project/`. It matches `src/lib/io.c`.
- In the same file, the glob `src/*.c` has a `/`, so it is relative to
  `/work/project/`. The `*` does not match a `/`, so the glob matches
  `src/main.c` but not `src/lib/io.c`. This section does not apply.
- In `/work/project/src/.editorconfig`, the glob `*.{c,h}` has no `/`. It
  matches `io.c` at any level below `/work/project/src/`.
- In the same file, the glob `lib/*.c` is relative to `/work/project/src/`.
  It matches `lib/io.c`.

**Step 3: read the pairs in order.** The core reads the farther file first:
`indent_style = space`, `indent_size = 4` and `end_of_line = lf`. Then it
reads the closer file: `indent_style = tab`, then `indent_size = 2`. Each
later pair replaces the earlier pair for the same key.

The pairs that apply to `/work/project/src/lib/io.c` are these:

| Key | Value | Set by |
|---|---|---|
| `indent_style` | `tab` | `[*.{c,h}]` in `/work/project/src/.editorconfig` |
| `indent_size` | `2` | `[lib/*.c]` in `/work/project/src/.editorconfig` |
| `end_of_line` | `lf` | `[*]` in `/work/project/.editorconfig` |

No `charset` pair applies, because the `root` key stopped the search before
`/work/.editorconfig`. No `insert_final_newline` pair applies, because
`[src/*.c]` does not match the path. The table shows the pairs that the rules
give. It is not the output of a core, and the specification does not fix the
order of a core's report.

## The file format

The rules in this section come from the "File Format" section of the
specification. An EditorConfig file must be encoded in UTF-8, with LF or CRLF
line separators.

### Lines

The format is like INI. A core reads a file one line at a time, from the
first line to the last. It removes the whitespace at the start and the end of
each line. Then it treats the line as one of these four types:

| Type | How it is recognised | Effect |
|---|---|---|
| Blank | It holds nothing. | Ignored. |
| Comment | It starts with `;` or `#`. | Ignored. |
| Section header | It starts with `[` and ends with `]`. | Starts a section, whose glob is the text between the brackets. |
| Pair | It holds a key and a value, separated by `=`. | Adds a pair to the current section, or to the preamble. |

A line that is none of these four types is not valid. The specification does
not say what a core does with a line that is not valid.

The specification lists the four types, but it does not give an order in
which to test them. A line such as `[a=b]` fits both the section header and
the pair descriptions. The specification does not say which type it is.

The key of a pair is the text before the first `=`, and the value is the text
after it. The core removes the whitespace at the start and the end of the key
and of the value. Whitespace inside them stays. Thus the line
`foo = a = b` gives the key `foo` the value `a = b`.

The text of a section header can hold any characters between its first `[`
and its last `]`, including `[`, `]`, spaces and tabs. In that text, `/`
separates the parts of a path. A backslash (`\`) does not separate the parts
of a path, even on Windows.

### Parts of a file

An EditorConfig file has these parts:

- The **preamble** is the lines before the first section header. It is
  optional. It can hold pairs, comments and blank lines. Of its pairs, only
  the `root` key has an effect.
- A **section name** is the text between the `[` and the `]` of a section
  header. It is a glob.
- A **section** is the lines from one section header to the next section
  header, or to the end of the file.

### No inline comments

A `;` or a `#` that is not at the start of a line does not start a comment.
It is part of the text of the line. This is where the format differs from
many INI dialects.

Fragment: one section, from the specification's "No inline comments" section.
The preamble and other sections are omitted.

```editorconfig
[*.txt]
foo = editorconfig ;)
```

This fragment gives the key `foo` the value `editorconfig ;)` in `*.txt`
files, not the value `editorconfig`. The specification defines no way to
escape a `;` or a `#`.

The format formerly allowed a `;` or a `#` after the start of a line to start
a comment. The specification notes that old parsers may still allow inline
comments.

### Length limits

A core must accept items up to these lengths:

- a section name of 1024 characters,
- a key of 1024 characters,
- a value of 4096 characters.

Above these lengths, each core can set its own limit or no limit.

### A complete file

Complete example: an EditorConfig file at the top of a project. It uses each
line type and each part of a file.

```editorconfig
# The top EditorConfig file of this project.
root = true

; Defaults for every file.
[*]
charset = utf-8
end_of_line = lf
insert_final_newline = true
trim_trailing_whitespace = true

[*.{py,rs}]
indent_style = space
indent_size = 4

[Makefile]
indent_style = tab

[docs/**.md]
trim_trailing_whitespace = false
spelling_language = en-GB
```

For the file `docs/guide/intro.md` in this project, the sections `[*]` and
`[docs/**.md]` match. The later section replaces `trim_trailing_whitespace`,
so its value is `false`. The pairs are `charset = utf-8`, `end_of_line = lf`,
`insert_final_newline = true`, `trim_trailing_whitespace = false` and
`spelling_language = en-GB`.

## The glob forms

The rules in this section come from the "Glob Expressions" section of the
specification. Each section name is a glob. The globs use the wildcards of a
Unix shell. The specification also says that they are similar to the patterns
of `.gitignore`. It does not say how similar, so this guide does not rely on
that comparison. The [reference page](reference.md#glob-forms) has one entry for each form.

### Where a glob matches

Where a glob matches depends on whether it holds a path separator. A path
separator is a `/` that is not inside square brackets.

- A glob that holds a path separator is relative to the directory of its
  EditorConfig file. For example, `subdir/*.c` matches only the files that end
  in `.c` in `subdir`, in the directory of the EditorConfig file.
- A glob that holds no path separator can match at any level below the
  directory of its EditorConfig file. For example, `*.c` matches each file
  that ends in `.c` in that directory or in any directory below it.

A section name that ends with `/` matches no file.

This is the main difference from a shell glob. A shell matches a glob from
the directory where the shell evaluates it. An EditorConfig glob always
starts from the directory of its file, and a glob with no `/` can match at
any depth.

### The seven forms

| Form | Matches |
|---|---|
| [`*`](reference.md#glob-star) | Any string of characters, except `/`. |
| [`**`](reference.md#glob-double-star) | Any string of characters, including `/`. |
| [`?`](reference.md#glob-question) | Any one character, except `/`. |
| [`[seq]`](reference.md#glob-seq) | Any one character in *seq*. |
| [`[!seq]`](reference.md#glob-not-seq) | Any one character not in *seq*. |
| [`{s1,s2,s3}`](reference.md#glob-alternatives) | Any of the strings, separated by commas. The braces can nest. |
| [`{num1..num2}`](reference.md#glob-range) | Any integer between *num1* and *num2*. Each can be positive or negative. |

Three of these differ from a shell glob. The `**` form matches across `/`.
The `{num1..num2}` form matches numbers. A brace with one string and no comma,
such as `{s1}`, matches only the literal text `{s1}`.

### Escapes

A backslash (`\`) before a character stops a core from treating that
character as special. For example, `\*` matches only a literal `*`.
