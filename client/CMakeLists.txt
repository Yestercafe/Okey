cmake_minimum_required(VERSION 3.10)
set(PROJECT_NAME Okey)

project(${PROJECT_NAME})

set(CMAKE_CXX_STANDARD 20)

include("cmake/settings.cmake")

add_subdirectory(src)
