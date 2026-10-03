/*
 * A fake harness that reports the process exec made it, and then execs the
 * step its arguments name, which keeps every part of that process:
 *
 *     session-probe [--alter] RECORDS NEXT [ARG...]
 *
 * RECORDS and NEXT are absolute, since `--alter` changes directory.
 *
 * It is C for the reason harness-dispatch's signal probe is: C's runtime
 * changes none of what it reports before main, while a shell rewrites its own
 * dispositions as it starts and a Rust program's runtime ignores SIGPIPE
 * before its main runs. The launch suite compiles it with the host's C
 * compiler.
 *
 * It takes the first free numbered directory RECORDS/<n>, writes `process`
 * there, and execs NEXT with SESSION_RECORD naming that directory, so that the
 * step can leave its own evidence beside the report. Exec keeps the PID, so
 * the step's exit or signal death is the harness's own.
 *
 *     pid 4242
 *     pgid 4242
 *     foreground 4242        the controlling terminal's foreground group, or -1
 *     stdin /dev/ttys007     the terminal on stdin, or -
 *     cwd /private/var/...
 *     ignored 30
 *     blocked 31
 *     pending
 *     caught
 *
 * `--alter` is the positive controls' fixture. The probe forks, and the child
 * leaves the process group, swaps its stdin for /dev/null, changes directory
 * to /, flips SIGPIPE between ignored and default and blocks SIGALRM, all
 * before it reports, so no line above says what the harness was handed. The
 * parent waits, and ends as the child ended.
 */

#include <errno.h>
#include <fcntl.h>
#include <limits.h>
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <sys/wait.h>
#include <unistd.h>

/* Every signal number either platform uses; the kernel refuses a query of one
 * it does not have. */
#define LAST_SIGNAL 64

_Noreturn static void fail(const char *what, int code) {
    perror(what);
    exit(code);
}

/* Wait for the altered child, then end as it did. */
_Noreturn static void mirror(pid_t child) {
    int status;
    while (waitpid(child, &status, 0) == -1) {
        if (errno != EINTR) {
            fail("session-probe: waiting for the altered child", 95);
        }
    }
    if (WIFSIGNALED(status)) {
        int number = WTERMSIG(status);
        sigset_t just;
        sigemptyset(&just);
        sigaddset(&just, number);
        signal(number, SIG_DFL);
        sigprocmask(SIG_UNBLOCK, &just, NULL);
        raise(number);
    }
    exit(WIFEXITED(status) ? WEXITSTATUS(status) : 96);
}

static void alter(void) {
    pid_t child = fork();
    if (child == -1) {
        fail("session-probe: fork", 97);
    }
    if (child > 0) {
        mirror(child);
    }
    int null = open("/dev/null", O_RDONLY);
    if (null == -1 || setpgid(0, 0) != 0 || dup2(null, STDIN_FILENO) == -1 || chdir("/") != 0) {
        fail("session-probe: altering", 98);
    }
    close(null);
    struct sigaction pipe;
    sigaction(SIGPIPE, NULL, &pipe);
    signal(SIGPIPE, pipe.sa_handler == SIG_IGN ? SIG_DFL : SIG_IGN);
    sigset_t alarm;
    sigemptyset(&alarm);
    sigaddset(&alarm, SIGALRM);
    sigprocmask(SIG_BLOCK, &alarm, NULL);
}

static void members(FILE *out, const char *name, const sigset_t *set) {
    fputs(name, out);
    for (int number = 1; number <= LAST_SIGNAL; number++) {
        if (sigismember(set, number) == 1) {
            fprintf(out, " %d", number);
        }
    }
    fputs("\n", out);
}

static void handlers(FILE *out, const char *name, int ignored) {
    fputs(name, out);
    for (int number = 1; number <= LAST_SIGNAL; number++) {
        struct sigaction action;
        if (sigaction(number, NULL, &action) != 0) {
            continue;
        }
        int is_ignored = action.sa_handler == SIG_IGN;
        int is_caught = !is_ignored && action.sa_handler != SIG_DFL;
        if (ignored ? is_ignored : is_caught) {
            fprintf(out, " %d", number);
        }
    }
    fputs("\n", out);
}

int main(int argc, char **argv) {
    int first = 1;
    if (argc > 1 && strcmp(argv[1], "--alter") == 0) {
        alter();
        first = 2;
    }
    if (argc - first < 2) {
        fputs("usage: session-probe [--alter] RECORDS NEXT [ARG...]\n", stderr);
        return 2;
    }

    sigset_t blocked, pending;
    if (sigprocmask(SIG_BLOCK, NULL, &blocked) != 0 || sigpending(&pending) != 0) {
        fail("session-probe: reading the mask", 91);
    }
    pid_t foreground = -1;
    int terminal = open("/dev/tty", O_RDONLY);
    if (terminal != -1) {
        foreground = tcgetpgrp(terminal);
        close(terminal);
    }
    const char *input = isatty(STDIN_FILENO) ? ttyname(STDIN_FILENO) : NULL;
    char cwd[PATH_MAX];
    if (getcwd(cwd, sizeof cwd) == NULL) {
        fail("session-probe: reading the cwd", 92);
    }

    char record[PATH_MAX];
    for (int n = 0;; n++) {
        if (n == 10) {
            fputs("session-probe: no free record directory\n", stderr);
            return 90;
        }
        snprintf(record, sizeof record, "%s/%d", argv[first], n);
        if (mkdir(record, 0700) == 0) {
            break;
        }
        if (errno != EEXIST) {
            fail("session-probe: creating the record", 90);
        }
    }
    char path[PATH_MAX + 16];
    snprintf(path, sizeof path, "%s/process", record);
    FILE *out = fopen(path, "w");
    if (out == NULL) {
        fail("session-probe: opening the report", 93);
    }
    fprintf(out, "pid %d\nppid %d\npgid %d\nforeground %d\nstdin %s\ncwd %s\n", (int)getpid(),
            (int)getppid(), (int)getpgrp(), (int)foreground, input == NULL ? "-" : input, cwd);
    handlers(out, "ignored", 1);
    members(out, "blocked", &blocked);
    members(out, "pending", &pending);
    handlers(out, "caught", 0);
    if (fclose(out) != 0) {
        fail("session-probe: writing the report", 94);
    }

    if (setenv("SESSION_RECORD", record, 1) != 0) {
        fail("session-probe: exporting the record", 99);
    }
    execv(argv[first + 1], argv + first + 1);
    fail("session-probe: exec", 99);
}
