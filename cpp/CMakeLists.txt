cmake_minimum_required(VERSION 2.6)
project(ZKStandard)

set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(zkcomponent gadget.cpp libsnark_integration.cpp)
install(TARGETS zkcomponent DESTINATION .)
install(FILES gadget.h DESTINATION .)

include_directories("$ENV{LIBSNARK_SOURCE_PATH}")
include_directories("$ENV{LIBSNARK_SOURCE_PATH}/depends/libff")
include_directories("$ENV{LIBSNARK_SOURCE_PATH}/depends/libfqfft")
add_definitions(
        -DWITH_PROCPS=OFF
        -DCURVE=ALT_BN128
        -DCURVE_ALT_BN128
        -DUSE_PT_COMPRESSION=OFF
        -DMONTGOMERY_OUTPUT=ON
        -DBINARY_OUTPUT=ON
)
add_library(zkcomponent_libsnark libsnark_integration.cpp)
install(TARGETS zkcomponent_libsnark DESTINATION .)
