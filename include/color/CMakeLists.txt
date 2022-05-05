cmake_policy(SET CMP0076 NEW)

add_library(color color.c)
target_sources(color PUBLIC color.h)
target_include_directories(color INTERFACE "${CMAKE_CURRENT_SOURCE_DIR}")

target_link_libraries(color PUBLIC vec3)
