cmake_minimum_required(VERSION 3.31)
project(crpfgcpp_to_rust C CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

set(BUILD_TESTING OFF CACHE BOOL "" FORCE)
set(BUILD_EXAMPLES OFF CACHE BOOL "" FORCE)
set(BUILD_SHARED_LIBS OFF CACHE BOOL "" FORCE)

include(FetchContent)

set(LIB_NAME "crpdfcpp")

FetchContent_Declare(
    ${LIB_NAME}
    GIT_REPOSITORY https://github.com/rsvzz/crpdfcpp.git
    GIT_TAG        bd74908704c0e312b6e342986975a33c814c6ce5 # tag or commit
    SYSTEM
)

FetchContent_MakeAvailable(${LIB_NAME})