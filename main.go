package main

import (
	rt "github.com/bonohub13/Raytracing_in_One_Weekend/tree/golang/pkg/librt"
	"os"
)

const (
	ASPECT_RATIO      = 16.0 / 9.0
	IMAGE_WIDTH       = 400
	IMAGE_HEIGHT      = int(IMAGE_WIDTH / ASPECT_RATIO)
	SAMPLES_PER_PIXEL = 100
	MAX_DEPTH         = 50
)

func main() {
	filename := os.Args[1]
	rt.InitRandom()
	// World
	world := rt.NewHittableList()

	world.Add(*rt.NewSphere(*rt.NewPoint3(0, 0, -1), 0.5))
	world.Add(*rt.NewSphere(*rt.NewPoint3(0, -100.5, -1), 100))

	// Camera
	cam := rt.NewCamera()

	renderer := rt.NewRenderer(
		IMAGE_WIDTH,
		IMAGE_HEIGHT,
		SAMPLES_PER_PIXEL,
		MAX_DEPTH,
		filename,
		cam,
		world)

	renderer.Render()
}
