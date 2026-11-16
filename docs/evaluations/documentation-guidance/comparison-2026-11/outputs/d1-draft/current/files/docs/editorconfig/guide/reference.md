# EditorConfig reference

This page describes version 0.17.2 of the EditorConfig specification. It has one entry for each key and one entry for each glob form. For how the pairs for a file are found and which pair wins, read the [explanation](explanation.md).

## Keys

The specification defines nine keys. A core also reports keys that the specification does not define. A plugin ignores keys that it does not recognise, and values that are not valid or not supported.

These rules apply to every key:

- Keys are not case sensitive. A core changes all keys to lower case after it reads them.
- The value `unset` removes the effect of the key, even if an earlier pair set it. The editor then uses its own default.
- A pair has an effect only in a section. The one exception is `root`, which has an effect only in the preamble.

| Key | Values |
|---|---|
| [`indent_style`](#indent_style) | `tab`, `space` |
| [`indent_size`](#indent_size) | A whole number, or `tab` |
| [`tab_width`](#tab_width) | A whole number |
| [`end_of_line`](#end_of_line) | `lf`, `cr`, `crlf` |
| [`charset`](#charset) | `latin1`, `utf-8`, `utf-8-bom`, `utf-16be`, `utf-16le` |
| [`spelling_language`](#spelling_language) | `ss` or `ss-TT` |
| [`trim_trailing_whitespace`](#trim_trailing_whitespace) | `true`, `false` |
| [`insert_final_newline`](#insert_final_newline) | `true`, `false` |
| [`root`](#root) | `true` |

### `indent_style`

Sets the type of indentation.

- `tab`: use hard tabs.
- `space`: use soft tabs.

The values are not case sensitive.

```ini
indent_style = space
```

### `indent_size`

Sets the number of columns for each level of indentation. It also sets the width of soft tabs, where the editor supports this.

- A whole number: the number of columns.
- `tab`: use the tab size. The tab size should be the value of [`tab_width`](#tab_width), if the file sets it. If not, the tab size is the editor's tab size.

The values are not case sensitive.

```ini
indent_size = 4
```

### `tab_width`

Sets the number of columns that show one tab character. The value is a whole number.

The default is the value of [`indent_size`](#indent_size). Usually a file does not need to set `tab_width`.

```ini
tab_width = 8
```

### `end_of_line`

Sets how line breaks are written.

- `lf`
- `cr`
- `crlf`

The values are not case sensitive.

```ini
end_of_line = lf
```

### `charset`

Sets the character set of the file.

- `latin1`
- `utf-8`
- `utf-8-bom`
- `utf-16be`
- `utf-16le`

The specification discourages `utf-8-bom`.

```ini
charset = utf-8
```

### `spelling_language`

Sets the natural language for spell checking. The value is one language only. There is no default.

The format is `ss` or `ss-TT`:

- `ss` is a two-letter language code from [ISO 639](https://en.wikipedia.org/wiki/ISO_639).
- `TT` is a two-letter territory code from [ISO 3166](https://en.wikipedia.org/wiki/ISO_3166).

Thus, the value is two characters or five characters long.

This key does not set the character set. The [`charset`](#charset) key sets it.

```ini
spelling_language = en-GB
```

### `trim_trailing_whitespace`

- `true`: remove all whitespace characters before each newline in the file.
- `false`: make sure that the editor does not remove them.

```ini
trim_trailing_whitespace = true
```

### `insert_final_newline`

- `true`: make sure that the file ends with a newline when it is saved.
- `false`: make sure that the file does not end with a newline.

```ini
insert_final_newline = true
```

### `root`

Stops the search for EditorConfig files. Set it to `true` in the file at the top of a project. The core reads that file, but it does not look in the parent directories.

The key must be in the preamble, before the first section header. The value is not case sensitive.

The `root` key is not the root directory of the filesystem. The search also stops at the filesystem's root directory, when no file sets `root = true`. The explanation describes [which files are searched](explanation.md#which-files-are-searched).

```ini
root = true
```

## Glob forms

A section name is a glob. These rules apply to every glob form:

- A glob that contains a `/` matches relative to the directory of its EditorConfig file. A `/` inside square brackets does not count.
- A glob without a `/` can also match at any level below that directory.
- A section name that ends with `/` matches no file.
- A backslash (`\`) before a character makes it an ordinary character.

The explanation describes [where a glob matches](explanation.md#where-a-glob-matches).

<a id="glob-star"></a>

### `*` (star)

Matches any string of characters, except `/`.

```ini
[*.py]
```

This matches every file name that ends in `.py`. Because the glob has no `/`, it also matches in subdirectories.

<a id="glob-double-star"></a>

### `**` (double star)

Matches any string of characters, including `/`.

```ini
[docs/**.md]
```

This matches every file that ends in `.md` in `docs` and in all directories below `docs`.

<a id="glob-question-mark"></a>

### `?` (question mark)

Matches any one character, except `/`.

```ini
[file?.txt]
```

This matches `file1.txt` and `fileA.txt`. It does not match `file10.txt`.

<a id="glob-seq"></a>

### `[seq]` (character set)

Matches any one character in *seq*.

```ini
[*.[ch]]
```

This matches file names that end in `.c` or `.h`.

<a id="glob-not-seq"></a>

### `[!seq]` (negated character set)

Matches any one character that is not in *seq*.

```ini
[*.[!o]]
```

This matches file names that end in `.` and one character other than `o`.

<a id="glob-strings"></a>

### `{s1,s2,s3}` (string list)

Matches any one of the strings in the list. Commas separate the strings. A list can contain other lists.

```ini
[*.{js,ts}]
```

This matches file names that end in `.js` or `.ts`.

A list with only one string is not a list. `{s1}` matches only the literal text `{s1}`.

<a id="glob-numbers"></a>

### `{num1..num2}` (number range)

Matches any integer between *num1* and *num2*. Each of *num1* and *num2* can be positive or negative.

```ini
[log{1..3}.txt]
```

This matches `log2.txt`. The specification does not say whether the range includes *num1* and *num2*.
