cmake_minimum_required(VERSION 2.6)
project(ZKStandard)

set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(gadget_example gadget.cpp)
include_directories(.)

install(TARGETS gadget_example DESTINATION .)
install(FILES gadget.h gadget_generated.h libsnark_integration.hpp DESTINATION .)
install(DIRECTORY flatbuffers DESTINATION .)
