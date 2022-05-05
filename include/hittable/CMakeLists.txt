cmake_policy(SET CMP0076 NEW)

add_library(hittable hittable.c)
target_sources(hittable PUBLIC hittable.h)
target_include_directories(hittable INTERFACE "${CMAKE_CURRENT_SOURCE_DIR}")

target_link_libraries(hittable PUBLIC ray vec3)
