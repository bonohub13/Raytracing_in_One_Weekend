#include <stdio.h>
#include <stdint.h>

#include "params.h"

int32_t main(void) {
    int32_t ij = 0;
    int32_t rgb_i[3] = { 0 };
    double rgb_f[3] = { 0 };

    printf("P3\n%d %d\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
    for (ij = 0; ij < IMAGE_WIDTH * IMAGE_HEIGHT; ij++) {
        rgb_f[0] = (double)(ij % IMAGE_WIDTH) / (double)(IMAGE_WIDTH - 1);
        rgb_f[1] = (double)(ij / IMAGE_WIDTH) / (double)(IMAGE_HEIGHT - 1);
        rgb_f[2] = 0.0;

        rgb_i[0] = (int32_t)(255.999 * rgb_f[0]);
        rgb_i[1] = (int32_t)(255.999 * rgb_f[1]);
        rgb_i[2] = (int32_t)(255.999 * rgb_f[2]);

        printf("%d %d %d\n", rgb_i[0], rgb_i[1], rgb_i[2]);
    }

    return 0;
}
