package rtweekend

import (
	"math"
)

type Vec3 struct {
	e [3]float64
}

// Aliases
type Point3 = Vec3
type Color = Vec3

func NewVec3(e0, e1, e2 float64) *Vec3 {
	vec := new(Vec3)

	vec.e = [3]float64{e0, e1, e2}

	return vec
}

func NewPoint3(e0, e1, e2 float64) *Point3 {
	vec := new(Point3)

	vec.e = [3]float64{e0, e1, e2}

	return vec
}

func NewColor(e0, e1, e2 float64) *Color {
	vec := new(Color)

	vec.e = [3]float64{e0, e1, e2}

	return vec
}

func (v *Vec3) X() float64 {
	return v.e[0]
}

func (v *Vec3) Y() float64 {
	return v.e[1]
}

func (v *Vec3) Z() float64 {
	return v.e[2]
}

func (v *Vec3) E(index int) float64 {
	return v.e[index]
}

func (v *Vec3) Negative() *Vec3 {
	neg := new(Vec3)

	for i := 0; i < 3; i++ {
		neg.e[i] = -v.e[i]
	}

	return neg
}

func (v *Vec3) AddToThis(u *Vec3) {
	for i := 0; i < 3; i++ {
		v.e[i] += u.e[i]
	}
}

func (v *Vec3) MultToThis(gain float64) {
	for i := 0; i < 3; i++ {
		v.e[i] *= gain
	}
}

func (v *Vec3) DivToThis(gain float64) {
	v.MultToThis(1 / gain)
}

func (v *Vec3) LengthSquared() float64 {
	retval := 0.0

	for i := 0; i < 3; i++ {
		retval += v.e[i] * v.e[i]
	}

	return retval
}

func (v *Vec3) Length() float64 {
	return math.Sqrt(v.LengthSquared())
}

func (v *Vec3) NearZero() bool {
	s := 1e-8

	return (math.Abs(v.e[0]) < s) &&
		(math.Abs(v.e[1]) < s) &&
		(math.Abs(v.e[2]) < s)
}
