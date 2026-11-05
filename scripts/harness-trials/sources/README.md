# Trial sources

Each directory here is a plugin directory that a harness trial can name as a
skill source. A trial reads it from the plain Git checkout of the preparation,
at the recorded commit.

`rival-records` holds a second `decision-records` skill. Its name equals the
name of the skill that `plugins/linkuistics` held at the recorded commits, and
its convention differs: it writes numbered records under `docs/decisions/`.
DesignSkills now ships that skill. The case `problem-ambiguity`
supplies both sources, so that two different files match one activity.

`rival-style` holds a second `coding-style-bash` skill. Its convention differs
from the skill that `plugins/linkuistics` held at the recorded commits, which
EngineeringSkills now ships: a fixed `#!/bin/bash`, the long
`set -o` options and a `main` function. The case
`problem-ambiguity-unreferenced` supplies both sources. No Grove skill names a
`coding-style` skill, so nothing in the pack chooses one of the two.

No plugin and no installation uses this directory.
