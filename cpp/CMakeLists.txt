cmake_minimum_required(VERSION 2.6)
project(zkinterface)

set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
enable_testing()

include_directories(.)
#include_directories(. libsnark libsnark/depends/libff)

link_libraries(ff gmp)

add_definitions(-DCURVE_ALT_BN128)

add_executable(backend backend.cpp)
add_test(backend backend)

add_library(gadget_example gadget_example.cpp)

install(TARGETS gadget_example DESTINATION .)
install(FILES zkinterface.h zkinterface_generated.h libsnark_integration.hpp DESTINATION .)
install(DIRECTORY flatbuffers DESTINATION .)
