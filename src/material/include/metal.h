#ifndef RTIOW_MATERIAL_METAL_H
#define RTIOW_MATERIAL_METAL_H

#include "vec3.h"
#include "material.h"

typedef struct metal {
    st_vec3_t albedo;
} st_metal_t;

extern st_material_t * create_metal(void);

#endif /* RTIOW_MATERIAL_METAL_H */
