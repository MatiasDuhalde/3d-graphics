cmake_minimum_required(VERSION 3.20.0)
project(Raytracing VERSION 0.1.0 LANGUAGES C CXX)

include(CTest)
enable_testing()

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(THREADS_PREFER_PTHREAD_FLAG ON)

add_executable(Raytracing main.cpp)

set(CPACK_PROJECT_NAME ${PROJECT_NAME})
set(CPACK_PROJECT_VERSION ${PROJECT_VERSION})
include(CPack)


# If release use multi-threading

if (CMAKE_BUILD_TYPE STREQUAL "Release")
    find_package(OpenMP)

    if(OpenMP_CXX_FOUND)
        target_link_libraries(Raytracing PUBLIC OpenMP::OpenMP_CXX)
    endif()
endif()

if (CMAKE_CXX_COMPILER_ID STREQUAL "GNU" OR CMAKE_CXX_COMPILER_ID STREQUAL "Clang")
    target_compile_options(Raytracing PUBLIC -fdiagnostics-color=always)
endif()


find_package(Threads REQUIRED)

if(THREADS_HAVE_PTHREAD_ARG AND CMAKE_THREAD_LIBS_INIT)
    target_link_libraries(Raytracing PUBLIC Threads::Threads)
endif()

set(CMAKE_CXX_FLAGS "-Wall -Wextra")
set(CMAKE_CXX_FLAGS_DEBUG "-g")
set(CMAKE_CXX_FLAGS_RELEASE "-O3")

