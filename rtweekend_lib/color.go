package rtweekendlib

import (
	"fmt"
)

func WriteColor(pixelColor *Color) {
	fmt.Printf("%d %d %d\n",
		int(255.999*pixelColor.X()),
		int(255.999*pixelColor.Y()),
		int(255.999*pixelColor.Z()))
}
