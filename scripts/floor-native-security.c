/* Test-only outer-container security controls, compiled for the glibc floor. */
#define _GNU_SOURCE
#include <errno.h>
#include <sched.h>
#include <stdio.h>
#include <sys/mount.h>
#include <sys/prctl.h>
#include <sys/syscall.h>
#include <unistd.h>

int main(void) {
  if (getuid() != 1000 || getgid() != 1000 ||
      prctl(PR_GET_NO_NEW_PRIVS, 0, 0, 0, 0) != 1 ||
      prctl(PR_GET_SECCOMP, 0, 0, 0, 0) != 2) return 1;
  /* No inherited bounding capabilities, including SYS_ADMIN and SETUID. */
  for (int cap = 0; cap <= 40; cap++)
    if (prctl(PR_CAPBSET_READ, cap, 0, 0, 0) != 0) return 2;
  errno = 0;
  if (mount("/", "/", NULL, MS_BIND, NULL) != -1 || errno != EPERM) return 3;
  errno = 0;
  if (unshare(CLONE_NEWNET) != -1 || errno != EPERM) return 4;
  errno = 0;
  if (syscall(SYS_keyctl, 0, 0, 0, 0, 0) != -1 || errno != EPERM) return 5;
  puts("floor-native-security: uid/gid, no-new-privileges, seccomp filter, empty capability bounding set, outer mount/network namespace/keyring denials passed");
  return 0;
}
