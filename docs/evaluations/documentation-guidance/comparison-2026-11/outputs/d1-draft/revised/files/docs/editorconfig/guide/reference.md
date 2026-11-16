# Reference

This page describes version 0.17.2 of the EditorConfig specification. It has
one entry for each of the nine keys and each of the seven glob forms that the
specification defines. Each entry can be read alone.

The text of each entry restates a rule of the specification, and its source
field names the section of the specification that holds the rule. A note or an
example is set apart in a quotation block and opens with its label. It is not
a rule.

## Keys

A **key** is the left side of a pair, such as `indent_style` in
`indent_style = tab`. A core reports the pairs that apply to a file, and a
plugin applies them to the editor's settings for that file. The
[precedence rule](explanation.md#precedence-rule) decides which pair wins when
two pairs set the same key.

These rules apply to every key. They come from the "Supported Pairs" section
of the specification.

- A key is case insensitive. A core lowercases each key after it parses the
  file.
- The value `unset` removes the effect of a pair, even if an earlier pair set
  it. The editor then uses its own default.
- A pair takes effect only in a section. The `root` key is the only
  exception.
- A core must report each pair that is syntactically valid, even if the
  specification does not define its key.
- A plugin must ignore a key that it does not recognise, and a value that is
  not valid or that it does not support.

Each key entry has these fields: values, case of values, default, effect, and
source. Where the specification states nothing for a field, the entry says
so.

<a id="key-indent_style"></a>

### `indent_style`

- **Values:** `tab` or `space`.
- **Case of values:** case insensitive.
- **Default:** the specification states none.
- **Effect:** `tab` indents with hard tabs, and `space` indents with soft
  tabs.
- **Source:** "Supported Pairs".

> **Fragment:** one section. The preamble and other sections are omitted.
>
> ```editorconfig
> [*.go]
> indent_style = tab
> ```

<a id="key-indent_size"></a>

### `indent_size`

- **Values:** a whole number, or `tab`.
- **Case of values:** case insensitive.
- **Default:** the specification states none.
- **Effect:** a whole number sets the number of columns for each indentation
  level. It also sets the width of a soft tab, where the editor supports it.
  The value `tab` sets the indentation size to the tab size. The
  specification recommends (its word is "should") that the tab size is the
  value of [`tab_width`](#key-tab_width), if a pair sets it. Otherwise the tab
  size is the one that the editor sets.
- **Source:** "Supported Pairs".

> **Fragment:** one section. The preamble and other sections are omitted.
>
> ```editorconfig
> [*.py]
> indent_style = space
> indent_size = 4
> ```

<a id="key-tab_width"></a>

### `tab_width`

- **Values:** a whole number.
- **Case of values:** the specification does not say.
- **Default:** the value of [`indent_size`](#key-indent_size).
- **Effect:** sets the number of columns that a tab character fills. The
  specification says that this key "should not usually need to be
  specified".
- **Source:** "Supported Pairs".

> **Note:** If `indent_size` is `tab` and no pair sets `tab_width`, each key
> refers to the other. The `indent_size` entry resolves this case: the tab
> size is then the one that the editor sets.

> **Fragment:** one section. The preamble and other sections are omitted.
>
> ```editorconfig
> [Makefile]
> indent_style = tab
> tab_width = 8
> ```

<a id="key-end_of_line"></a>

### `end_of_line`

- **Values:** `lf`, `cr` or `crlf`.
- **Case of values:** case insensitive.
- **Default:** the specification states none.
- **Effect:** sets the characters that end each line.
- **Source:** "Supported Pairs".

> **Fragment:** one section. The preamble and other sections are omitted.
>
> ```editorconfig
> [*.bat]
> end_of_line = crlf
> ```

<a id="key-charset"></a>

### `charset`

- **Values:** `latin1`, `utf-8`, `utf-8-bom`, `utf-16be` or `utf-16le`.
- **Case of values:** the specification does not say.
- **Default:** the specification states none.
- **Effect:** sets the character set of the file. The specification
  discourages `utf-8-bom`.
- **Source:** "Supported Pairs".

> **Note:** `charset` sets the encoding of the file that the editor edits. An
> EditorConfig file itself must always be in UTF-8.

> **Fragment:** one section. The preamble and other sections are omitted.
>
> ```editorconfig
> [*]
> charset = utf-8
> ```

<a id="key-spelling_language"></a>

### `spelling_language`

- **Values:** `ss` or `ss-TT`. Here `ss` is a two-letter language code from
  [ISO 639](https://en.wikipedia.org/wiki/ISO_639), and `TT` is a two-letter
  territory identifier from
  [ISO 3166](https://en.wikipedia.org/wiki/ISO_3166). Thus a value must have
  two or five characters. A value names one language only.
- **Case of values:** the specification does not say.
- **Default:** none. The specification states that there is no default.
- **Effect:** sets the natural language that the editor uses (the
  specification's word is "should") to check spelling. It does not set the
  character set. [`charset`](#key-charset) sets the character set.
- **Source:** "Supported Pairs".

> **Fragment:** one section. The preamble and other sections are omitted.
>
> ```editorconfig
> [*.md]
> spelling_language = en-GB
> ```

<a id="key-trim_trailing_whitespace"></a>

### `trim_trailing_whitespace`

- **Values:** `true` or `false`.
- **Case of values:** the specification does not say.
- **Default:** the specification states none.
- **Effect:** `true` removes all whitespace characters before each newline
  character in the file. `false` makes sure that the editor does not remove
  them.
- **Source:** "Supported Pairs".

> **Fragment:** one section. The preamble and other sections are omitted.
>
> ```editorconfig
> [*.md]
> trim_trailing_whitespace = false
> ```

<a id="key-insert_final_newline"></a>

### `insert_final_newline`

- **Values:** `true` or `false`.
- **Case of values:** the specification does not say.
- **Default:** the specification states none.
- **Effect:** `true` makes sure that the file ends with a newline when the
  editor saves it. `false` makes sure that it does not.
- **Source:** "Supported Pairs".

> **Fragment:** one section. The preamble and other sections are omitted.
>
> ```editorconfig
> [*]
> insert_final_newline = true
> ```

<a id="key-root"></a>

### `root`

- **Values:** `true`. The specification names no other value.
- **Case of values:** case insensitive.
- **Default:** the specification states none.
- **Effect:** the pair must be in the preamble of the file. The value `true`
  stops the search for EditorConfig files at this file. The core uses this
  file and searches no directory above it. This is the only pair that takes
  effect outside a section.
- **Source:** "Supported Pairs" and "File Processing". The search is
  described under
  [Which files are searched](explanation.md#which-files-are-searched).

> **Note:** The `root` key is not the filesystem root directory. The search
> also stops at the filesystem root directory when no file sets `root` to
> `true`.

> **Fragment:** the preamble of a file. The sections are omitted.
>
> ```editorconfig
> root = true
> ```

## Glob forms

A section name is a glob. A core compares it with the path of a file, and the
section applies when the glob matches. A glob that holds a `/` outside square
brackets is relative to the directory of its EditorConfig file. A glob with no
such `/` can match at any level below that directory. The rules are under
[Where a glob matches](explanation.md#where-a-glob-matches).

These rules apply to every glob form. They come from the "Glob Expressions"
section of the specification.

- A backslash (`\`) before a character stops a core from treating that
  character as special.
- A section name that ends with `/` matches no file.
- A core must accept a section name of up to 1024 characters.

Each glob form entry has these fields: matches, path separators, and source.
In each example, the paths are relative to the directory of the EditorConfig
file.

<a id="glob-star"></a>

### `*`

- **Matches:** any string of characters.
- **Path separators:** does not match `/`.
- **Source:** "Glob Expressions".

> **Fragment:** one section header. The pairs of the section are omitted.
>
> ```editorconfig
> [*.md]
> ```
>
> This glob has no `/`, so it matches `README.md` and `docs/guide.md`.

<a id="glob-double-star"></a>

### `**`

- **Matches:** any string of characters.
- **Path separators:** matches `/`.
- **Source:** "Glob Expressions".

> **Fragment:** one section header. The pairs of the section are omitted.
>
> ```editorconfig
> [lib/**.js]
> ```
>
> This glob matches `lib/a.js` and `lib/x/y/a.js`. With `*` in place of
> `**`, it would match `lib/a.js` but not `lib/x/y/a.js`.

<a id="glob-question"></a>

### `?`

- **Matches:** any one character.
- **Path separators:** does not match `/`.
- **Source:** "Glob Expressions".

> **Fragment:** one section header. The pairs of the section are omitted.
>
> ```editorconfig
> [file?.txt]
> ```
>
> This glob matches `file1.txt` and `fileA.txt`, but not `file10.txt`.

<a id="glob-seq"></a>

### `[seq]`

- **Matches:** any one character in *seq*.
- **Path separators:** a `/` inside the brackets does not make the glob
  relative to the directory of its file. The specification does not say
  whether the form matches a `/` in the path.
- **Source:** "Glob Expressions".

> **Note:** The specification does not define any syntax inside *seq*, such
> as a range of characters.

> **Fragment:** one section header. The pairs of the section are omitted.
>
> ```editorconfig
> [*.[ch]]
> ```
>
> This glob matches `main.c` and `main.h`, but not `main.o`.

<a id="glob-not-seq"></a>

### `[!seq]`

- **Matches:** any one character that is not in *seq*.
- **Path separators:** a `/` inside the brackets does not make the glob
  relative to the directory of its file. The specification does not say
  whether the form matches a `/` in the path.
- **Source:** "Glob Expressions".

> **Fragment:** one section header. The pairs of the section are omitted.
>
> ```editorconfig
> [*.[!o]]
> ```
>
> This glob matches `main.c`, but not `main.o`.

<a id="glob-alternatives"></a>

### `{s1,s2,s3}`

- **Matches:** any one of the strings, which commas separate. A string can
  hold another brace form, because the braces can nest. A brace with one
  string and no comma, such as `{s1}`, matches only the literal text `{s1}`.
- **Path separators:** the specification does not say.
- **Source:** "Glob Expressions".

> **Fragment:** one section header. The pairs of the section are omitted.
>
> ```editorconfig
> [*.{js,ts}]
> ```
>
> This glob matches `app.js` and `app.ts`, but not `app.css`.

<a id="glob-range"></a>

### `{num1..num2}`

- **Matches:** any integer between *num1* and *num2*. Each of *num1* and
  *num2* can be positive or negative.
- **Path separators:** does not apply, because the form matches integers.
- **Source:** "Glob Expressions".

> **Note:** The specification does not say whether *num1* and *num2*
> themselves match. It also does not say how it treats leading zeros or a
> *num1* greater than *num2*.

> **Fragment:** one section header. The pairs of the section are omitted.
>
> ```editorconfig
> [log{1..3}.txt]
> ```
>
> This glob matches `log2.txt`, but not `log4.txt`.
