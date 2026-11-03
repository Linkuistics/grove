# Seccomp source of the QEMU floor route

`default.json` is the unchanged Apache-2.0 Moby default allowlist at
[4b8214fc4971e06172938a1f1554ccff6647ee80](https://github.com/moby/profiles/blob/4b8214fc4971e06172938a1f1554ccff6647ee80/seccomp/default.json).
SHA-256: `6416b47770785a41ac59073cdc77d9fe98517df2799dc83ef207e622de3053f6`.
The upstream licence is beside it.

`../floor_seccomp.py` verifies these bytes before it derives the test profile
of the helper container. No network fetch occurs at execution. The profile is
an explicitly pinned baseline. It does not reproduce the built-in profile of
the installed Docker Engine. See [the QEMU floor route](../../docs/linux-floor-qemu.md)
for the additions and the security of the helper.
