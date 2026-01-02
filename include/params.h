#ifndef RTIOW_PARAMS_H
#define RTIOW_PARAMS_H

#include "rtweekend.h"

// World info
//  Hittable objects
#define SPHERE_COUNT            (5)
#define SPHERE_ALIGNMENT        (0x20)
//      Size has to be aligned by 0x20 bytes
#define SINGLE_SPHERE_SIZE      (round_up(sizeof(st_sphere_t), SPHERE_ALIGNMENT))
#define SPHERE_SIZE             (SINGLE_SPHERE_SIZE * (SPHERE_COUNT + 1))
#define HITTABLE_COUNT          (SPHERE_COUNT)
#define HITTABLE_DATA_SIZE      (sizeof(void*) * HITTABLE_COUNT)
#define HITTABLE_OBJ_SIZE       (sizeof(st_hittable_t*) * HITTABLE_COUNT)
//  Material objects
#define LAMBERTIAN_COUNT        (2)
#define LAMBERTIAN_SIZE         (sizeof(st_lambertian_t) * LAMBERTIAN_COUNT)
#define METAL_COUNT             (1)
#define METAL_SIZE              (sizeof(st_metal_t) * METAL_COUNT)
#define DIELECTRIC_COUNT        (2)
#define DIELECTRIC_SIZE         (sizeof(st_dielectric_t) * DIELECTRIC_COUNT)
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
