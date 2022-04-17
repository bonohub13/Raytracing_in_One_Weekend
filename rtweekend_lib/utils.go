package rtweekendlib

import (
	"fmt"
)

func Format(v *Vec3) {
	fmt.Printf("%f %f %f\n", v.e[0], v.e[1], v.e[2])
}

func (v *Vec3) Add(u *Vec3) *Vec3 {
	ans := new(Vec3)

	for i := 0; i < 3; i++ {
		ans.e[i] = v.e[i] + u.e[i]
	}

	return ans
}

func (v *Vec3) Substract(u *Vec3) *Vec3 {
	ans := new(Vec3)

	for i := 0; i < 3; i++ {
		ans.e[i] = v.e[i] - u.e[i]
	}

	return ans
}

func (v *Vec3) MultiplyVertices(u *Vec3) *Vec3 {
	ans := new(Vec3)

	for i := 0; i < 3; i++ {
		ans.e[i] = v.e[i] * u.e[i]
	}

	return ans
}

func (v *Vec3) Multiply(gain float64) *Vec3 {
	ans := new(Vec3)

	for i := 0; i < 3; i++ {
		ans.e[i] = v.e[i] * gain
	}

	return ans
}

func (v *Vec3) Divide(gain float64) *Vec3 {
	return v.Multiply(1 / gain)
}

func Dot(v *Vec3, u *Vec3) float64 {
	ans := 0.0

	for i := 0; i < 3; i++ {
		ans += v.e[i] * u.e[i]
	}

	return ans
}

func Cross(v *Vec3, u *Vec3) *Vec3 {
	ans := new(Vec3)

	for i := 0; i < 3; i++ {
		ans.e[i] = (u.e[(i+1)%3] * v.e[(i+2)%3]) - (u.e[(i+2)%3] * v.e[(i+1)%3])
	}

	return ans
}

func UnitVector(v *Vec3) *Vec3 {
	return v.Divide(v.Length())
}
