package rtweekendlib

import (
	"fmt"
	"math"
)

const (
	INFINITY = math.MaxFloat64
	PI       = 3.1415926535897932385
)

func DegreesToRadians(degrees float64) float64 {
	return degrees * PI / 180
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

func RayColor(r *Ray, world Hittable) *Color {
	rec := new(HitRecord)
	c0 := rec.normal.Add(NewColor(1.0, 1.0, 1.0))

	if world.(Hittable).Hit(r, 0, math.Inf(1), rec) {
		fmt.Println("Here!")
		return c0.Multiply(0.5)
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
