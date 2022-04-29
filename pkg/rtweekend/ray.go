package rtweekend

import ()

type Ray struct {
	orig Point3
	dir  Vec3
	tm   float64
}

func NewRay(origin Point3, direction Vec3, time ...float64) *Ray {
	ray := new(Ray)

	if len(time) == 1 {
		ray.tm = time[0]
	} else {
		ray.tm = 0
	}
	ray.orig = origin
	ray.dir = direction

	return ray
}

func (r *Ray) Origin() *Point3  { return &r.orig }
func (r *Ray) Direction() *Vec3 { return &r.dir }
func (r *Ray) Time() float64    { return r.tm }

func (r *Ray) At(gain float64) *Point3 {
	return r.orig.Add(r.dir.Multiply(gain))
}
