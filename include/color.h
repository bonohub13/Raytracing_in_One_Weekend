#ifndef RTIOW_COLOR_H
#define RTIOW_COLOR_H

#include <stdint.h>

#include "vec3.h"

extern void write_color(const st_vec3_t * const p_pixel_color,
        uint32_t * const p_buffer);

#endif /* RTIOW_COLOR_H */
