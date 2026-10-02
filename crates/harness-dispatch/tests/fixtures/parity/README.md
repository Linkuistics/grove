# Parity fixture

`commands.json` is what Grove's configuration resolver produced from the
owner's pinned `config.kdl` (6063 bytes, SHA-256
`2294bd271586ae1723bdda4b6e6977a324666264ac7149be9560a78a3de22647`), which is
not kept here. It is the record the shipped sample policy is compared against,
argument for argument.

Each entry is one selection the sample offers: an arrangement, alone and with
each combination of the two modifiers. `select` is the profile list the
resolver was given, and `commands` holds the program and arguments it resolved
for every kind the file routes. An argument spelled `${name}` is a runtime
value the resolver left symbolic: the prompt, the session name or the
main-repository root. The two modifiers resolve to the same commands in either
order.

Grove 21.13.0's `grove config show --json` produced it, with nothing
transcribed by hand. The pinned file was `~/.config/grove/config.kdl` under a
temporary `HOME`, and each entry is one run in a scratch jj workspace whose
ignored `.grove.kdl` held only that entry's `select` line. `jq` projected each
command's `words` to `program` and `args`, writing a slot word as `${name}`.
The file is a record of that resolver and is not regenerated or edited.
