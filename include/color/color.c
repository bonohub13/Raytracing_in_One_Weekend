#include <stdio.h>
#include "color.h"

void write_color(const Color *pixel_color) {
    printf("%d %d %d\n",
        (int)(255.999 * x(pixel_color)),
        (int)(255.999 * y(pixel_color)),
        (int)(255.999 * z(pixel_color))
    );
}
