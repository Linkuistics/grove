/* Test infrastructure: enter the mount root, then irreversibly drop authority. */
#define _GNU_SOURCE
#include <grp.h>
#include <linux/capability.h>
#include <stdio.h>
#include <sys/mount.h>
#include <sys/prctl.h>
#include <sys/syscall.h>
#include <unistd.h>

int main(int argc, char **argv) {
  if (argc < 3) {
    fputs("usage: floor-qemu-enter ROOT ABSOLUTE-PROGRAM [ARG...]\n", stderr);
    return 2;
  }
  /* A chroot cannot create unprivileged user namespaces. Preserve submounts,
     pivot, and detach the old root: https://man7.org/linux/man-pages/man2/pivot_root.2.html */
  if (mount(NULL, "/", NULL, MS_REC | MS_PRIVATE, NULL) ||
      mount(argv[1], argv[1], NULL, MS_BIND | MS_REC, NULL) || chdir(argv[1]) ||
      syscall(SYS_pivot_root, ".", ".") || umount2(".", MNT_DETACH) || chdir("/")) {
    perror("floor-qemu-enter: pivot root");
    return 3;
  }
  for (int cap = 0; cap <= CAP_LAST_CAP; cap++) {
    if (prctl(PR_CAPBSET_DROP, cap, 0, 0, 0)) {
      perror("floor-qemu-enter: drop bounding capability");
      return 4;
    }
  }
  if (setgroups(0, NULL) || setgid(1000) || setuid(1000) ||
      prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0)) {
    perror("floor-qemu-enter: drop identity/privilege");
    return 5;
  }
  execv(argv[2], argv + 2);
  perror("floor-qemu-enter: exec target");
  return 6;
}
