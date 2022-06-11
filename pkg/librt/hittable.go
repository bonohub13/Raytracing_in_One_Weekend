package librt

import (
	"math"
)

type Hittable interface {
	Hit(r *Ray, t_min float64, t_max float64) (bool, *HitRecord)
}

type Sphere struct {
	center Point3
	radius float64
}

func NewSphere(center Point3, radius float64) *Sphere {
	s := &Sphere{}

	s.center = center
	s.radius = radius

	return s
}

func (s Sphere) Hit(r *Ray, t_min float64, t_max float64) (bool, *HitRecord) {
	oc := r.origin.Substract(&s.center)
	a := r.direction.LengthSquared()
	half_b := Dot(oc, &r.direction)
	c := oc.LengthSquared() - s.radius*s.radius
	discriminant := half_b*half_b - a*c

	if discriminant >= 0 {
		sqrtd := math.Sqrt(discriminant)
		root := (-half_b - sqrtd) / a

		if root >= t_min && t_max >= root {
			hit := &HitRecord{}

			hit.t = root
			hit.p = *r.At(root)
			outwardNormal := hit.p.Substract(&s.center).DivFloat64(s.radius)
			hit.SetFaceNormal(r, outwardNormal)

			return true, hit
		}
		root = (-half_b + sqrtd) / a

		if root >= t_min && t_max >= root {
			hit := &HitRecord{}

			hit.t = root
			hit.p = *r.At(root)
			outwardNormal := hit.p.Substract(&s.center).DivFloat64(s.radius)
			hit.SetFaceNormal(r, outwardNormal)

			return true, hit
		}
	}

	return false, nil
}

type HittableList struct {
	objects []Hittable
}

func NewHittableList() *HittableList {
	hl := new(HittableList)

	hl.Clear()

	return hl
}

func (hl *HittableList) Clear() {
	hl.objects = make([]Hittable, 0)
}

func (hl *HittableList) Add(object Hittable) {
	hl.objects = append(hl.objects, object)
}

func (hl HittableList) Hit(r *Ray, t_min float64, t_max float64) (bool, *HitRecord) {
	hr := &HitRecord{}
	hitAnything := false
	closestSoFar := t_max

	for _, object := range hl.objects {
		if hitAnythingTmp, hit := object.Hit(r, t_min, closestSoFar); hitAnythingTmp {
			hitAnything = hitAnythingTmp
			closestSoFar = hit.t
			hr = hit
		}
	}

	return hitAnything, hr
}
