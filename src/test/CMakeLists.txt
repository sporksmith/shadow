## testing for the shadow simulator
    
## these are common flags that are needed for shadow plugins
add_cflags("-fPIC -fno-inline -fno-strict-aliasing -U_FORTIFY_SOURCE -Wno-unused-command-line-argument")
add_cflags(-std=gnu99)
add_definitions(-fPIC -g3 -DDEBUG -D_GNU_SOURCE)

## ensure that the Host LLVM plugin is built first
#get_property(LLVMHoistGlobalsPATH TARGET LLVMHoistGlobals PROPERTY LOCATION)

include(ExternalProject)

## always build tests with the debug profile, even when shadow is built in release mode
set(CARGO_ENV_VARS "${CARGO_ENV_VARS} RUSTFLAGS=\"${RUSTFLAGS}\"")
ExternalProject_Add(
   shadow-tests
   PREFIX ${CMAKE_CURRENT_BINARY_DIR}
   BINARY_DIR ${CMAKE_CURRENT_SOURCE_DIR}
   BUILD_ALWAYS 1
   DOWNLOAD_COMMAND ""
   CONFIGURE_COMMAND ""
   BUILD_COMMAND bash -c "${CARGO_ENV_VARS} cargo build --target-dir \"${CMAKE_CURRENT_BINARY_DIR}/target\""
   INSTALL_COMMAND ""
   LOG_BUILD OFF
)

## a helper program for tests needing multiple processes
add_executable(shadow-test-launcher test_launcher.c test_launcher_common.c)
add_executable(shadow-test-launcher-fail test_launcher_fail.c test_launcher_common.c)

## === Helper macros for consistently defining tests. ===
## Tests that need more features should use add_test directly.

## example: add_linux_tests(BASENAME bind COMMAND test_bind)
## will create one test named bind-linux
macro(add_linux_tests)
   cmake_parse_arguments(LINUX_TEST "" "BASENAME" "COMMAND" ${ARGN})
   add_test(NAME ${LINUX_TEST_BASENAME}-linux COMMAND ${LINUX_TEST_COMMAND})
   set_property(TEST ${LINUX_TEST_BASENAME}-linux PROPERTY ENVIRONMENT "RUST_BACKTRACE=1")
endmacro()

## example: add_shadow_tests(BASENAME bind METHODS ptrace preload LOGLEVEL debug ARGS --pin-cpus)
## will create two tests named bind-shadow-ptrace and bind-shadow-preload
macro(add_shadow_tests)
   cmake_parse_arguments(SHADOW_TEST "" "BASENAME;LOGLEVEL;SHADOW_CONFIG;CHECK_RETVAL;POST_CMD" "METHODS;ARGS;CONFIGURATIONS;PROPERTIES" ${ARGN})

   if(NOT DEFINED SHADOW_TEST_LOGLEVEL)
      if("${CMAKE_BUILD_TYPE}" STREQUAL "Debug")
         set(SHADOW_TEST_LOGLEVEL "trace")
      else()
         set(SHADOW_TEST_LOGLEVEL "info")
      endif()
   endif()

   if(NOT DEFINED SHADOW_TEST_SHADOW_CONFIG)
       set(SHADOW_TEST_SHADOW_CONFIG "${CMAKE_CURRENT_SOURCE_DIR}/${SHADOW_TEST_BASENAME}.yaml")
   endif()

   if(NOT DEFINED SHADOW_TEST_CHECK_RETVAL)
      set(SHADOW_TEST_CHECK_RETVAL TRUE)
   endif()

   if(NOT DEFINED SHADOW_TEST_POST_CMD)
      set(SHADOW_TEST_POST_CMD "true")
   endif()
   
   # If cpu pinning is not explicitly set, we disable it so we avoid pinning
   # multiple tests that should be running in parallel to the same exact CPUs.
   if(NOT "${SHADOW_TEST_ARGS}" MATCHES ".*--use-cpu-pinning.*")
      list(APPEND SHADOW_TEST_ARGS "--use-cpu-pinning" "false")
   endif()

   string(REPLACE ";" " " SHADOW_TEST_ARGS "${SHADOW_TEST_ARGS}")

   foreach(SHADOW_TEST_METHOD ${SHADOW_TEST_METHODS})
      set(SHADOW_TEST_NAME ${SHADOW_TEST_BASENAME}-shadow-${SHADOW_TEST_METHOD})
      
      set(SHADOW_TEST_COMMAND sh -c "\
         rm -rf ${SHADOW_TEST_NAME}.data \
         && ${CMAKE_BINARY_DIR}/src/main/shadow \
         --data-directory=${SHADOW_TEST_NAME}.data \
         --interpose-method=${SHADOW_TEST_METHOD} \
         --log-level=${SHADOW_TEST_LOGLEVEL} \
         ${SHADOW_TEST_ARGS} \
         ${SHADOW_TEST_SHADOW_CONFIG} \
         && (cd ${SHADOW_TEST_NAME}.data && ${SHADOW_TEST_POST_CMD}) \
         "
      )
      
      if(DEFINED SHADOW_TEST_CONFIGURATIONS)
         add_test(
            NAME ${SHADOW_TEST_NAME}
            COMMAND ${SHADOW_TEST_COMMAND}
            CONFIGURATIONS ${SHADOW_TEST_CONFIGURATIONS}
         )
      else()
         add_test(
            NAME ${SHADOW_TEST_NAME}
            COMMAND ${SHADOW_TEST_COMMAND}
         )
      endif()

      # need to check the test's return code, not just shadow's (see shadow/shadow#902)
      if(SHADOW_TEST_CHECK_RETVAL)
         set_property(TEST ${SHADOW_TEST_NAME} PROPERTY FAIL_REGULAR_EXPRESSION "main error code '.*' for process")
      endif()
      set_property(TEST ${SHADOW_TEST_NAME} PROPERTY ENVIRONMENT "RUST_BACKTRACE=1")

      # check that we didn't leak any reference-counted objects
      set_property(TEST ${SHADOW_TEST_NAME} PROPERTY FAIL_REGULAR_EXPRESSION "Memory leak detected")

      if(DEFINED SHADOW_TEST_PROPERTIES)
         set_tests_properties(${SHADOW_TEST_NAME} PROPERTIES ${SHADOW_TEST_PROPERTIES})
      endif()

      set(ALL_SHADOW_TESTS "${ALL_SHADOW_TESTS}" PARENT_SCOPE)
      list(APPEND ALL_SHADOW_TESTS ${SHADOW_TEST_NAME})
   endforeach()
endmacro()
## === end test helper macros ===

## add the test directories
# FIXME uncomment these as we get them working in Phantom.
# FIXME add_subdirectory(dynlink)
# FIXME add_subdirectory(preload)

add_subdirectory(bindc)
add_subdirectory(clone)
add_subdirectory(config)
add_subdirectory(cpp)
add_subdirectory(determinism)
add_subdirectory(epoll)
add_subdirectory(eventfd)
add_subdirectory(exit)
add_subdirectory(file)
add_subdirectory(futex)
add_subdirectory(ifaddrs)
add_subdirectory(memory)
add_subdirectory(phold)
add_subdirectory(pipe)
add_subdirectory(poll)
add_subdirectory(random)
add_subdirectory(resolver)
add_subdirectory(signal)
add_subdirectory(sleep)
add_subdirectory(sockbuf)
add_subdirectory(socket)
add_subdirectory(tcp)
add_subdirectory(threads)
add_subdirectory(timerfd)
add_subdirectory(tor)
add_subdirectory(udp)
add_subdirectory(unistd)

list(LENGTH ALL_SHADOW_TESTS ALL_SHADOW_TESTS_LENGTH)
message(STATUS "Configured to build ${ALL_SHADOW_TESTS_LENGTH} Shadow tests.")
message(STATUS "ALL_SHADOW_TESTS = ${ALL_SHADOW_TESTS}")
