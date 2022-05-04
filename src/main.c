#include <stdio.h>
#include "../include/rt.h"
#include "../include/color/color.h"
#include "../include/ray/ray.h"
#include "../include/vec3/vec3.h"

// consts
const double ASPECT_RATIO = 16.0 / 9.0;
const int IMAGE_WIDTH = 256;
const int IMAGE_HEIGHT = (int)(IMAGE_WIDTH / ASPECT_RATIO);

int main() {
    int i, j; // for loops
    double u, v;

    // Camera
    double viewport_height = 2.0f;
    double viewport_width = ASPECT_RATIO * viewport_height;
    double focal_length = 1.0;
    Point3 origin = vec3(0, 0, 0);
    Vec3 horizontal = vec3(viewport_width, 0, 0);
    Vec3 vertical = vec3(0, viewport_height, 0);
    Vec3 half_h = divVec3(&horizontal, 2);
    Vec3 half_v = divVec3(&vertical, 2);
    Vec3 _fl = vec3(0, 0, focal_length);
    Vec3 lower_left_corner = substractVec3(&origin, &half_h);

    substractFromVec3(&lower_left_corner, &half_v);
    substractFromVec3(&lower_left_corner, &_fl);

    printf("P3\n%d %d\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for (j = IMAGE_HEIGHT-1; j >= 0; --j) {
        // Progress
        fprintf(stderr, "\rScanlines remaining: %d ", j);
        fflush(stderr);
        for (i = 0; i < IMAGE_WIDTH; ++i) {
            u = (double)i / (IMAGE_WIDTH-1);
            v = (double)j / (IMAGE_HEIGHT-1);
            Vec3 _v = multVec3(&vertical, v);
            Vec3 _h = multVec3(&horizontal, u);
            Vec3 r_dir = addVec3(&lower_left_corner, &_h);
            addToVec3(&r_dir, &_v);
            substractFromVec3(&r_dir, &origin);
            Ray r = ray(&origin, &r_dir);
            Color pixel_color = ray_color(&r);

            write_color(&pixel_color);
        }
    }

    fprintf(stderr, "\nDone!\n");
}
