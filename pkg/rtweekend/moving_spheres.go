package rtweekend

import (
	"math"
)

type MovingSphere struct {
	center0, center1 Point3
	time0, time1     float64
	radius           float64
	material         interface{}
}

/* Returns a pointer of MovingSphere object
   Params
       cen0    Point3      Center point of sphere at time t0
       cen1    Point3      Center point of sphere at time t1
       time0   float64     Point of time at t0
       time1   float64     Point of time at t1
       r       float64     Radius of sphere
       m       Material    Material of sphere
*/
func NewMovingSphere(
	cen0, cen1 Point3,
	time0, time1, r float64,
	m Material,
) *MovingSphere {
	ms := new(MovingSphere)

	ms.center0 = cen0
	ms.center1 = cen1
	ms.time0 = time0
	ms.time1 = time1
	ms.radius = r
	ms.material = m

	return ms
}

/* Returns a pointer of a data containing center coordinate at point of time t
   Params
       time float64    Point of time t
*/
func (ms *MovingSphere) Center(time float64) *Point3 {
	return ms.center0.Add(
		ms.center1.Substract(
			&ms.center0,
		).Multiply(
			(time - ms.time0) / (ms.time1 - ms.time0),
		),
	)
}

func (ms MovingSphere) Hit(r *Ray, t_min, t_max float64, rec *HitRecord) bool {
	oc := r.orig.Substract(ms.Center(r.tm))
	a := r.dir.LengthSquared()
	halfB := Dot(oc, &r.dir)
	c := oc.LengthSquared() - ms.radius*ms.radius
	discriminant := halfB*halfB - a*c

	setHitRecord := func(distance float64) {
		*rec = HitRecord{}
		rec.t = distance
		rec.p = *r.At(rec.t)
		outwardNormal := rec.p.Substract(ms.Center(r.tm)).Divide(ms.radius)
		rec.SetFaceNormal(r, outwardNormal)
		rec.material = ms.material
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

func (ms MovingSphere) BoundingBox(time0, time1 float64, outputBox *AABB) bool {
	box0 := NewAABB(
		*ms.Center(time0).Substract(NewVec3(ms.radius, ms.radius, ms.radius)),
		*ms.Center(time0).Add(NewVec3(ms.radius, ms.radius, ms.radius)),
	)
	box1 := NewAABB(
		*ms.Center(time1).Substract(NewVec3(ms.radius, ms.radius, ms.radius)),
		*ms.Center(time1).Add(NewVec3(ms.radius, ms.radius, ms.radius)),
	)
	*outputBox = *SurroundingBox(box0, box1)

	return true
}
