package main

import (
	"fmt"
	rt "github.com/bonohub13/Raytracing_in_One_Weekend/pkg/rtweekend"
	"log"
)

// Image
const (
	ASPECT_RATIO      = 16.0 / 9.0
	IMAGE_WIDTH       = 400
	IMAGE_HEIGHT      = int(IMAGE_WIDTH / ASPECT_RATIO)
	SAMPLES_PER_PIXEL = 100
	MAX_DEPTH         = 50
)

func main() {
	rt.InitRandom()

	// World
	world := new(rt.HittableList)

	materialGround := rt.NewLambertian(*rt.NewColor(0.8, 0.8, 0))
	materialCenter := rt.NewLambertian(*rt.NewColor(0.7, 0.3, 0.3))
	materialLeft := rt.NewMetal(*rt.NewColor(0.8, 0.8, 0.8))
	materialRight := rt.NewMetal(*rt.NewColor(0.8, 0.6, 0.2))

	sphereGround := rt.NewSphere(
		*rt.NewPoint3(0, -100.5, -1),
		100,
		*materialGround,
	)
	sphereCenter := rt.NewSphere(
		*rt.NewPoint3(0, 0, -1),
		0.5,
		*materialCenter,
	)
	sphereLeft := rt.NewSphere(
		*rt.NewPoint3(-1.0, 0, -1),
		0.5,
		*materialLeft,
	)
	sphereRight := rt.NewSphere(
		*rt.NewPoint3(1.0, 0, -1),
		0.5,
		materialRight,
	)

	world.Add(*sphereGround)
	world.Add(*sphereCenter)
	world.Add(*sphereLeft)
	world.Add(*sphereRight)

	// Camera
	cam := rt.NewCamera()

	// Render
	fmt.Printf("P3\n%d %d\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT)

	for j := IMAGE_HEIGHT - 1; j >= 0; j-- {
		log.Printf("\rScanlines remaining: %d", j)
		for i := 0; i < IMAGE_WIDTH; i++ {
			pixelColor := rt.NewColor(0, 0, 0)
			for s := 0; s < SAMPLES_PER_PIXEL; s++ {
				u := (float64(i) + rt.RandomFloat64()) / float64(IMAGE_WIDTH-1)
				v := (float64(j) + rt.RandomFloat64()) / float64(IMAGE_HEIGHT-1)
				r := cam.GetRay(u, v)

				pixelColor.AddToThis(rt.RayColor(r, world, MAX_DEPTH))
			}

			rt.WriteColor(pixelColor, SAMPLES_PER_PIXEL)
		}
	}
	log.Println("\nDone.")
}
