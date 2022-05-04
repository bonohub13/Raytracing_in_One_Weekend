// Axis-Aligned bounding boxes
package rtweekend

import (
	"math"
)

type AABB struct {
	minimum Point3
	maximum Point3
}

/* Returns a pointer for AABB (Axis-Aligned Bounding Boxes) object
   Params
       minimum Point3  Coordinate of passing interval
       maximum Point3  Coordinate of passing interval
*/
func NewAABB(minimum, maximum Point3) *AABB {
	aabb := new(AABB)

	aabb.minimum = minimum
	aabb.maximum = maximum

	return aabb
}

func SurroundingBox(box0, box1 *AABB) *AABB {
	small := NewPoint3(
		math.Min(box0.minimum.X(), box1.minimum.X()),
		math.Min(box0.minimum.Y(), box1.minimum.Y()),
		math.Min(box0.minimum.Z(), box1.minimum.Z()),
	)
	big := NewPoint3(
		math.Min(box0.maximum.X(), box1.maximum.X()),
		math.Min(box0.maximum.Y(), box1.maximum.Y()),
		math.Min(box0.maximum.Z(), box1.maximum.Z()),
	)

	return NewAABB(*small, *big)
}

// Returns minimum coordinate of passing interval in AABB
func (aabb *AABB) Min() *vec3.Point3 {
	return &aabb.minimum
}

// Returns maximum coordinate of passing interval in AABB
func (aabb *AABB) Max() *vec3.Point3 {
	return &aabb.maximum
}

func (aabb AABB) Hit(r *ray.Ray, t_min, t_max float64, rec *HitRecord) bool {
	var t0, t1, invD float64

	swap := func(t0, t1 *float64) {
		*t0, *t1 = *t1, *t0
	}

	for a := 0; a < 3; a++ {
		invD = 1.0 / r.Direction().E(a)
		t0 = (aabb.minimum.E(a) - r.Origin().E(a)) * invD
		t1 = (aabb.maximum.E(a) - r.Origin().E(a)) * invD

		if invD < 1 {
			swap(&t0, &t1)
		}

		if t0 > t_min {
			t_min = t0
		}

		if t_max > t1 {
			t_max = t1
		}

		if t_max <= t_min {
			return false
		}
	}

	return true
}

func (aabb AABB) BoundingBox(time0, time1 float64, outputBox *AABB) bool {
	return false
}
