#ifndef RTIOW_RTWEEKEND_H
#define RTIOW_RTWEEKEND_H

#include <stddef.h>

#define PI                  (3.1415926535897932385)

extern double degrees_to_radians(double degrees);
extern double random_double(void);
extern double random_double_in_range(double min, double max);
extern size_t round_up(size_t val, size_t div);

#endif /* RTIOW_RTWEEKEND_H */
