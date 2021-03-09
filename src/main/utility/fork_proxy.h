#ifndef FORK_PROXY_H
#define FORK_PROXY_H

#include <sys/types.h>

// An object for forking processes on a separate thread.  `parent_cb` will be
// executed in the parent after each fork, and `child_cb` will be executed in
// the child before calling `exec`.
//
// ForkProxy itself is *not* thread safe.
typedef struct _ForkProxy ForkProxy;

// Creates a new ForkProxy.  `parent_cb` will be executed in the parent after
// each fork, and `child_cb` will be executed in the child before calling
// `exec`.
ForkProxy* forkproxy_new(void (*parent_cb)(pid_t), void (*child_cb)(void));

// Fork and exec the specified program. Uses `fork` when SHADOW_COVERAGE is
// defined, and `vfork` otherwise.
pid_t forkproxy_forkExec(ForkProxy* forkproxy, const char* file, char* const argv[],
                         char* const envp[]);

#endif
