package main

import (
	"fmt"
	rt "github.com/bonohub13/Raytracing_in_One_Weekend/rtweekend_lib"
)

// Image
const (
	ASPECT_RATIO = 16.0 / 9.0
	IMAGE_WIDTH  = 400
	IMAGE_HEIGHT = int(IMAGE_WIDTH / ASPECT_RATIO)
)

func main() {
	// World
	world := new(rt.HittableList)
	world.Add(*rt.NewSphere(rt.NewPoint3(0, 0, -1), 0.5))
	world.Add(*rt.NewSphere(rt.NewPoint3(0, -100.5, -1), 100))

	// Camera
	viewportHeight := 2.0
	viewportWidth := ASPECT_RATIO * viewportHeight
	focalLength := 1.0

	origin := rt.NewPoint3(0, 0, 0)
	horizontal := rt.NewVec3(viewportWidth, 0, 0)
	vertical := rt.NewVec3(0, viewportHeight, 0)
	focalLengthVec3 := rt.NewVec3(0, 0, focalLength)
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
			r := rt.NewRay(
				origin,
				lowerLeftCorner.Add(
					horizontal.Multiply(u)).Add(
					vertical.Multiply(v)))
			pixelColor := rt.RayColor(r, world)

			rt.WriteColor(pixelColor)
		}
	}
}
