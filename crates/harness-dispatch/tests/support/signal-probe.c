/*
 * A fake harness that reports the signal state exec delivered to it: which
 * signals are ignored, blocked, pending and caught, read first thing in main.
 *
 * It is C because C's runtime changes none of that before main. A shell
 * rewrites its own dispositions as it starts, and a Rust program's runtime
 * ignores SIGPIPE before its main runs, which is the very change the front
 * works around. The command seam compiles it with the host's C compiler.
 *
 * Like the shell fake harness, it creates $FAKE_HARNESS_RECORD exclusively, so
 * its presence is the marker that the harness ran, and writes its report to
 * `signals` inside it, one line per set:
 *
 *     ignored 1 13
 *     blocked 10
 *     pending
 *     caught
 *
 * `--alter` is the positive control's fixture: the probe first flips its own
 * SIGPIPE between ignored and default and blocks SIGUSR2, then execs itself
 * without the flag, so what it reports is not what its caller handed it.
 */

#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <unistd.h>

/* Every signal number either platform uses; the kernel refuses a query of one
 * it does not have. */
#define LAST_SIGNAL 64

static void alter(char **argv) {
    struct sigaction pipe;
    sigaction(SIGPIPE, NULL, &pipe);
    signal(SIGPIPE, pipe.sa_handler == SIG_IGN ? SIG_DFL : SIG_IGN);
    sigset_t usr2;
    sigemptyset(&usr2);
    sigaddset(&usr2, SIGUSR2);
    sigprocmask(SIG_BLOCK, &usr2, NULL);
    argv[1] = argv[0];
    execvp(argv[0], argv + 1);
    perror("signal-probe: re-exec");
    exit(93);
}

int main(int argc, char **argv) {
    sigset_t blocked, pending;
    if (sigprocmask(SIG_BLOCK, NULL, &blocked) != 0 || sigpending(&pending) != 0) {
        perror("signal-probe: reading the mask");
        return 91;
    }
    if (argc > 1 && strcmp(argv[1], "--alter") == 0) {
        alter(argv);
    }

    const char *record = getenv("FAKE_HARNESS_RECORD");
    if (record == NULL || mkdir(record, 0700) != 0) {
        perror("signal-probe: creating the record");
        return 90;
    }
    char path[4096];
    snprintf(path, sizeof path, "%s/signals", record);
    FILE *out = fopen(path, "w");
    if (out == NULL) {
        perror("signal-probe: opening the report");
        return 92;
    }

    fputs("ignored", out);
    for (int signal = 1; signal <= LAST_SIGNAL; signal++) {
        struct sigaction action;
        if (sigaction(signal, NULL, &action) == 0 && action.sa_handler == SIG_IGN) {
            fprintf(out, " %d", signal);
        }
    }
    fputs("\nblocked", out);
    for (int signal = 1; signal <= LAST_SIGNAL; signal++) {
        if (sigismember(&blocked, signal) == 1) {
            fprintf(out, " %d", signal);
        }
    }
    fputs("\npending", out);
    for (int signal = 1; signal <= LAST_SIGNAL; signal++) {
        if (sigismember(&pending, signal) == 1) {
            fprintf(out, " %d", signal);
        }
    }
    fputs("\ncaught", out);
    for (int signal = 1; signal <= LAST_SIGNAL; signal++) {
        struct sigaction action;
        if (sigaction(signal, NULL, &action) == 0 && action.sa_handler != SIG_IGN
            && action.sa_handler != SIG_DFL) {
            fprintf(out, " %d", signal);
        }
    }
    fputs("\n", out);
    return fclose(out) == 0 ? 0 : 94;
}
