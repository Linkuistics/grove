# How EditorConfig finds the pairs for a file

This page describes version 0.17.2 of the EditorConfig specification. It explains how a core finds the pairs that apply to one file, and which pair wins when two pairs set the same key. Then it describes the file format and the glob forms that express those pairs.

Each key and each glob form also has an entry in the [reference](reference.md).

## Lookup and precedence

A core starts with the path of one file. It finds the EditorConfig files that can apply to that path. Then it reads the sections in those files whose glob matches the path. The result is one set of pairs.

### Which files are searched

The core looks for an EditorConfig file in the directory of the given file. Then it looks in each parent directory, one level at a time. The default file name is `.editorconfig`.

The search stops at the first of these two conditions:

- The core finds an EditorConfig file that sets the `root` key to `true` in its preamble. The core reads this file, but it does not look in its parent directories.
- The core reaches the root directory of the filesystem.

The `root` key and the filesystem's root directory are different things. The `root` key stops the search at a project's top directory. The filesystem's root directory is the end of the search when no file sets the `root` key.

A directory that does not exist is not an error. The core treats it as a directory that exists and holds no EditorConfig file. Thus, a core can look up the pairs for a path before the file or its directory exists.

### Which sections apply

In each EditorConfig file that the search finds, the core looks for sections whose glob matches the given path. A glob matches relative to the directory of the EditorConfig file that holds it:

- A glob that contains a `/` matches only from that directory. For example, `[subdir/*.c]` matches `.c` files in `subdir`, directly below the EditorConfig file.
- A glob that has no `/` matches at any level below that directory. For example, `[*.c]` matches `.c` files in that directory and in all its subdirectories.

A `/` inside square brackets does not count for this rule. The [glob forms](#the-glob-forms) section gives the full rule.

### Which pair wins

<!-- The precedence rule is a direct entry route from README.md. Keep this heading's text stable. -->

When two pairs set the same key, the pair that the core reads last wins. The core reads in this order:

1. EditorConfig files from the farthest directory to the closest. The file in the same directory as the given file is read last.
2. In each file, the lines from top to bottom.

Thus, a pair in a closer file overrides a pair in a farther file. In one file, a pair in a later matching section overrides a pair in an earlier matching section.

The value `unset` removes the effect of a pair, even if an earlier pair set it. For example, `indent_size = unset` removes `indent_size`, and the editor uses its own default. Because `unset` is also a pair, the same precedence rule applies to it.

Pairs in the preamble have no effect, with one exception: the `root` key. The preamble is the lines before the first section. The [file format](#the-file-format) section describes it.

### Example: one path in a small tree

This example has two EditorConfig files:

```text
/work/project/.editorconfig
/work/project/src/.editorconfig
```

The file `/work/project/.editorconfig` contains:

```ini
# Top of the project. Do not search the parent directories.
root = true

[*]
indent_style = space
indent_size = 4
end_of_line = lf

[*.md]
trim_trailing_whitespace = false
```

The file `/work/project/src/.editorconfig` contains:

```ini
[*.py]
indent_size = 2

[lib/*.py]
indent_style = tab
```

A core receives the path `/work/project/src/lib/util.py`. It searches these directories in this order:

1. `/work/project/src/lib`: no EditorConfig file.
2. `/work/project/src`: an EditorConfig file. It does not set `root`, so the search continues.
3. `/work/project`: an EditorConfig file. It sets `root = true` in its preamble, so the search stops here. The core does not look in `/work`.

The core reads the farthest file first. In `/work/project/.editorconfig`:

- `[*]` matches, so `indent_style = space`, `indent_size = 4` and `end_of_line = lf` are read.
- `[*.md]` does not match.

Then, in `/work/project/src/.editorconfig`:

- `[*.py]` matches, because a glob without `/` matches at any level below. `indent_size = 2` overrides `indent_size = 4`.
- `[lib/*.py]` matches, because `lib/util.py` is relative to `/work/project/src`. `indent_style = tab` overrides `indent_style = space`.

The pairs for `/work/project/src/lib/util.py` are:

```text
indent_style = tab
indent_size = 2
end_of_line = lf
```

`root = true` is not in this list. It is in the preamble, outside every section, so no section gives it to the path. It only stops the search.

## The file format

An EditorConfig file is in a format like INI. It must be encoded in UTF-8. Lines are separated by LF or by CRLF.

### Lines

A core reads the file one line at a time, from start to end. It removes the whitespace at the start and at the end of each line. Then the line is one of these types:

| Type | Form | Effect |
|---|---|---|
| Blank | Nothing. | Ignored. |
| Comment | Starts with `;` or `#`. | Ignored. |
| Section header | Starts with `[` and ends with `]`. | Starts a section. The text between the brackets is a glob. |
| Pair | A key and a value, separated by `=`. | Sets the key to the value. |

A line that is not one of these types is not valid.

### Section headers

A section name is the text between the first `[` and the last `]` of a section header. It can contain any characters, including `[`, `]`, spaces and tabs.

In a section name, `/` separates the parts of a path. A backslash (`\`) is not a path separator, even on Windows.

```ini
[src/*.js]
```

### Pairs

The key is the text before the first `=`. The value is the text after the first `=`. The core removes whitespace at the start and end of the key and of the value. Whitespace in the middle stays.

```ini
indent_style = space
```

Keys are not case sensitive. The core changes all keys to lower case after it reads them. Whether a value is case sensitive depends on its key. Each key's entry in the [reference](reference.md#keys) says.

A core reports every pair that is valid by syntax, also when the specification does not define the key. A plugin ignores keys that it does not recognise. It also ignores values that are not valid or not supported.

### No comments after text

A `;` or `#` starts a comment only at the start of a line. Anywhere else, it is part of the line's text. For example:

```ini
[*.txt]
foo = editorconfig ;)
```

This gives `foo` the value `editorconfig ;)` in `*.txt` files. It does not give the value `editorconfig`.

The specification has no escape for `;` or `#`. Some old parsers still read comments after text, so a file that relies on this rule can give different results in those parsers.

### Parts of a file

An EditorConfig file has these parts:

- **Preamble**: the lines before the first section header. The preamble is optional. It can contain pairs, comments and blank lines. Only the `root` key has an effect in the preamble.
- **Section**: a section header and the lines after it, up to the next section header or the end of the file.

The first file in the [example](#example-one-path-in-a-small-tree) is a complete EditorConfig file with a preamble and two sections.

### Length limits

A core must accept:

- section names up to and including 1024 characters,
- keys up to and including 1024 characters,
- values up to and including 4096 characters.

Above these lengths, each core can set its own limit or no limit.

## The glob forms

Each section name is a glob that matches file paths. The notation is like the shell's wildcards and like `.gitignore`. These are the special forms:

| Form | Matches |
|---|---|
| [`*`](reference.md#glob-star) | Any string of characters, except `/`. |
| [`**`](reference.md#glob-double-star) | Any string of characters, including `/`. |
| [`?`](reference.md#glob-question-mark) | Any one character, except `/`. |
| [`[seq]`](reference.md#glob-seq) | Any one character in *seq*. |
| [`[!seq]`](reference.md#glob-not-seq) | Any one character not in *seq*. |
| [`{s1,s2,s3}`](reference.md#glob-strings) | Any one of the strings, separated by commas. |
| [`{num1..num2}`](reference.md#glob-numbers) | Any integer between *num1* and *num2*. |

### Where a glob matches

A glob that contains a `/` is relative to the directory of its EditorConfig file. A `/` inside square brackets does not count. A glob without a `/` can also match at any level below that directory.

For example, `[*.c]` matches any file that ends in `.c` in the directory of the EditorConfig file or below it. `[subdir/*.c]` matches only files that end in `.c` in `subdir`, directly below the EditorConfig file.

Because of this rule, a section name that ends with `/` matches no file.

### Escapes

A backslash (`\`) before a character makes it an ordinary character. For example, `[\*.txt]` matches files named `*.txt`, and no other files.
