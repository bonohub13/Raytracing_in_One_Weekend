#include <stdio.h>
#include <stdint.h>
#include <math.h>

#include "color.h"
#include "interval.h"

static inline double linear_to_gamma(const double linear_component) {
    if (linear_component > 0) {
        return sqrt(linear_component);
    }

    return 0;
}

void write_color(const st_vec3_t * const p_pixel_color) {
    static const st_interval_t s_intensity = {
        .min = 0.000,
        .max = 0.999,
    };
    double rgb[3] = {
        linear_to_gamma(p_pixel_color->e[0]),
        linear_to_gamma(p_pixel_color->e[1]),
        linear_to_gamma(p_pixel_color->e[2]),
    };
    int32_t rgb_byte[3] = {
        (int32_t)(256 * interval_clamp(&s_intensity, rgb[0])),
        (int32_t)(256 * interval_clamp(&s_intensity, rgb[1])),
        (int32_t)(256 * interval_clamp(&s_intensity, rgb[2])),
    };

    printf("%d %d %d\n", rgb_byte[0], rgb_byte[1], rgb_byte[2]);

    return;
}
