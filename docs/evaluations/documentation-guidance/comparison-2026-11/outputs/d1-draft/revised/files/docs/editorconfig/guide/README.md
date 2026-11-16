# A guide to the EditorConfig specification

This guide describes version 0.17.2 of the EditorConfig specification. It is
for a developer who is about to write an EditorConfig plugin for a text editor.
After reading it, you can find the entry for any key or glob form. You can
explain which EditorConfig files apply to a path, and which pair wins. You can
also predict the pairs that a core reports for a path in a small directory
tree.

The guide assumes that you know INI-style files, shell glob notation and the
plugin interface of your editor. It does not teach them.

## What EditorConfig decides for a file

EditorConfig gives each file a set of editing settings, such as the indent
style, the line ending and the character set. Each setting is a **pair**: a
key and a value, such as `indent_style` and `space`. The specification also
calls a pair a "key-value pair" or a "rule". This guide says "pair" every
time. It uses "rule" only for a rule of the specification, such as the
precedence rule.

The project authors write the pairs in files named `.editorconfig`. The files
are in the project's directories. One file can hold pairs for many files, and
many `.editorconfig` files can apply to one file. Thus the central question
for each file that the editor opens is this:

> Which `.editorconfig` files apply to this path, which of their pairs match
> it, and which pair wins when two pairs set the same key?

The answer is the list of pairs for that one path. The
[lookup and precedence](explanation.md#lookup-and-precedence) section of the
explanation page gives the rules that compute it.

EditorConfig applies wherever an editor or a tool applies settings per file.
The specification compares its file format to INI and its globs to
`.gitignore`. These comparisons help you read the notation. They do not limit
where EditorConfig applies.

## Which component does what

The specification names four roles.

| Role | What it does | Who supplies it |
|---|---|---|
| EditorConfig file | Holds sections, and each section holds pairs. | The project's authors write it. |
| Core | Receives a file's path, finds and parses the EditorConfig files, and reports the pairs that apply to that path. | A core library or program. |
| Plugin | Receives the pairs from a core and changes the editor's settings for the file. | You, the plugin developer. |
| Editor | Lets a person edit files, and uses a plugin to set its settings for each file. | The editor's own developers. |

A **section** is one part of an EditorConfig file. It starts with a glob in
square brackets, such as `[*.py]`, and holds the pairs that apply to the paths
that the glob matches. A **core** is the component that answers the central
question above. Both words have everyday meanings, but in this guide they
mean only the EditorConfig concepts.

Some plugins include their own core, and other plugins use an external core.
Some editors include the plugin or the core themselves. Different people can
supply each component, and one program can fill more than one role. Either way, the work divides like this:

- The core applies the lookup and precedence rules. It reports every pair
  that is syntactically valid, including pairs whose key the specification
  does not define.
- The plugin applies the pairs. It ignores keys that it does not recognise,
  and values that are not valid or that it does not support.

If your plugin includes its own core, your plugin does both jobs. The
[explanation page](explanation.md) gives the rules that a core applies. The
[reference page](reference.md) gives one entry for each key that a plugin
applies.

## What this guide covers

The guide covers the nine keys that the specification defines, and its seven
glob forms. It does not cover how to write a core, the specification's test
suites, or the plugin interface of a specific editor.

The guide has three pages. Read them in order to learn the model:

1. This page: what EditorConfig decides and which component does what.
2. [How EditorConfig finds the pairs for a file](explanation.md): lookup and
   precedence, then the file format, then the glob forms.
3. [Reference](reference.md): one entry for each key and each glob form.

Each entry on the reference page, and the
[precedence rule](explanation.md#precedence-rule), can also be read alone.

The specification uses "must" and "shall" for requirements, and "should" for
recommendations. Where this guide gives a requirement, it says "must". Where
the specification only recommends, this guide says so and quotes the
specification's word.
