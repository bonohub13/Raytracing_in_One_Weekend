add_library(rt
  SHARED
    rt.c
    color/color.c
    ray/ray.c
    renderer/renderer.c
    vec3/vec3.c
)

target_link_libraries(rt m)

target_include_directories(rt
  PUBLIC ${PROJECT_SOURCE_DIR}/include
  PUBLIC ${PROJECT_SOURCE_DIR}/include/color
  PUBLIC ${PROJECT_SOURCE_DIR}/include/ray
  PUBLIC ${PROJECT_SOURCE_DIR}/include/renderer
  PUBLIC ${PROJECT_SOURCE_DIR}/include/vec3
)
