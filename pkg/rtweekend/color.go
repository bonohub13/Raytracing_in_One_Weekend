package rtweekend

import (
	"fmt"
	"math"
)

func WriteColor(pixelColor *Color, samplesPerPixel int) {
	r := pixelColor.X()
	g := pixelColor.Y()
	b := pixelColor.Z()
	scale := 1.0 / float64(samplesPerPixel)

	r = math.Sqrt(scale * r)
	g = math.Sqrt(scale * g)
	b = math.Sqrt(scale * b)

	fmt.Printf("%d %d %d\n",
		int(256*Clamp(r, 0, 0.999)),
		int(256*Clamp(g, 0, 0.999)),
		int(256*Clamp(b, 0, 0.999)),
	)
}
