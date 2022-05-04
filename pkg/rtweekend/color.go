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

func HitSphere(center *Point3, radius float64, r *Ray) float64 {
	oc := r.Origin().Substract(center)
	a := Dot(r.Direction(), r.Direction())
	halfB := Dot(oc, r.Direction())
	c := Dot(oc, oc) - radius*radius
	discriminant := halfB*halfB - a*c

	if discriminant < 0 {
		return -1
	} else {
		return (-halfB - math.Sqrt(discriminant)) / a
	}
}

func RayColor(r *Ray, world Hittable, depth int) *Color {
	rec := new(HitRecord)

	if depth <= 0 {
		return NewColor(0, 0, 0)
	}

	c0 := NewColor(1.0, 1.0, 1.0)

	if world.Hit(r, 0.001, math.Inf(1), rec) {
		scattered := Ray{}
		attenuation := Color{}

		if rec.material.(Material).Scatter(r, rec, &attenuation, &scattered) {
			return attenuation.MultiplyVertices(
				RayColor(&scattered, world, depth-1),
			)
		}

		return NewColor(0, 0, 0)
	}

	unitDirection := UnitVector(r.Direction())
	t := 0.5 * (unitDirection.Y() + 1.0)
	c1 := NewColor(0.5, 0.7, 1.0)
	// (1.0 - t) * c0
	c0.MultToThis(1.0 - t)
	// t * c1
	c1.MultToThis(t)

	return c0.Add(c1)
}
