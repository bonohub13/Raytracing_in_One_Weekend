#ifndef RTIOW_MATERIAL_LAMBERTIAN_H
#define RTIOW_MATERIAL_LAMBERTIAN_H

#include "material.h"
#include "vec3.h"

typedef struct lambertian {
    st_vec3_t albedo;
} st_lambertian_h;

extern st_material_t * create_lambertian(void);

#endif /* RTIOW_MATERIAL_LAMBERTIAN_H */
