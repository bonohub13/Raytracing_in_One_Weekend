#include <stdlib.h>
#include <stdint.h>

#include "scenes.h"
#include "params.h"

int32_t main(void) {
    // World
    uint8_t * p_memory = (uint8_t*)aligned_alloc(
            SPHERE_ALIGNMENT,
            WORLD_SIZE);

    wide_angle(p_memory);

    free(p_memory);

    return 0;
}
