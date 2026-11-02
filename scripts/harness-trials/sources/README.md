# Trial sources

Each directory here is a plugin directory that a harness trial can name as a
skill source. A trial reads it from the plain Git checkout of the preparation,
at the recorded commit.

`rival-records` holds a second `decision-records` skill. Its name equals the
name of the skill in `plugins/linkuistics`, and its convention differs: it
writes numbered records under `docs/decisions/`. The case `problem-ambiguity`
supplies both sources, so that two different files match one activity. No
plugin and no installation uses this directory.
