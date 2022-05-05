add_library(vec3 vec3.c)
target_sources(vec3 PUBLIC vec3.h)
target_include_directories(vec3 INTERFACE "${CMAKE_CURRENT_SOURCE_DIR}")
