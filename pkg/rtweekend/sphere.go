package rtweekend

import (
	"math"
)

type Sphere struct {
	center   Point3
	radius   float64
	material interface{}
}

func NewSphere(center Point3, radius float64, material Material) *Sphere {
	sphere := new(Sphere)

	sphere.center = center
	sphere.radius = radius
	sphere.material = material

	return sphere
}

func (s Sphere) Hit(r *Ray, t_min, t_max float64, rec *HitRecord) bool {
	oc := r.Origin().Substract(&s.center)
	a := r.Direction().LengthSquared()
	halfB := Dot(oc, &r.dir)
	c := oc.LengthSquared() - s.radius*s.radius
	discriminant := halfB*halfB - a*c

	setHitRecord := func(distance float64) {
		*rec = HitRecord{}
		rec.t = distance
		rec.p = *r.At(distance)
		normal := rec.p.Substract(&s.center).Divide(s.radius)
		rec.SetFaceNormal(r, normal)
		rec.material = s.material
	}

	if discriminant <= 0 {
		return false
	}

	sqrtd := math.Sqrt(discriminant)
	root := (-halfB - sqrtd) / a

	if (root > t_min) && (t_max > root) {
		setHitRecord(root)

		return true
	}

	root = (-halfB + sqrtd) / a

	if (root > t_min) && (t_max > root) {
		setHitRecord(root)

		return true
	}

	return false
}
