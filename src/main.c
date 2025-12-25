#include <stdio.h>
#include <stdint.h>

#include "params.h"
#include "vec3.h"
#include "color.h"

int32_t main(void) {
    int32_t ij = 0;
    st_vec3_t pixel_color = { 0 };

    printf("P3\n%d %d\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
    for (ij = 0; ij < IMAGE_WIDTH * IMAGE_HEIGHT; ij++) {
        pixel_color.e[0] = (double)(ij % IMAGE_WIDTH) / (double)(IMAGE_WIDTH - 1);
        pixel_color.e[1] = (double)(ij / IMAGE_WIDTH) / (double)(IMAGE_HEIGHT - 1);
        pixel_color.e[2] = 0.0;

        write_color(&pixel_color);
    }

    return 0;
}
