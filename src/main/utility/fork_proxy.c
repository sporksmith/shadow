#include "main/utility/fork_proxy.h"

#include <errno.h>
#include <semaphore.h>
#include <unistd.h>
#include <sys/prctl.h>

#include "main/core/logger/shadow_logger.h"
#include "support/logger/logger.h"

struct _ForkProxy {
    // To be called in parent after fork.
    void (*parent_cb)(pid_t);
    // To be called in child between fork and exec.
    void (*child_cb)(void);

    // Thread that will fork the requested processes.
    pthread_t pthread;

    // Post to initiate a fork request.
    sem_t sem_begin;
    // Posts to signal request completion.
    sem_t sem_done;

    // Request arguments.
    const char* file;
    char* const * argv;
    char* const * envp;

    // Request result.
    pid_t child_pid;
};

// Function executed by a ForkProxy thread.
void* forkproxy_fn(void* void_forkproxy) {
    ForkProxy* forkproxy = void_forkproxy;
    pid_t shadow_pid = getpid();
    shadow_logger_register(shadow_logger_getDefault(), pthread_self());
    // TODO: ignore SIGCHLD so we don't have to reap children.

    while (1) {
        // Wait for a request.
        if (sem_wait(&forkproxy->sem_begin) != 0) {
            error("sem_wait: %s", g_strerror(errno));
        }

        // fork requested process.
#ifdef SHADOW_COVERAGE
        // The instrumentation in coverage mode causes corruption in between vfork
        // and exec. Use fork instead.
        pid_t pid = fork();
#else
        pid_t pid = vfork();
#endif

        switch (pid) {
            case -1: {
                error("fork: %s", g_strerror(errno));
                break;
            }
            case 0: {
                // Ensure that the child process exits when Shadow does.  Shadow
                // ought to have already tried to terminate the child via SIGTERM
                // before shutting down (though see
                // https://github.com/shadow/shadow/issues/903), so now we jump all
                // the way to SIGKILL.
                if (prctl(PR_SET_PDEATHSIG, SIGKILL) < 0) {
                    error("prctl: %s", g_strerror(errno));
                }
                // Validate that Shadow is still alive (didn't die in between forking and calling
                // prctl).
                if (getppid() != shadow_pid) {
                    error("parent (shadow) exited");
                }
                forkproxy->child_cb();
                if (execvpe(forkproxy->file, forkproxy->argv, forkproxy->envp) < 0) {
                    error("execvpe: %s", g_strerror(errno));
                }
                break;
            }
            default: {
                // Parent
                forkproxy->child_pid = pid;
                info("started process %s with PID %d", forkproxy->file, forkproxy->child_pid);
                forkproxy->parent_cb(pid);
                // Signal calling thread that we're done.
                sem_post(&forkproxy->sem_done);
                break;
            }
        }
    }
}

ForkProxy* forkproxy_new(void (*parent_cb)(pid_t), void (*child_cb)(void)) {
    ForkProxy* forkproxy = malloc(sizeof(*forkproxy));
    *forkproxy = (ForkProxy){
        .parent_cb = parent_cb,
        .child_cb = child_cb,
    };
    if (sem_init(&forkproxy->sem_begin, 0, 0) != 0) {
        error("sem_init: %s", g_strerror(errno));
    }
    if (sem_init(&forkproxy->sem_done, 0, 0) != 0) {
        error("sem_init: %s", g_strerror(errno));
    }
    int rv;
    if ((rv = pthread_create(&forkproxy->pthread, NULL, forkproxy_fn, forkproxy)) != 0) {
        error("pthread_create: %s", g_strerror(rv));
    }
    return forkproxy;
}

pid_t forkproxy_forkExec(ForkProxy* forkproxy, const char* file, char* const argv[], char* const envp[]) {
    forkproxy->file = file;
    forkproxy->argv = argv;
    forkproxy->envp = envp;
    if (sem_post(&forkproxy->sem_begin) != 0) {
        error("sem_post: %s", g_strerror(errno));
    }
    if (sem_wait(&forkproxy->sem_done) != 0) {
        error("sem_wait: %s", g_strerror(errno));
    }
    return forkproxy->child_pid;
}

