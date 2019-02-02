cmake_minimum_required(VERSION 2.6)
project(ZKStandard)

set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(zkcomponent gadget.cpp)
install(TARGETS zkcomponent DESTINATION .)
install(FILES gadget.h gadget_generated.h DESTINATION .)
