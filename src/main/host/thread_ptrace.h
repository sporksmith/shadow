#ifndef SRC_MAIN_HOST_SHD_THREAD_PTRACE_H_
#define SRC_MAIN_HOST_SHD_THREAD_PTRACE_H_

#include "main/host/syscall_handler.h"
#include "main/host/syscall_types.h"
#include "main/host/thread.h"

// Create a thread managed via ptrace and shim-ipc.
Thread* threadptrace_new(Host* host, Process* process, gint threadID);

// Create a thread managed via ptrace only.
Thread* threadptraceonly_new(Host* host, Process* process, gint threadID);

// Returns true if threadptrace_detach needs to be called to allow the Thread
// to be migrated across worker threads.
bool threadptrace_needsToDetachBeforeMigrating();

void threadptrace_detach(Thread* base);

#endif
