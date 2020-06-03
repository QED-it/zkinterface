cmake_minimum_required(VERSION 2.6)
if(CMAKE_INSTALL_PREFIX_INITIALIZED_TO_DEFAULT)
    set(CMAKE_INSTALL_PREFIX "${CMAKE_SOURCE_DIR}/target" CACHE PATH "default install path" FORCE )
endif()

project(zkinterface)

set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(gadget_example gadget_example.cpp)
include_directories(.)

install(TARGETS gadget_example DESTINATION .)
install(FILES zkinterface.h zkinterface_generated.h libsnark_integration.hpp ../LICENSE DESTINATION .)
install(DIRECTORY flatbuffers DESTINATION .)
