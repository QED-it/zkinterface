cmake_minimum_required(VERSION 2.6)
if (CMAKE_INSTALL_PREFIX_INITIALIZED_TO_DEFAULT)
    set(CMAKE_INSTALL_PREFIX "${CMAKE_SOURCE_DIR}/target" CACHE PATH "default install path" FORCE)
endif ()

# Optimizations above -O1 trigger this bug on MacOS: https://github.com/CodaProtocol/coda/pull/1419
set(CMAKE_CXX_FLAGS_RELEASE "${CMAKE_CXX_FLAGS_RELEASE} -O1")

project(zkinterface)

set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
enable_testing()


# Frontend gadgetlib.
add_executable(gadgetlib gadgetlib.cpp gadgetlib_example.cpp  libsnark_integration.cpp)
# Dependencies.
target_link_libraries(gadgetlib ff gmp)
target_include_directories(gadgetlib PRIVATE . libsnark libsnark/depends/libff libsnark/depends/libfqfft /usr/local/include)
target_link_directories(gadgetlib PRIVATE ${CMAKE_SOURCE_DIR}/libsnark/libsnark/ ${CMAKE_SOURCE_DIR}/libsnark/depends/libff/libff/ /usr/local/lib)
# Configuration from libsnark/libsnark/CMakeFiles/snark.dir/flags.make
target_compile_definitions(gadgetlib PRIVATE -DBINARY_OUTPUT -DCURVE_ALT_BN128 -DMONTGOMERY_OUTPUT -DNO_PROCPS -DUSE_ASM)


# Backend.
add_executable(backend backend.cpp libsnark_integration.cpp)
add_test(backend backend)
# Dependencies.
target_link_libraries(backend ff gmp)
target_include_directories(backend PRIVATE . libsnark libsnark/depends/libff libsnark/depends/libfqfft /usr/local/include)
target_link_directories(backend PRIVATE ${CMAKE_SOURCE_DIR}/libsnark/libsnark/ ${CMAKE_SOURCE_DIR}/libsnark/depends/libff/libff/ /usr/local/lib)
# Configuration from libsnark/libsnark/CMakeFiles/snark.dir/flags.make
target_compile_definitions(backend PRIVATE -DBINARY_OUTPUT -DCURVE_ALT_BN128 -DMONTGOMERY_OUTPUT -DNO_PROCPS -DUSE_ASM)

install(TARGETS gadgetlib DESTINATION .)
install(FILES zkinterface.h zkinterface_generated.h libsnark_integration.hpp ../LICENSE DESTINATION .)
install(DIRECTORY flatbuffers DESTINATION .)
