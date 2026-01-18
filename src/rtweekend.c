#include <stdlib.h>
#include <stdint.h>
#include <time.h>

#include "rtweekend.h"

double degrees_to_radians(double degrees) {
    static const double s_degrees_to_radians = PI / 180;

    return degrees * s_degrees_to_radians;
}

double random_double(void) {
    static uint32_t s_seed = 0;

    if (0 == s_seed) {
        s_seed = time(NULL);
        srand(s_seed);
    }

    return (double)rand() / ((double)RAND_MAX + 1.0);
}

double random_double_in_range(double min, double max) {
    return min + (max - min) * random_double();
}

size_t round_up(size_t val, size_t div) {
    return ((val + div - 1) / div) * div;
}
