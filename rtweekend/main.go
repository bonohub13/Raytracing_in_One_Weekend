package main

import (
	"fmt"
	"github.com/bonohub13/Raytracing_in_One_Weekend/color"
	"github.com/bonohub13/Raytracing_in_One_Weekend/common"
	"github.com/bonohub13/Raytracing_in_One_Weekend/ray"
	"github.com/bonohub13/Raytracing_in_One_Weekend/vec3"
)

// 定数
const (
	// Image
	ASPECT_RATIO = 16.0 / 9.0
	IMAGE_WIDTH  = 400
	IMAGE_HEIGHT = int(IMAGE_WIDTH / ASPECT_RATIO)
)

func main() {
	// Camera
	viewportHeight := 2.0
	viewportWidth := ASPECT_RATIO * viewportHeight
	focalLength := 1.0

	origin := vec3.NewPoint3(0, 0, 0)
	horizontal := vec3.NewVec3(viewportWidth, 0, 0)
	vertical := vec3.NewVec3(0, viewportHeight, 0)
	focalLengthVec3 := vec3.NewVec3(0, 0, focalLength)
	lowerLeftCorner := origin.Substract(
		horizontal.Divide(2)).Substract(
		vertical.Divide(2)).Substract(
		focalLengthVec3)

	// Render
	fmt.Printf("P3\n%d %d\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT)

	for j := IMAGE_HEIGHT - 1; j >= 0; j-- {
		for i := 0; i < IMAGE_WIDTH; i++ {
			u := float64(i) / float64(IMAGE_WIDTH-1)
			v := float64(j) / float64(IMAGE_HEIGHT-1)
			r := ray.NewRay(
				origin,
				lowerLeftCorner.Add(
					horizontal.Multiply(u).Add(
						vertical.Multiply(v).Substract(origin))))
			pixelColor := common.RayColor(r)

			color.WriteColor(pixelColor)
		}
	}
}
