#include <stdio.h>
#include "../include/rt.h"
#include "../include/color/color.h"
#include "../include/ray/ray.h"
#include "../include/renderer/renderer.h"
#include "../include/vec3/vec3.h"

// consts
const double ASPECT_RATIO = 16.0 / 9.0;
const int IMAGE_WIDTH = 256;
const int IMAGE_HEIGHT = (int)(IMAGE_WIDTH / ASPECT_RATIO);

int main() {
    // Camera
    double viewport_height = 2.0f;
    double viewport_width = ASPECT_RATIO * viewport_height;
    double focal_length = 1.0;
    Point3 origin = point3(0, 0, 0);
    Vec3 horizontal = vec3(viewport_width, 0, 0);
    Vec3 vertical = vec3(0, viewport_height, 0);
    Vec3 half_h = divVec3(&horizontal, 2);
    Vec3 half_v = divVec3(&vertical, 2);
    Vec3 _fl = vec3(0, 0, focal_length);
    Vec3 lower_left_corner = substractVec3(&origin, &half_h);

    substractFromVec3(&lower_left_corner, &half_v);
    substractFromVec3(&lower_left_corner, &_fl);

    format_ppm(IMAGE_WIDTH, IMAGE_HEIGHT);

    render(IMAGE_WIDTH, IMAGE_HEIGHT,
            &vertical, &horizontal,
            &lower_left_corner, &origin);

    fprintf(stderr, "\nDone!\n");
}
