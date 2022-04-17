package rtweekendlib

import (
	"fmt"
	"math"
)

type Sphere struct {
	center Point3
	radius float64
}

func (s Sphere) Hit(r *Ray, t_min, t_max float64, rec *HitRecord) bool {
	oc := r.Origin().Substract(&s.center)
	a := r.Direction().LengthSquared()
	halfB := Dot(oc, r.Direction())
	c := oc.LengthSquared() - s.radius*s.radius
	discriminant := halfB*halfB - a*c

	if discriminant < 0 {
		return false
	}

	sqrtd := math.Sqrt(discriminant)
	root := (-halfB - sqrtd) / a

	fmt.Printf("root: %v\n", sqrtd)

	if (root < t_min) || (t_max < root) {
		root = (-halfB + sqrtd) / a
		if (root < t_min) || (t_max < root) {
			return false
		}
	}

	rec.SetT(root)
	rec.SetP(r.At(rec.T()))

	outwardNormal := rec.P().Substract(&s.center)
	outwardNormal.DivToThis(s.radius)
	rec.SetFaceNormal(r, outwardNormal)

	return true
}

func NewSphere(center *Point3, radius float64) *Sphere {
	sphere := new(Sphere)

	sphere.center = *center
	sphere.radius = radius

	return sphere
}
