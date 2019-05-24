cmake_minimum_required(VERSION 2.6)
project(AtomVM)

include_directories(
    ${CMAKE_CURRENT_BINARY_DIR}
    libAtomVM/
    ${CMAKE_SOURCE_DIR}/src/rust/include
)

link_directories(
    ${CMAKE_BINARY_DIR}/src/rust
)

option(ADVANCED_TRACING "Enable advanced tracing" OFF)
if (ADVANCED_TRACING)
    add_definitions(-DENABLE_ADVANCED_TRACE)
endif ()

add_subdirectory(libAtomVM)

add_subdirectory(platforms/generic_unix)

set(PLATFORM_LIB_SUFFIX
    ${CMAKE_SYSTEM_NAME}-${CMAKE_SYSTEM_PROCESSOR})

if (CMAKE_COMPILER_IS_GNUCC)
    set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -Wall -pedantic -Wextra -ggdb")
endif ()

add_executable(
    AtomVM
    main.c
    ../rust/include/atomvm_rust.h
)

add_dependencies(
    AtomVM
    libAtomVM
    libAtomVM${PLATFORM_LIB_SUFFIX}
    rust
)

target_link_libraries(
    AtomVM
    libAtomVM
    libAtomVM${PLATFORM_LIB_SUFFIX}
    rust
    # required by rust library
    pthread
    dl
)

set_property(
    TARGET AtomVM
    PROPERTY C_STANDARD 99
)

if (CMAKE_BUILD_TYPE STREQUAL "Coverage")
    set_target_properties(
        AtomVM
        PROPERTIES COMPILE_FLAGS "-O0 -fprofile-arcs -ftest-coverage")
endif ()
