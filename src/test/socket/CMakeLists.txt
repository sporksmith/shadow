# Make sure tests in subdirs append to previously collected tests.
set(ALL_SHADOW_TESTS "${ALL_SHADOW_TESTS}")

add_subdirectory(socket)
add_subdirectory(bind)
add_subdirectory(listen)
add_subdirectory(getsockname)
add_subdirectory(accept)
add_subdirectory(connect)
add_subdirectory(getpeername)
add_subdirectory(socketpair)
add_subdirectory(shutdown)
add_subdirectory(sendto_recvfrom)
add_subdirectory(sockopt)
add_subdirectory(ioctl)

# Now set the variable in the parent scope to ours, which includes subdir tests.
set(ALL_SHADOW_TESTS "${ALL_SHADOW_TESTS}" PARENT_SCOPE)
