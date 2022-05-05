cmake_policy(SET CMP0076 NEW)

add_library(renderer renderer.c)
target_sources(renderer PUBLIC renderer.h)
target_include_directories(renderer INTERFACE "${CMAKE_CURRENT_SOURCE_DIR}")

target_link_libraries(renderer PUBLIC color ray rt vec3)
