# EditorConfig for plugin authors

This guide describes version 0.17.2 of the EditorConfig specification. It is for a developer who will write an EditorConfig plugin for a text editor. The specification's own text is in [`../source/index.rst`](../source/index.rst).

## What EditorConfig decides for a file

EditorConfig gives one file path a set of key-value pairs. Each pair is a setting for that file, such as `indent_style = space` or `end_of_line = lf`. A plugin applies these settings in the editor while a person edits the file.

The pairs come from files named `.editorconfig`. These files can be in the directory of the edited file and in each parent directory. Each EditorConfig file holds sections. Each section has a glob, and the pairs in a section apply to the paths that its glob matches.

Thus, for one path, the result is a set of pairs. The specification decides three things about that result:

- which EditorConfig files are read for the path,
- which sections in those files apply to the path,
- which pair wins when two pairs set the same key.

The specification calls a key-value pair a "pair" and sometimes a "rule". This guide uses "pair" only.

## Which component does what

The specification names four roles. A single program can fill more than one role.

| Component | What it does |
|---|---|
| EditorConfig file | Holds sections of pairs. People write these files. The usual name is `.editorconfig`. |
| Core | Reads the EditorConfig files for a path, and gives the resulting pairs to a plugin. |
| Plugin | Receives the pairs from a core, and changes the editor's settings to match them. |
| Editor | Edits files, and uses a plugin to set the settings for each file. |

A plugin can include its own core, or it can use an external core. An editor can include the plugin or core functions itself. Different people can write each of these components.

The division of work between core and plugin matters to a plugin author:

- A core reports every pair that is valid by syntax, including pairs whose key the specification does not define.
- A plugin ignores keys that it does not recognise, and values that are not valid or not supported.

How to write a core is not in this guide. Neither are the conformance test suites, or the plugin interface of any one editor.

## How to read this guide

To learn EditorConfig, read the three pages in order:

1. This page, for orientation.
2. [Explanation](explanation.md): how a core finds the pairs for a path and which pair wins. Then the file format and the glob forms.
3. [Reference](reference.md): one entry for each key and one entry for each glob form.

To look up one item, go directly to:

- the [entry for a key](reference.md#keys),
- the [entry for a glob form](reference.md#glob-forms),
- the [precedence rule](explanation.md#which-pair-wins).
