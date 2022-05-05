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

func randomScene() *rt.HittableList {
	world := rt.NewHittableList()

	checker := rt.NewCheckerTextureFromColor(
		*rt.NewColor(0.2, 0.3, 0.1),
		*rt.NewColor(0.9, 0.9, 0.9),
	)
	materialGround := rt.NewLambertian(*checker)
	sphereGround := rt.NewSphere(
        *rt.NewPoint3(0, -1000, 0),
        1000,
        *materialGround,
    )
	world.Add(*sphereGround)

	for a := -11; a < 11; a++ {
		for b := -11; b < 11; b++ {
			chooseMat := rt.RandomFloat64()
			center := rt.NewPoint3(
				float64(a)+0.9*rt.RandomFloat64(),
				0.2,
				float64(b)+0.9*rt.RandomFloat64(),
			)

			if center.Substract(rt.NewPoint3(4, 0.2, 0)).Length() > 0.9 {
				if chooseMat < 0.8 {
					// diffuse
					albedo := rt.RandomColor().MultiplyVertices(
                        rt.RandomColor(),
                    )
					material := rt.NewLambertianFromColor(*albedo)
					center2 := center.Add(
						rt.NewVec3(0, rt.RandomFloat64InRange(0, 0.5), 0),
					)
					sphere := rt.NewMovingSphere(
						*center, *center2,
						0, 1,
						0.2,
						*material,
					)

					world.Add(*sphere)
				} else if chooseMat < 0.95 {
					// metal
					albedo := rt.RandomColorInRange(0.5, 1)
					fuzz := rt.RandomFloat64InRange(0, 0.5)
					material := rt.NewMetal(*albedo, fuzz)
					sphere := rt.NewSphere(*center, 0.2, *material)

					world.Add(*sphere)
				} else {
					// glass
					material := rt.NewDialectric(1.5)
					sphere := rt.NewSphere(*center, 0.2, *material)

					world.Add(*sphere)
				}
			}
		}
	}
	material1 := rt.NewDialectric(1.5)
	material2 := rt.NewLambertianFromColor(*rt.NewColor(0.4, 0.2, 0.1))
	material3 := rt.NewMetal(*rt.NewColor(0.7, 0.6, 0.5), 0)

	sphere1 := rt.NewSphere(*rt.NewPoint3(0, 1, 0), 1, *material1)
	sphere2 := rt.NewSphere(*rt.NewPoint3(-4, 1, 0), 1, *material2)
	sphere3 := rt.NewSphere(*rt.NewPoint3(4, 1, 0), 1, *material3)

	world.Add(*sphere1)
	world.Add(*sphere2)
	world.Add(*sphere3)

	return world
}

func sceneWithTwoSpheres() *rt.HittableList {
	objects := rt.NewHittableList()

	checker := rt.NewCheckerTextureFromColor(
		*rt.NewColor(0.2, 0.3, 0.1),
		*rt.NewColor(0.9, 0.9, 0.9),
	)
	material := rt.NewLambertian(*checker)
	sphereTop := rt.NewSphere(*rt.NewPoint3(0, 10, 0), 10, *material)
	sphereBottom := rt.NewSphere(*rt.NewPoint3(0, -10, 0), 10, *material)

	objects.Add(*sphereBottom)
	objects.Add(*sphereTop)

	return objects
}

func sceneWithTwoPerlinSpheres() *rt.HittableList {
	objects := rt.NewHittableList()

	pertext := rt.NewNoiseTexture(4)
	material := rt.NewLambertian(pertext)

	sphereGround := rt.NewSphere(*rt.NewPoint3(0, -1000, 0), 1000, *material)
	sphere := rt.NewSphere(*rt.NewPoint3(0, 2, 0), 2, *material)

	objects.Add(*sphereGround)
	objects.Add(*sphere)

	return objects
}

func imageImport() *rt.HittableList {
	hl := rt.NewHittableList()

	earthTexture, err := rt.LoadImageTexture("assets/earth.jpg")
	if err != nil {
		log.Fatalln(err.Error())
	}
	earthSurface := rt.NewLambertian(earthTexture)
	earth := rt.NewSphere(*rt.NewPoint3(0, 0, 0), 2, earthSurface)

	hl.Add(*earth)

	return hl
}

func main() {
	// Initialize seed for random
	rt.InitRandom()

	// World
	world := new(rt.HittableList)

	// Camera
	lookFrom := new(rt.Point3)
	lookAt := new(rt.Point3)
	vfov := 40.0
	aperture := 0.0
    background := rt.NewColor(0, 0, 0)
	distToFocus := 10.0
	vup := rt.NewVec3(0, 1, 0)

	switch v := 0; v {
	case 1:
		*world = *randomScene()
		*lookFrom = *rt.NewPoint3(13, 2, 3)
		*lookAt = *rt.NewPoint3(0, 0, 0)
		vfov = 20.0
		aperture = 0.1
        background = rt.NewColor(0.7, 0.8, 1)
	case 2:
		*world = *sceneWithTwoSpheres()
        background = rt.NewColor(0.7, 0.8, 1)
		*lookFrom = *rt.NewPoint3(13, 2, 3)
		*lookAt = *rt.NewPoint3(0, 0, 0)
		vfov = 20.0

	case 3:
		*world = *sceneWithTwoPerlinSpheres()
        background = rt.NewColor(0.7, 0.8, 1)
		*lookFrom = *rt.NewPoint3(13, 2, 3)
		*lookAt = *rt.NewPoint3(0, 0, 0)
		vfov = 20
	case 4:
		*world = *imageImport()
        background = rt.NewColor(0.7, 0.8, 1)
		*lookFrom = *rt.NewPoint3(13, 2, 3)
		*lookAt = *rt.NewPoint3(0, 0, 0)
		vfov = 20
	default:
		fallthrough
    case 5:
        break
	}

	cam := rt.NewCamera(
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
			pixelColor := rt.NewColor(0, 0, 0)
			for s := 0; s < SAMPLES_PER_PIXEL; s++ {
				u := (float64(i) + rt.RandomFloat64()) / float64(IMAGE_WIDTH-1)
				v := (float64(j) + rt.RandomFloat64()) / float64(IMAGE_HEIGHT-1)
				r := cam.GetRay(u, v)

				pixelColor.AddToThis(
                    rt.RayColor(r, background, world, MAX_DEPTH),
                )
			}

			rt.WriteColor(pixelColor, SAMPLES_PER_PIXEL)
		}
	}
	log.Println("\nDone.")
}
