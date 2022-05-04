package main

import (
	"fmt"
	cam "github.com/bonohub13/Raytracing_in_One_Weekend/pkg/camera"
	hit "github.com/bonohub13/Raytracing_in_One_Weekend/pkg/hittable"
	mat "github.com/bonohub13/Raytracing_in_One_Weekend/pkg/material"
	rt "github.com/bonohub13/Raytracing_in_One_Weekend/pkg/rtweekend"
	tex "github.com/bonohub13/Raytracing_in_One_Weekend/pkg/texture"
	"github.com/bonohub13/Raytracing_in_One_Weekend/pkg/vec3"
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

func randomScene() *hit.HittableList {
	world := hit.NewHittableList()

	checker := tex.NewCheckerTextureFromColor(
		*vec3.NewColor(0.2, 0.3, 0.1),
		*vec3.NewColor(0.9, 0.9, 0.9),
	)
	materialGround := mat.NewLambertian(*checker)
	sphereGround := hit.NewSphere(*vec3.NewPoint3(0, -1000, 0), 1000, *materialGround)
	world.Add(*sphereGround)

	for a := -11; a < 11; a++ {
		for b := -11; b < 11; b++ {
			chooseMat := rt.RandomFloat64()
			center := vec3.NewPoint3(
				float64(a)+0.9*rt.RandomFloat64(),
				0.2,
				float64(b)+0.9*rt.RandomFloat64(),
			)

			if center.Substract(vec3.NewPoint3(4, 0.2, 0)).Length() > 0.9 {
				if chooseMat < 0.8 {
					// diffuse
					albedo := vec3.RandomColor().MultiplyVertices(vec3.RandomColor())
					material := mat.NewLambertianFromColor(*albedo)
					center2 := center.Add(
						vec3.NewVec3(0, rt.RandomFloat64InRange(0, 0.5), 0),
					)
					sphere := hit.NewMovingSphere(
						*center, *center2,
						0, 1,
						0.2,
						*material,
					)

					world.Add(*sphere)
				} else if chooseMat < 0.95 {
					// metal
					albedo := vec3.RandomColorInRange(0.5, 1)
					fuzz := rt.RandomFloat64InRange(0, 0.5)
					material := mat.NewMetal(*albedo, fuzz)
					sphere := hit.NewSphere(*center, 0.2, *material)

					world.Add(*sphere)
				} else {
					// glass
					material := mat.NewDialectric(1.5)
					sphere := hit.NewSphere(*center, 0.2, *material)

					world.Add(*sphere)
				}
			}
		}
	}
	material1 := mat.NewDialectric(1.5)
	material2 := mat.NewLambertianFromColor(*vec3.NewColor(0.4, 0.2, 0.1))
	material3 := mat.NewMetal(*vec3.NewColor(0.7, 0.6, 0.5), 0)

	sphere1 := hit.NewSphere(*vec3.NewPoint3(0, 1, 0), 1, *material1)
	sphere2 := hit.NewSphere(*vec3.NewPoint3(-4, 1, 0), 1, *material2)
	sphere3 := hit.NewSphere(*vec3.NewPoint3(4, 1, 0), 1, *material3)

	world.Add(*sphere1)
	world.Add(*sphere2)
	world.Add(*sphere3)

	return world
}

func sceneWithTwoSpheres() *hit.HittableList {
	objects := hit.NewHittableList()

	checker := tex.NewCheckerTextureFromColor(
		*vec3.NewColor(0.2, 0.3, 0.1),
		*vec3.NewColor(0.9, 0.9, 0.9),
	)
	material := mat.NewLambertian(*checker)
	sphereTop := hit.NewSphere(*vec3.NewPoint3(0, 10, 0), 10, *material)
	sphereBottom := hit.NewSphere(*vec3.NewPoint3(0, -10, 0), 10, *material)

	objects.Add(*sphereBottom)
	objects.Add(*sphereTop)

	return objects
}

func sceneWithTwoPerlinSpheres() *hit.HittableList {
	objects := hit.NewHittableList()

	pertext := tex.NewNoiseTexture(4)
	material := mat.NewLambertian(pertext)

	sphereGround := hit.NewSphere(*vec3.NewPoint3(0, -1000, 0), 1000, *material)
	sphere := hit.NewSphere(*vec3.NewPoint3(0, 2, 0), 2, *material)

	objects.Add(*sphereGround)
	objects.Add(*sphere)

	return objects
}

func imageImport() *hit.HittableList {
	hl := hit.NewHittableList()

	earthTexture, err := tex.LoadImageTexture("assets/earth.jpg")
	if err != nil {
		log.Fatalln(err.Error())
	}
	earthSurface := mat.NewLambertian(earthTexture)
	earth := hit.NewSphere(*vec3.NewPoint3(0, 0, 0), 2, earthSurface)

	hl.Add(*earth)

	return hl
}

func main() {
	// Initialize seed for random
	rt.InitRandom()

	// World
	world := new(hit.HittableList)

	// Camera
	lookFrom := new(vec3.Point3)
	lookAt := new(vec3.Point3)
	vfov := 40.0
	aperture := 0.0
	distToFocus := 10.0
	vup := vec3.NewVec3(0, 1, 0)

	switch v := 0; v {
	case 1:
		*world = *randomScene()
		*lookFrom = *vec3.NewPoint3(13, 2, 3)
		*lookAt = *vec3.NewPoint3(0, 0, 0)
		vfov = 20.0
		aperture = 0.1
	case 2:
		*world = *sceneWithTwoSpheres()
		*lookFrom = *vec3.NewPoint3(13, 2, 3)
		*lookAt = *vec3.NewPoint3(0, 0, 0)
		vfov = 20.0
	case 3:
		*world = *sceneWithTwoPerlinSpheres()
		*lookFrom = *vec3.NewPoint3(13, 2, 3)
		*lookAt = *vec3.NewPoint3(0, 0, 0)
		vfov = 20
	default:
		fallthrough
	case 4:
		*world = *imageImport()
		*lookFrom = *vec3.NewPoint3(13, 2, 3)
		*lookAt = *vec3.NewPoint3(0, 0, 0)
		vfov = 20
	}

	cam := cam.NewCamera(
		*lookFrom, *lookAt,
		*vup,
		vfov,
		ASPECT_RATIO,
		aperture,
		distToFocus,
		0, 1,
	)

	// Render
	fmt.Printf("P3\n%d %d\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT)

	for j := IMAGE_HEIGHT - 1; j >= 0; j-- {
		log.Printf("\rScanlines remaining: %d", j)
		for i := 0; i < IMAGE_WIDTH; i++ {
			pixelColor := vec3.NewColor(0, 0, 0)
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
