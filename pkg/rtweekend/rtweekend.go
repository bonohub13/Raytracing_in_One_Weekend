package rtweekend

import (
	"math"
	"math/rand"
	"time"
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

func RayColor(r *Ray, world Hittable, depth int) *Color {
	rec := new(HitRecord)

	if depth <= 0 {
		return NewColor(0, 0, 0)
	}

	c0 := NewColor(1.0, 1.0, 1.0)

	if world.Hit(r, 0.001, math.Inf(1), rec) {
		scattered := Ray{}
		attenuation := Color{}

		if rec.material.(Material).Scatter(r, rec, &attenuation, &scattered) {
			return attenuation.MultiplyVertices(
				RayColor(&scattered, world, depth-1),
			)
		}

		return NewColor(0, 0, 0)
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

func InitRandom() {
	rand.Seed(time.Now().UnixNano())
}

func RandomFloat64() float64 {
	return rand.Float64()
}

func RandomFloat64InRange(min, max float64) float64 {
	return min + (max-min)*RandomFloat64()
}

func RandomIntInRange(min, max int) int {
	return int(RandomFloat64InRange(float64(min), float64(max)+1))
}

func Clamp(x, min, max float64) float64 {
	if x < min {
		return min
	} else if x > max {
		return max
	} else {
		return x
	}
}
