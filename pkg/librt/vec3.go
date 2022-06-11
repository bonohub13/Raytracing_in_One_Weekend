package librt

import (
	"fmt"
	"io"
	"math"
)

type Vec3 struct {
	e [3]float64
}

type Point3 = Vec3
type Color = Vec3

func NewVec3(e0, e1, e2 float64) *Vec3 {
	v := &Vec3{}

	v.e = [3]float64{e0, e1, e2}

	return v
}

func NewPoint3(e0, e1, e2 float64) *Point3 {
	return NewVec3(e0, e1, e2)
}

func NewColor(e0, e1, e2 float64) *Color {
	return NewVec3(e0, e1, e2)
}

func (v *Vec3) X() float64    { return v.e[0] }
func (v *Vec3) Y() float64    { return v.e[1] }
func (v *Vec3) Z() float64    { return v.e[2] }
func (v *Vec3) E() [3]float64 { return v.e }

func (v *Vec3) Length() float64 {
	return math.Sqrt(v.LengthSquared())
}

func (v *Vec3) LengthSquared() float64 {
	retval := 0.0

	for _, e := range v.e {
		retval += e * e
	}

	return retval
}

func (v *Vec3) Neg() *Vec3 {
	retval := &Vec3{}
	for i := range retval.e {
		retval.e[i] = -v.e[i]
	}

	return retval
}

func (v *Vec3) Add(u *Vec3) *Vec3 {
	retval := &Vec3{}

	for i := range retval.e {
		retval.e[i] = v.e[i] + u.e[i]
	}

	return retval
}

func (v *Vec3) Substract(u *Vec3) *Vec3 {
	return v.Add(u.Neg())
}

func (v *Vec3) MultFloat64(t float64) *Vec3 {
	retval := &Vec3{}

	for i := range retval.e {
		retval.e[i] = v.e[i] * t
	}

	return retval
}

func (v *Vec3) DivFloat64(t float64) *Vec3 {
	return v.MultFloat64(1.0 / t)
}

func (v *Vec3) MultVec3(u *Vec3) *Vec3 {
	retval := &Vec3{}

	for i := range v.e {
		retval.e[i] = v.e[i] * u.e[i]
	}

	return retval
}

func (v *Vec3) Format(writer io.Writer) {
	fmt.Fprint(writer, v.e[0], v.e[1], v.e[2])
}
