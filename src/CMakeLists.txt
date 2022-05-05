add_executable(rtweekend main.c)

# Added dependencies (headers and such)
target_link_libraries(rtweekend color ray renderer rt vec3)
