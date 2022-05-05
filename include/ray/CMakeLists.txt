cmake_policy(SET CMP0076 NEW)

add_library(ray ray.c)
target_sources(ray PUBLIC ray.h)
target_include_directories(ray INTERFACE "${CMAKE_CURRENT_SOURCE_DIR}")

target_link_libraries(ray PUBLIC vec3)
