#ifndef RTIOW_PARAMS_H
#define RTIOW_PARAMS_H

#include "hittable.h"
#include "sphere.h"
#include "lambertian.h"
#include "metal.h"
#include "dielectric.h"

// World info
//  Hittable objects
#define SPHERE_COUNT            (484 + 4)
#define SPHERE_SIZE             (SINGLE_SPHERE_SIZE * (SPHERE_COUNT + 1))
#define HITTABLE_COUNT          (SPHERE_COUNT)
#define HITTABLE_DATA_SIZE      (HITTABLE_DATA_SINGLE_SIZE * HITTABLE_COUNT)
#define HITTABLE_OBJ_SIZE       (HITTABLE_OBJ_SINGLE_SIZE * HITTABLE_COUNT)
//  Material objects
#define LAMBERTIAN_COUNT        (484 + 2)
#define LAMBERTIAN_SIZE         (LAMBERTIAN_SINGLE_SIZE * LAMBERTIAN_COUNT)
#define METAL_COUNT             (484 + 1)
#define METAL_SIZE              (METAL_SINGLE_SIZE * METAL_COUNT)
#define DIELECTRIC_COUNT        (484 + 2)
#define DIELECTRIC_SIZE         (DIELECTRIC_SINGLE_SIZE * DIELECTRIC_COUNT)
#define MATERIAL_COUNT          ( \
                                    LAMBERTIAN_COUNT \
                                  + METAL_COUNT \
                                  + DIELECTRIC_COUNT \
                                )
#define WORLD_SIZE              ( \
                                    SPHERE_SIZE \
                                  + LAMBERTIAN_SIZE \
                                  + METAL_SIZE \
                                  + DIELECTRIC_SIZE \
                                  + HITTABLE_DATA_SIZE \
                                  + HITTABLE_OBJ_SIZE \
                                )
#define SPHERE_OFFSET           (0)
#define LAMBERTIAN_OFFSET       (SPHERE_OFFSET + SPHERE_SIZE)
#define METAL_OFFSET            (LAMBERTIAN_OFFSET + LAMBERTIAN_SIZE)
#define DIELECTRIC_OFFSET       (METAL_OFFSET + METAL_SIZE)
#define HITTABLE_DATA_OFFSET    (DIELECTRIC_OFFSET + DIELECTRIC_SIZE)
#define HITTABLE_OBJ_OFFSET     (HITTABLE_DATA_OFFSET + HITTABLE_DATA_SIZE)

#endif /*RTIOW_PARAMS_H  */
