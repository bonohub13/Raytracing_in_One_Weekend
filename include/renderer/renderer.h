#ifndef RENDERER_H
#define RENDERER_H

#include "../vec3/vec3.h"

void format_ppm(int image_width, int image_height);
void render(int image_width, int image_height,
            const Vec3 *vertical, const Vec3 *horizontal,
            const Vec3 *lower_left_corner, const Point3 *origin);

#endif //RENDERER_H
