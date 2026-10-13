# Native floor seccomp source

`default.json` is the unchanged Apache-2.0 Moby default allowlist at
[4b8214fc4971e06172938a1f1554ccff6647ee80](https://github.com/moby/profiles/blob/4b8214fc4971e06172938a1f1554ccff6647ee80/seccomp/default.json).
SHA-256: `6416b47770785a41ac59073cdc77d9fe98517df2799dc83ef207e622de3053f6`.
The upstream license is alongside it. `../floor_seccomp.py` verifies these bytes
before deriving the native test profile; no network fetch occurs at execution.
This is an explicitly pinned baseline, not a claim to reproduce the installed
Docker Engine's built-in profile. See `docs/linux-floor-native.md` for the
additions, supported environment and actual denial/refusal controls.
