cmake_policy(SET CMP0076 NEW)

add_subdirectory(color)
add_subdirectory(ray)
add_subdirectory(renderer)
add_subdirectory(vec3)

add_library(rt rt.c)
target_sources(rt PUBLIC rt.h)
target_include_directories(rt INTERFACE ${CMAKE_CURRENT_SOURCE_DIR})

target_link_libraries(rt m ray vec3)
