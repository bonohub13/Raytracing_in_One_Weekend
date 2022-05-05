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

func RayColor(r *Ray, background *Color, world Hittable, depth int) *Color {
	rec := new(HitRecord)

	if depth <= 0 {
		return NewColor(0, 0, 0)
	}

	if world.Hit(r, 0.001, math.Inf(1), rec) {
		scattered := Ray{}
		attenuation := Color{}
        emitted := rec.material.(Material).Emitted(rec.u, rec.v, &rec.p)

		if rec.material.(Material).Scatter(r, rec, &attenuation, &scattered) {
            return emitted.Add(
                attenuation.MultiplyVertices(
                    RayColor(&scattered, background, world, depth-1),
                ),
            )
        } else {
            return emitted
        }
    } else {
        return background
    }
}
