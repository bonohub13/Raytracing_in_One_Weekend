#include <stdio.h>
#include "renderer.h"
#include "../rt.h"
#include "../color/color.h"
#include "../ray/ray.h"

void format_ppm(int image_width, int image_height) {
    printf("P3\n%d %d\n255\n", image_width, image_height);
}

void render(
        int image_width, int image_height,
        const Vec3 *vertical, const Vec3 *horizontal,
        const Vec3 *lower_left_corner, const Point3 *origin
) {
    int i, j;
    double u, v;

    for (j=image_height-1; j>=0; --j) {
        // Progress
        fprintf(stderr, "\rScanlines remaining: %d ", j);
        fflush(stderr);
        for (i=0; i<image_width; ++i) {
            u = (double)i / (image_width-1);
            v = (double)j / (image_height-1);
            Vec3 tmp_v = multVec3(vertical, v);
            Vec3 tmp_h = multVec3(horizontal, u);
            Vec3 r_dir = addVec3(lower_left_corner, &tmp_h);
            addToVec3(&r_dir, &tmp_v);
            substractFromVec3(&r_dir, origin);

            Ray r = ray(origin, &r_dir);
            Color pixel_color = ray_color(&r);
            
            write_color(&pixel_color);
        }
    }
}
