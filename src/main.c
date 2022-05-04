#include <stdio.h>

// consts
const int IMAGE_WIDTH = 256;
const int IMAGE_HEIGHT = 256;

int main() {
    int i, j; // for loops
    double r, g, b;
    int ir, ig, ib;

    printf("P3\n%d %d\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    b = 0.25;
    ib = (int)(255.999*b);

    for (j = IMAGE_HEIGHT-1; j >= 0; --j) {
        for (i = 0; i < IMAGE_WIDTH; ++i) {
            r = (double)i / (double)(IMAGE_WIDTH-1);
            g = (double)i / (double)(IMAGE_HEIGHT-1);

            ir = (int)(255.999 * r);
            ig = (int)(255.999 * g);

            printf("%d %d %d\n", ir, ig, ib);
        }
    }
}
