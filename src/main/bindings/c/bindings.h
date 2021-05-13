/*
 * The Shadow Simulator
 * See LICENSE for licensing information
 */
// clang-format off


#ifndef main_bindings_h
#define main_bindings_h

/* Warning, this file is autogenerated by cbindgen. Don't modify this manually. */

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include "main/bindings/c/bindings-opaque.h"
#include "main/core/scheduler/scheduler_policy_type.h"
#include "main/host/descriptor/descriptor_types.h"
#include "main/host/status_listener.h"
#include "main/host/syscall_handler.h"
#include "main/host/syscall_types.h"
#include "main/host/thread.h"
#include "main/host/tracker.h"

// Memory allocated by Shadow, in a remote address space.
typedef struct AllocdMem_u8 AllocdMem_u8;

// A queue of byte chunks.
typedef struct ByteQueue ByteQueue;

// Run real applications over simulated networks.
typedef struct CliOptions CliOptions;

typedef struct CompatDescriptor CompatDescriptor;

// Options contained in a configuration file.
typedef struct ConfigFileOptions ConfigFileOptions;

// Shadow configuration options after processing command-line and configuration file options.
typedef struct ConfigOptions ConfigOptions;

// The main counter object that maps individual keys to count values.
typedef struct Counter Counter;

typedef struct HostOptions HostOptions;

// Manages memory of a plugin process.
typedef struct MemoryManager MemoryManager;

// Read-accessor to plugin memory.
typedef struct MemoryReader_u8 MemoryReader_u8;

// Write-accessor to plugin memory.
typedef struct MemoryWriter_u8 MemoryWriter_u8;

// An opaque type used when passing `*const AtomicRefCell<File>` to C.
typedef struct PosixFileArc PosixFileArc;

typedef struct ProcessOptions ProcessOptions;

void rust_logging_init(void);

struct CliOptions *clioptions_parse(int argc, const char *const *argv);

void clioptions_free(struct CliOptions *options);

void clioptions_freeString(char *string);

bool clioptions_getGdb(const struct CliOptions *options);

bool clioptions_getShmCleanup(const struct CliOptions *options);

bool clioptions_getShowBuildInfo(const struct CliOptions *options);

bool clioptions_getShowConfig(const struct CliOptions *options);

char *clioptions_getConfig(const struct CliOptions *options);

struct ConfigFileOptions *configfile_parse(const char *filename);

void configfile_free(struct ConfigFileOptions *config);

struct ConfigOptions *config_new(const struct ConfigFileOptions *config_file,
                                 const struct CliOptions *options);

void config_free(struct ConfigOptions *config);

void config_freeString(char *string);

void config_showConfig(const struct ConfigOptions *config);

unsigned int config_getSeed(const struct ConfigOptions *config);

LogLevel config_getLogLevel(const struct ConfigOptions *config);

SimulationTime config_getHeartbeatInterval(const struct ConfigOptions *config);

SimulationTime config_getRunahead(const struct ConfigOptions *config);

bool config_getUseCpuPinning(const struct ConfigOptions *config);

enum InterposeMethod config_getInterposeMethod(const struct ConfigOptions *config);

bool config_getUseSchedFifo(const struct ConfigOptions *config);

bool config_getUseOnWaitpidWorkarounds(const struct ConfigOptions *config);

bool config_getUseExplicitBlockMessage(const struct ConfigOptions *config);

bool config_getUseSyscallCounters(const struct ConfigOptions *config);

bool config_getUseObjectCounters(const struct ConfigOptions *config);

bool config_getUseMemoryManager(const struct ConfigOptions *config);

bool config_getUseShimSyscallHandler(const struct ConfigOptions *config);

int32_t config_getPreloadSpinMax(const struct ConfigOptions *config);

int32_t config_getMaxConcurrency(const struct ConfigOptions *config);

SimulationTime config_getStopTime(const struct ConfigOptions *config);

SimulationTime config_getBootstrapEndTime(const struct ConfigOptions *config);

unsigned int config_getWorkers(const struct ConfigOptions *config);

SchedulerPolicyType config_getSchedulerPolicy(const struct ConfigOptions *config);

char *config_getDataDirectory(const struct ConfigOptions *config);

char *config_getTemplateDirectory(const struct ConfigOptions *config);

uint64_t config_getSocketRecvBuffer(const struct ConfigOptions *config);

uint64_t config_getSocketSendBuffer(const struct ConfigOptions *config);

bool config_getSocketSendAutotune(const struct ConfigOptions *config);

bool config_getSocketRecvAutotune(const struct ConfigOptions *config);

uint64_t config_getInterfaceBuffer(const struct ConfigOptions *config);

enum QDiscMode config_getInterfaceQdisc(const struct ConfigOptions *config);

char *config_getTopology(const struct ConfigOptions *config);

void config_iterHosts(const struct ConfigOptions *config,
                      void (*f)(const char*, const struct ConfigOptions*, const struct HostOptions*, void*),
                      void *data);

void hostoptions_freeString(char *string);

unsigned int hostoptions_getQuantity(const struct HostOptions *host);

LogLevel hostoptions_getLogLevel(const struct HostOptions *host);

LogLevel hostoptions_getHeartbeatLogLevel(const struct HostOptions *host);

LogInfoFlags hostoptions_getHeartbeatLogInfo(const struct HostOptions *host);

SimulationTime hostoptions_getHeartbeatInterval(const struct HostOptions *host);

char *hostoptions_getPcapDirectory(const struct HostOptions *host);

char *hostoptions_getIpAddressHint(const struct HostOptions *host);

char *hostoptions_getCountryCodeHint(const struct HostOptions *host);

char *hostoptions_getCityCodeHint(const struct HostOptions *host);

uint64_t hostoptions_getBandwidthDown(const struct HostOptions *host);

uint64_t hostoptions_getBandwidthUp(const struct HostOptions *host);

void hostoptions_iterProcesses(const struct HostOptions *host,
                               void (*f)(const struct ProcessOptions*, void*),
                               void *data);

void processoptions_freeString(char *string);

// Will return a NULL pointer if the path does not exist.
char *processoptions_getPath(const struct ProcessOptions *proc);

void processoptions_getArgs(const struct ProcessOptions *proc,
                            void (*f)(const char*, void*),
                            void *data);

char *processoptions_getEnvironment(const struct ProcessOptions *proc);

uint32_t processoptions_getQuantity(const struct ProcessOptions *proc);

SimulationTime processoptions_getStartTime(const struct ProcessOptions *proc);

SimulationTime processoptions_getStopTime(const struct ProcessOptions *proc);

// Parses a string as bits-per-second. Returns '-1' on error.
int64_t parse_bandwidth(const char *s);

// The new compat descriptor takes ownership of the reference to the legacy descriptor and
// does not increment its ref count, but will decrement the ref count when this compat
// descriptor is freed/dropped.
struct CompatDescriptor *compatdescriptor_fromLegacy(LegacyDescriptor *legacy_descriptor);

// If the compat descriptor is a legacy descriptor, returns a pointer to the legacy
// descriptor object. Otherwise returns NULL. The legacy descriptor's ref count is not
// modified, so the pointer must not outlive the lifetime of the compat descriptor.
LegacyDescriptor *compatdescriptor_asLegacy(const struct CompatDescriptor *descriptor);

// When the compat descriptor is freed/dropped, it will decrement the legacy descriptor's
// ref count.
void compatdescriptor_free(struct CompatDescriptor *descriptor);

// This is a no-op for non-legacy descriptors.
void compatdescriptor_setHandle(struct CompatDescriptor *descriptor, int handle);

// If the compat descriptor is a new descriptor, returns a pointer to the reference-counted
// posix file object. Otherwise returns NULL. The posix file object's ref count is not
// modified, so the pointer must not outlive the lifetime of the compat descriptor.
const struct PosixFileArc *compatdescriptor_borrowPosixFile(struct CompatDescriptor *descriptor);

// If the compat descriptor is a new descriptor, returns a pointer to the reference-counted
// posix file object. Otherwise returns NULL. The posix file object's ref count is
// incremented, so the pointer must always later be passed to `posixfile_drop()`, otherwise
// the memory will leak.
const struct PosixFileArc *compatdescriptor_newRefPosixFile(struct CompatDescriptor *descriptor);

// Decrement the ref count of the posix file object. The pointer must not be used after
// calling this function.
void posixfile_drop(const struct PosixFileArc *file);

// Get the status of the posix file object.
Status posixfile_getStatus(const struct PosixFileArc *file);

// Add a status listener to the posix file object. This will increment the status
// listener's ref count, and will decrement the ref count when this status listener is
// removed or when the posix file is freed/dropped.
void posixfile_addListener(const struct PosixFileArc *file, StatusListener *listener);

// Remove a listener from the posix file object.
void posixfile_removeListener(const struct PosixFileArc *file, StatusListener *listener);

// # Safety
// * `thread` must point to a valid object.
struct MemoryManager *memorymanager_new(pid_t pid);

// # Safety
// * `mm` must point to a valid object.
void memorymanager_free(struct MemoryManager *mm);

struct AllocdMem_u8 *allocdmem_new(uintptr_t len);

void allocdmem_free(struct AllocdMem_u8 *allocd_mem);

PluginPtr allocdmem_pluginPtr(const struct AllocdMem_u8 *allocd_mem);

// Initialize the MemoryMapper if it isn't already initialized. `thread` must
// be running and ready to make native syscalls.
void memorymanager_initMapperIfNeeded(struct MemoryManager *memory_manager, Thread *thread);

// Get a read-accessor to the specified plugin memory.
// Must be freed via `memorymanager_freeReader`.
struct MemoryReader_u8 *memorymanager_getReader(struct MemoryManager *memory_manager,
                                                PluginPtr plugin_src,
                                                uintptr_t n);

// Get a write-accessor to the specified plugin memory.
// Must be freed via `memorymanager_flushAndFreeWriter`.
int32_t memorymanager_getStringReader(struct MemoryManager *memory_manager,
                                      PluginPtr plugin_src,
                                      uintptr_t n,
                                      struct MemoryReader_u8 **reader_out,
                                      uintptr_t *strlen);

void memorymanager_freeReader(struct MemoryReader_u8 *reader);

// Get a pointer to this reader's memory.
const void *memorymanager_getReadablePtr(struct MemoryReader_u8 *reader);

// Copy data from this reader's memory.
int32_t memorymanager_readPtr(struct MemoryManager *memory_manager,
                              void *dst,
                              PluginPtr src,
                              uintptr_t n);

// Get a write-accessor to the specified plugin memory.
struct MemoryWriter_u8 *memorymanager_getWriter(struct MemoryManager *memory_manager,
                                                PluginPtr plugin_src,
                                                uintptr_t n);

// Write-back any previously returned writable memory, and free the writer.
int32_t memorymanager_flushAndFreeWriter(struct MemoryWriter_u8 *writer);

// Write data to this writer's memory.
int32_t memorymanager_writePtr(struct MemoryManager *memory_manager,
                               PluginPtr dst,
                               const void *src,
                               uintptr_t n);

// Get a writable pointer to this writer's memory. Initial contents are unspecified.
void *memorymanager_getWritablePtr(struct MemoryWriter_u8 *writer);

// Get a readable and writable pointer to this writer's memory.
void *memorymanager_getMutablePtr(struct MemoryWriter_u8 *writer);

// Fully handles the `brk` syscall, keeping the "heap" mapped in our shared mem file.
SysCallReturn memorymanager_handleBrk(struct MemoryManager *memory_manager,
                                      Thread *thread,
                                      PluginPtr plugin_src);

// Fully handles the `mmap` syscall
SysCallReturn memorymanager_handleMmap(struct MemoryManager *memory_manager,
                                       Thread *thread,
                                       PluginPtr addr,
                                       uintptr_t len,
                                       int32_t prot,
                                       int32_t flags,
                                       int32_t fd,
                                       int64_t offset);

// Fully handles the `munmap` syscall
SysCallReturn memorymanager_handleMunmap(struct MemoryManager *memory_manager,
                                         Thread *thread,
                                         PluginPtr addr,
                                         uintptr_t len);

SysCallReturn memorymanager_handleMremap(struct MemoryManager *memory_manager,
                                         Thread *thread,
                                         PluginPtr old_addr,
                                         uintptr_t old_size,
                                         uintptr_t new_size,
                                         int32_t flags,
                                         PluginPtr new_addr);

SysCallReturn memorymanager_handleMprotect(struct MemoryManager *memory_manager,
                                           Thread *thread,
                                           PluginPtr addr,
                                           uintptr_t size,
                                           int32_t prot);

SysCallReturn rustsyscallhandler_close(SysCallHandler *sys, const SysCallArgs *args);

SysCallReturn rustsyscallhandler_dup(SysCallHandler *sys, const SysCallArgs *args);

SysCallReturn rustsyscallhandler_read(SysCallHandler *sys, const SysCallArgs *args);

SysCallReturn rustsyscallhandler_pread64(SysCallHandler *sys, const SysCallArgs *args);

SysCallReturn rustsyscallhandler_write(SysCallHandler *sys, const SysCallArgs *args);

SysCallReturn rustsyscallhandler_pwrite64(SysCallHandler *sys, const SysCallArgs *args);

SysCallReturn rustsyscallhandler_pipe(SysCallHandler *sys, const SysCallArgs *args);

SysCallReturn rustsyscallhandler_pipe2(SysCallHandler *sys, const SysCallArgs *args);

struct ByteQueue *bytequeue_new(size_t chunk_size);

void bytequeue_free(struct ByteQueue *bq_ptr);

size_t bytequeue_len(struct ByteQueue *bq);

bool bytequeue_isEmpty(struct ByteQueue *bq);

void bytequeue_push(struct ByteQueue *bq, const unsigned char *src, size_t len);

size_t bytequeue_pop(struct ByteQueue *bq, unsigned char *dst, size_t len);

struct Counter *counter_new(void);

void counter_free(struct Counter *counter_ptr);

int64_t counter_add_value(struct Counter *counter, const char *id, int64_t value);

int64_t counter_sub_value(struct Counter *counter, const char *id, int64_t value);

void counter_add_counter(struct Counter *counter, struct Counter *other);

void counter_sub_counter(struct Counter *counter, struct Counter *other);

bool counter_equals_counter(const struct Counter *counter, const struct Counter *other);

// Creates a new string representation of the counter, e.g., for logging.
// The returned string must be free'd by passing it to counter_free_string.
char *counter_alloc_string(struct Counter *counter);

// Frees a string previously returned from counter_alloc_string.
void counter_free_string(struct Counter *counter, char *ptr);

#endif /* main_bindings_h */
