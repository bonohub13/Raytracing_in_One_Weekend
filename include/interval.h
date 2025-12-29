#ifndef RTIOW_INTERVAL_H
#define RTIOW_INTERVAL_H

#include <stdbool.h>

typedef struct interval {
    double min;
    double max;
} st_interval_t;

extern const st_interval_t g_empty;
extern const st_interval_t g_universe;

extern double interval_size(const st_interval_t * const p_interval);
extern bool interval_contains(const st_interval_t * const p_interval, const double x);
extern bool interval_surrounds(const st_interval_t * const p_interval, const double x);
extern double interval_clamp(const st_interval_t * const p_interval, const double x);

#endif /* RTIOW_INTERVAL_H */
