#include "rtweekend.h"

double degrees_to_radians(double degrees) {
    return degrees * PI / 180.0;
}

size_t round_up(size_t val, size_t div) {
    return ((val + div - 1) / div) * div;
}
