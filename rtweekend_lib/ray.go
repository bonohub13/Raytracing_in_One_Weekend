package rtweekendlib

import ()

type Ray struct {
	orig Point3
	dir  Vec3
}

func NewRay(origin *Point3, direction *Vec3) *Ray {
	ray := new(Ray)

	ray.orig = *origin
	ray.dir = *direction

	return ray
}

func (r *Ray) Origin() *Point3  { return &r.orig }
func (r *Ray) Direction() *Vec3 { return &r.dir }

func (r *Ray) At(gain float64) *Point3 {
	return r.orig.Add(r.dir.Multiply(gain))
}
