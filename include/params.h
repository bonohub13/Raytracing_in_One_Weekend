#ifndef RTIOW_PARAMS_H
#define RTIOW_PARAMS_H

#include "rtweekend.h"

// World info
#define SPHERE_COUNT            (2)
//  Size has to be aligned by 0x20 bytes
#define SINGLE_SPHERE_SIZE      (round_up(sizeof(st_sphere_t), 0x20))
#define SPHERE_SIZE             (SINGLE_SPHERE_SIZE * SPHERE_COUNT)
#define HITTABLE_COUNT          (SPHERE_COUNT)
#define HITTABLE_DATA_SIZE      (sizeof(void*) * HITTABLE_COUNT)
#define HITTABLE_OBJ_SIZE       (sizeof(st_hittable_t*) * HITTABLE_COUNT)
#define WORLD_SIZE              ( \
                                    SPHERE_SIZE \
                                  + HITTABLE_DATA_SIZE \
                                  + HITTABLE_OBJ_SIZE \
                                )
#define SPHERE_OFFSET           (0)
#define HITTABLE_DATA_OFFSET    (SPHERE_SIZE)
#define HITTABLE_OBJ_OFFSET     (HITTABLE_DATA_OFFSET + HITTABLE_DATA_SIZE)

#endif /*RTIOW_PARAMS_H  */
