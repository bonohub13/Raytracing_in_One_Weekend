cmake_policy(SET CMP0076 NEW)

add_library(sphere sphere.c)
target_sources(sphere PUBLIC sphere.h)
target_include_directories(sphere INTERFACE "${CMAKE_CURRENT_SOURCE_DIR}")

target_link_libraries(sphere PUBLIC hittable ray vec3)
