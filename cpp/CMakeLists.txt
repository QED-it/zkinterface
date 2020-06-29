cmake_minimum_required(VERSION 3.5)
if (CMAKE_INSTALL_PREFIX_INITIALIZED_TO_DEFAULT)
    set(CMAKE_INSTALL_PREFIX "${CMAKE_SOURCE_DIR}/target" CACHE PATH "default install path" FORCE)
endif ()

# Optimizations above -O1 trigger this bug on MacOS: https://github.com/CodaProtocol/coda/pull/1419
set(CMAKE_CXX_FLAGS_RELEASE "${CMAKE_CXX_FLAGS_RELEASE} -O1")

project(zkinterface)

set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
enable_testing()

# libsnark dependencies.
include_directories(. libsnark libsnark/depends/libff libsnark/depends/libfqfft /usr/local/include)
link_directories(${CMAKE_SOURCE_DIR}/libsnark/libsnark/ ${CMAKE_SOURCE_DIR}/libsnark/depends/libff/libff/ /usr/local/lib)

# Frontend gadget lib.
add_library(zkif_gadgetlib STATIC gadgetlib.cpp zkinterface_utils.cpp libsnark_converters.cpp gadgetlib_example.cpp gadgetlib_alu.cpp)
# Dependencies.
target_link_libraries(zkif_gadgetlib ff gmp)
# Configuration from libsnark/libsnark/CMakeFiles/snark.dir/flags.make
target_compile_definitions(zkif_gadgetlib PUBLIC -DBINARY_OUTPUT -DCURVE_ALT_BN128 -DMONTGOMERY_OUTPUT -DNO_PROCPS -DUSE_ASM)

# Frontend gadget CLI.
add_executable(zkif_gadget gadget_cli.cpp)
target_link_libraries(zkif_gadget zkif_gadgetlib)

# Backend.
add_executable(zkif_snark snark_cli.cpp libsnark_importer.cpp)
target_link_libraries(zkif_snark zkif_gadgetlib)

install(TARGETS zkif_gadgetlib DESTINATION .)
install(FILES gadgetlib.h zkinterface_generated.h libsnark_converters.hpp ../LICENSE DESTINATION .)
install(DIRECTORY flatbuffers DESTINATION .)
