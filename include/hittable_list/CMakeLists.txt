cmake_policy(SET CMP0076 NEW)

add_library(hittable_list hittable_list.c)
target_sources(hittable_list PUBLIC hittable_list.h)
target_include_directories(hittable_list INTERFACE "${CMAKE_CURRENT_SOURCE_DIR}")

target_link_libraries(hittable_list PUBLIC sphere)
