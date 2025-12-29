#include <math.h>

#include "interval.h"

const st_interval_t g_empty = {
    .min = HUGE_VAL,
    .max = -HUGE_VAL,
};

const st_interval_t g_universe = {
    .min = -HUGE_VAL,
    .max = HUGE_VAL,
};

double interval_size(const st_interval_t * const p_interval) {
    return p_interval->max - p_interval->min;
}

bool interval_contains(const st_interval_t * const p_interval, const double x) {
    return (p_interval->min <= x) && (x <= p_interval->max);
}

bool interval_surrounds(const st_interval_t * const p_interval, const double x) {
    return (p_interval->min < x) && (x < p_interval->max);
}

double interval_clamp(const st_interval_t * const p_interval, const double x) {
    if (x < p_interval->min) {
        return p_interval->min;
    } else if (p_interval->max < x) {
        return p_interval->max;
    } else {
        return x;
    }
}
