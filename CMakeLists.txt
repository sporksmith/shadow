## (c) 2010-2012 Shadow, Rob Jansen jansen@cs.umn.edu

## set build parameters
project(Shadow)
set(SHADOW_VERSION_FULL 2.0.0-pre.2)

## This tells cmake to generate a database of compilation commands `compile_commands.json`,
## which is used by tools such as YouCompleteMe and include-what-you-use.
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

set (CMAKE_CXX_STANDARD 11)

message(STATUS "System name: ${CMAKE_SYSTEM_NAME}")
message(STATUS "System version: ${CMAKE_SYSTEM_VERSION}")
message(STATUS "System processor: ${CMAKE_SYSTEM_PROCESSOR}")

## ensure cmake version
cmake_minimum_required(VERSION 3.2 FATAL_ERROR)

## building with support for C11 on platforms that default to C99 or C89
set (CMAKE_C_FLAGS "-std=gnu11 ${CMAKE_C_FLAGS}")

## ensure unix environment (CMAKE_SYSTEM_NAME == "Linux")
if((NOT UNIX) OR (NOT (CMAKE_SYSTEM_NAME STREQUAL "Linux")))
    message(FATAL_ERROR "Shadow requires a Unix/Linux environment.")
endif((NOT UNIX) OR (NOT (CMAKE_SYSTEM_NAME STREQUAL "Linux")))

## ensure out-of-source build
if(${CMAKE_SOURCE_DIR} STREQUAL ${CMAKE_BINARY_DIR})
    message(FATAL_ERROR "Shadow requires an out-of-source build. Please create a separate build directory and run 'cmake path/to/shadow [options]' there.")
endif(${CMAKE_SOURCE_DIR} STREQUAL ${CMAKE_BINARY_DIR})

## additional user-defined include directories
foreach(include ${CMAKE_EXTRA_INCLUDES})
    include_directories(${include})
    set(CMAKE_MODULE_PATH "${include}" ${CMAKE_MODULE_PATH})
endforeach(include)
set(CMAKE_MODULE_PATH "${CMAKE_SOURCE_DIR}/cmake/" ${CMAKE_MODULE_PATH})

message(STATUS "CMAKE_MODULE_PATH = ${CMAKE_MODULE_PATH}")

## Mismatched C and C++ compilers can result in confusing errors at link time.
## Try to catch it here.
if(((CMAKE_C_COMPILER MATCHES "[/^]gcc") AND (NOT (CMAKE_CXX_COMPILER MATCHES "[/^]g\\+\\+")))
   OR ((CMAKE_C_COMPILER MATCHES "[/^]clang") AND (NOT (CMAKE_CXX_COMPILER MATCHES "[/^]clang\\+\\+"))))
    message(FATAL_ERROR "Mismatched C and C++ compiler ${CMAKE_C_COMPILER} and ${CMAKE_CXX_COMPILER}")
endif()

## additional user-defined library directories
foreach(library ${CMAKE_EXTRA_LIBRARIES})
    link_directories(${library})
endforeach(library)
set(CMAKE_INSTALL_RPATH_USE_LINK_PATH TRUE)

## use the installed shim path only when shadow is installed
set(CMAKE_INSTALL_RPATH "${CMAKE_INSTALL_PREFIX}/lib")

## get general includes
include(CheckIncludeFile)
include(CheckFunctionExists)
include(CheckLibraryExists)
include(TestBigEndian)
include(ShadowTools)

## general tests and configurations
TEST_BIG_ENDIAN(IS_BIG_ENDIAN)
set(CMAKE_INCLUDE_DIRECTORIES_BEFORE ON)
set(CMAKE_INCLUDE_DIRECTORIES_PROJECT_BEFORE ON)
#set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)
#set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)

## Check if we are able to compile various compilation units
compile_test(HAVE_STRUCT_STATX 
            ${CMAKE_SOURCE_DIR}/cmake/compile_tests/struct_statx.c)
compile_test(HAVE_STRUCT_STATX_TIMESTAMP 
            ${CMAKE_SOURCE_DIR}/cmake/compile_tests/struct_statx_timestamp.c)
compile_test(HAVE_STRUCT_LINUX_DIRENT 
            ${CMAKE_SOURCE_DIR}/cmake/compile_tests/struct_linux_dirent.c)
compile_test(HAVE_STRUCT_LINUX_DIRENT64 
            ${CMAKE_SOURCE_DIR}/cmake/compile_tests/struct_linux_dirent64.c)

## construct info/version strings
string(REGEX REPLACE "^([0-9]+)\\.[0-9]+\\.[0-9]+(\\-[a-z]+)?(\\.[0-9]+)?$" "\\1" SHADOW_VERSION_MAJOR ${SHADOW_VERSION_FULL})
string(REGEX REPLACE "^[0-9]+\\.([0-9]+)\\.[0-9]+(\\-[a-z]+)?(\\.[0-9]+)?$" "\\1" SHADOW_VERSION_MINOR ${SHADOW_VERSION_FULL})
string(REGEX REPLACE "^[0-9]+\\.[0-9]+\\.([0-9]+)(\\-[a-z]+)?(\\.[0-9]+)?$" "\\1" SHADOW_VERSION_PATCH ${SHADOW_VERSION_FULL})

message(STATUS "SHADOW_VERSION_FULL=${SHADOW_VERSION_FULL}")
message(STATUS "SHADOW_VERSION_MAJOR=${SHADOW_VERSION_MAJOR}")
message(STATUS "SHADOW_VERSION_MINOR=${SHADOW_VERSION_MINOR}")
message(STATUS "SHADOW_VERSION_PATCH=${SHADOW_VERSION_PATCH}")

set(SHADOW_VERSION_STRING_CONF "Shadow")

if(EXISTS ${CMAKE_SOURCE_DIR}/.git)
    ## current git commit version and hash
    EXECUTE_PROCESS(COMMAND "git" "describe" "--long" "--dirty" OUTPUT_VARIABLE DESCRIBE)
    if(DESCRIBE)
        string(REGEX REPLACE "\n" "" DESCRIBE ${DESCRIBE})
        set(SHADOW_VERSION_STRING_CONF "${SHADOW_VERSION_STRING_CONF} ${DESCRIBE}")
    endif(DESCRIBE)

    ## current git commit short date
    EXECUTE_PROCESS(COMMAND "git" "log" "--pretty=format:%ad" "--date=format:%Y-%m-%d--%H:%M:%S" "-n"  "1" OUTPUT_VARIABLE COMMITDATE)
    if(COMMITDATE)
        set(SHADOW_VERSION_STRING_CONF "${SHADOW_VERSION_STRING_CONF} ${COMMITDATE}")
    endif(COMMITDATE)
else()
    set(SHADOW_VERSION_STRING_CONF "${SHADOW_VERSION_STRING_CONF} v${SHADOW_VERSION_MAJOR}.${SHADOW_VERSION_MINOR}.${SHADOW_VERSION_PATCH}")
endif()

message(STATUS "Building ${SHADOW_VERSION_STRING_CONF}")

## setup shadow options
option(SHADOW_PROFILE "build with profile settings (default: OFF)" OFF)
option(SHADOW_TEST "build tests (default: OFF)" OFF)
option(SHADOW_EXPORT "export service libraries and headers (default: OFF)" OFF)
option(SHADOW_WERROR "turn compiler warnings into errors. (default: OFF)" OFF)
option(SHADOW_COVERAGE "enable code-coverage instrumentation. (default: OFF)" OFF)
option(SHADOW_USE_C_SYSCALLS "use only the C syscall handlers. (default: OFF)" OFF)
option(SHADOW_USE_PERF_TIMERS "compile in timers for tracking the run time of various internal operations. (default: OFF)" OFF)

## display selected user options
MESSAGE(STATUS)
MESSAGE(STATUS "-------------------------------------------------------------------------------")
MESSAGE(STATUS "Current settings: (change with '$ cmake -D<OPTION>=<ON|OFF>')")
MESSAGE(STATUS "SHADOW_PROFILE=${SHADOW_PROFILE}")
MESSAGE(STATUS "SHADOW_TEST=${SHADOW_TEST}")
MESSAGE(STATUS "SHADOW_EXPORT=${SHADOW_EXPORT}")
MESSAGE(STATUS "SHADOW_WERROR=${SHADOW_WERROR}")
MESSAGE(STATUS "SHADOW_COVERAGE=${SHADOW_COVERAGE}")
MESSAGE(STATUS "SHADOW_USE_C_SYSCALLS=${SHADOW_USE_C_SYSCALLS}")
MESSAGE(STATUS "SHADOW_USE_PERF_TIMERS=${SHADOW_USE_PERF_TIMERS}")
MESSAGE(STATUS "-------------------------------------------------------------------------------")
MESSAGE(STATUS)

## now handle the options, set up our own flags
set(CMAKE_C_FLAGS_DEBUG "")
set(CMAKE_C_FLAGS_RELEASE "")
add_compile_options(-ggdb)
add_compile_options(-fno-omit-frame-pointer)
add_compile_options(-Wreturn-type)
add_compile_options(-Wswitch)

if("${CMAKE_BUILD_TYPE}" STREQUAL "Debug")
    # we only check for unused functions when builing in debug mode since some
    # functions are only called when logging, which can be #ifdef'd out in
    # release mode
    add_compile_options(-Wunused-function)
endif()

if("${CMAKE_BUILD_TYPE}" STREQUAL "")
    set(CMAKE_BUILD_TYPE "Release")
endif()

message(STATUS "Found CMAKE_BUILD_TYPE='${CMAKE_BUILD_TYPE}'")

if("${CMAKE_BUILD_TYPE}" STREQUAL "Debug")
    message(STATUS "Debug build enabled.")
    add_definitions(-DDEBUG)
    add_compile_options(-O0)
elseif("${CMAKE_BUILD_TYPE}" STREQUAL "Release")
    message(STATUS "Release build enabled.")
    add_definitions(-DNDEBUG)
    add_compile_options(-O3)
else()
    MESSAGE(FATAL_ERROR "Unknown build type '${CMAKE_BUILD_TYPE}'; valid types are 'Release' or 'Debug'")
endif()

if(SHADOW_WERROR STREQUAL ON)
    add_compile_options(-Werror)
endif(SHADOW_WERROR STREQUAL ON)

if(SHADOW_COVERAGE STREQUAL ON)
    add_compile_options(--coverage)
    add_definitions(-DSHADOW_COVERAGE)
    add_ldflags(--coverage)
    ## from https://github.com/mozilla/grcov
    set(RUSTFLAGS "-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code \
                   -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort")
    set(CARGO_ENV_VARS "CARGO_INCREMENTAL=0 RUSTDOCFLAGS=\"-Cpanic=abort\"")
else()
    set(RUSTFLAGS "-C force-frame-pointers=y")
    set(CARGO_ENV_VARS "")
endif(SHADOW_COVERAGE STREQUAL ON)

if(SHADOW_USE_C_SYSCALLS STREQUAL ON)
    message(STATUS "C syscalls enabled")
    add_definitions(-DUSE_C_SYSCALLS)
endif()

if(SHADOW_USE_PERF_TIMERS STREQUAL ON)
    message(STATUS "Perf timers enabled")
    add_definitions(-DUSE_PERF_TIMERS)
endif()

if($ENV{VERBOSE})
    add_definitions(-DVERBOSE)
endif()

if(SHADOW_PROFILE STREQUAL ON)
    add_definitions(-DDEBUG)
    ## see src/main/CMakeLists.txt, where we add the -pg flags
endif(SHADOW_PROFILE STREQUAL ON)

if(SHADOW_EXPORT STREQUAL ON)
    ## the actual work happens in the CMakeLists files in each plug-in directory
    MESSAGE(STATUS "will export Shadow plug-in service libraries and headers")
endif(SHADOW_EXPORT STREQUAL ON)

#if(POLICY  CMP0026)
#    cmake_policy(SET  CMP0026  OLD)
#endif()

if(SHADOW_TEST STREQUAL ON)
    ## http://www.cmake.org/Wiki/CMake_Testing_With_CTest
    message(STATUS "SHADOW_TEST enabled")
    enable_testing()
endif(SHADOW_TEST STREQUAL ON)

## recurse our project tree
add_subdirectory(${CMAKE_SOURCE_DIR}/src/)

## Add more build info that will appear in 'shadow -v' and in the log file
## This should come after the project tree recursion so we can pick up the
## compile options for the shadow target
string(TIMESTAMP BUILDDATE "%Y-%m-%d--%H:%M:%S")
get_target_property(SHADOW_COMPILE_OPTIONS shadow COMPILE_OPTIONS)
get_target_property(SHADOW_LINK_OPTIONS shadow LINK_FLAGS)
set(SHADOW_BUILD_STRING_CONF "Shadow was built")
if(EXISTS ${CMAKE_SOURCE_DIR}/.git)
    ## current branch name (will be added)
    EXECUTE_PROCESS(COMMAND "git" "rev-parse" "--abbrev-ref" "HEAD" OUTPUT_VARIABLE BRANCHNAME)
    if(BRANCHNAME)
        string(REGEX REPLACE "\n" "" BRANCHNAME ${BRANCHNAME})
        set(SHADOW_BUILD_STRING_CONF "${SHADOW_BUILD_STRING_CONF} from branch ${BRANCHNAME}")
    endif(BRANCHNAME)
endif()
if(BUILDDATE)
    set(SHADOW_BUILD_STRING_CONF "${SHADOW_BUILD_STRING_CONF} on ${BUILDDATE}")
endif(BUILDDATE)
if(CMAKE_BUILD_TYPE)
    set(SHADOW_BUILD_STRING_CONF "${SHADOW_BUILD_STRING_CONF} in ${CMAKE_BUILD_TYPE} mode")
endif(CMAKE_BUILD_TYPE)
if(SHADOW_COMPILE_OPTIONS)
    set(SHADOW_BUILD_STRING_CONF "${SHADOW_BUILD_STRING_CONF} with compile options: ${SHADOW_COMPILE_OPTIONS}")
endif(SHADOW_COMPILE_OPTIONS)
if(SHADOW_LINK_OPTIONS)
    set(SHADOW_BUILD_STRING_CONF "${SHADOW_BUILD_STRING_CONF} and link options: ${SHADOW_LINK_OPTIONS}")
endif(SHADOW_LINK_OPTIONS)
## build info
set(SHADOW_INFO_STRING_CONF "For more information, visit https://shadow.github.io or https://github.com/shadow")

## generate config header and make sure its on the include path
## this should come after create the version info to make sure it's included
configure_file(${CMAKE_CURRENT_SOURCE_DIR}/config.h.in ${CMAKE_BINARY_DIR}/src/shd-config.h)
