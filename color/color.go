package color

import (
	"fmt"
	"github.com/bonohub13/Raytracing_in_One_Weekend/vec3"
)

func WriteColor(pixelColor *vec3.Color) {
	fmt.Printf("%d %d %d\n",
		int(255.999*pixelColor.X()),
		int(255.999*pixelColor.Y()),
		int(255.999*pixelColor.Z()))
}
