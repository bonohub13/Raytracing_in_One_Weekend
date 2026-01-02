#ifndef RTIOW_MATERIAL_DIELECTRIC_H
#define RTIOW_MATERIAL_DIELECTRIC_H

#include "material.h"

#define DIELECTRIC_SINGLE_SIZE      (sizeof(st_dielectric_t))

typedef struct dielectric {
    double refraction_index;
} st_dielectric_t;

extern st_material_t * create_dielectric(void);

#endif /* RTIOW_MATERIAL_DIELECTRIC_H */
