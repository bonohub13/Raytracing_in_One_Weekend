package librt

import (
	"math"
	"math/rand"
	"time"
)

const (
	PI = 3.1415926535897932385
)

var INFINITY = math.Inf(1)

func DegreesToRadians(degrees float64) float64 {
	return degrees * PI / 180
}

func InitRandom() {
	rand.Seed(time.Now().UnixNano())
}

func RandomFloat64() float64 {
	return rand.Float64()
}

func RandomFloat64InRange(min, max float64) float64 {
	return min + (max-min)*RandomFloat64()
}

func Clamp(x, min, max float64) float64 {
	if x < min {
		return min
	}
	if x > max {
		return max
	}
	return x
}

func RayColor(r *Ray, world *HittableList, depth int) *Color {
	if depth <= 0 {
		return NewColor(0, 0, 0)
	}
	if hitSomething, hit := world.Hit(r, 0, INFINITY); hitSomething {
		target := hit.p.Add(&hit.normal).Add(RandomInUnitSphere())

		return RayColor(
			NewRay(hit.p, *target.Substract(&hit.p)),
			world,
			depth-1).MultFloat64(0.5)
	}
	unitDirection := UnitVector(&r.direction)
	t := 0.5 * (unitDirection.Y() + 1.0)

	return NewColor(1.0, 1.0, 1.0).MultFloat64(1.0 - t).Add(
		NewColor(0.5, 0.7, 1.0).MultFloat64(t))
}

func HitSphere(center *Point3, radius float64, r *Ray) float64 {
	dir := r.Direction()
	oc := r.origin.Substract(center)
	a := dir.LengthSquared()
	half_b := Dot(oc, &dir)
	c := oc.LengthSquared() - radius*radius
	discriminant := half_b*half_b - a*c

	if discriminant < 0.0 {
		return -1.0
	} else {
		return (-half_b - math.Sqrt(discriminant)) / a
	}
}
