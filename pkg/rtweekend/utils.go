package rtweekend

import (
	"fmt"
	"math"
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

func RandomVec3() *Vec3 {
	return NewVec3(RandomFloat64(), RandomFloat64(), RandomFloat64())
}

func RandomVec3InRange(min, max float64) *Vec3 {
	return NewVec3(
		RandomFloat64InRange(min, max),
		RandomFloat64InRange(min, max),
		RandomFloat64InRange(min, max),
	)
}

func RandomPoint3() *Point3 {
	return NewPoint3(RandomFloat64(), RandomFloat64(), RandomFloat64())
}

func RandomPoint3InRange(min, max float64) *Point3 {
	return NewPoint3(
		RandomFloat64InRange(min, max),
		RandomFloat64InRange(min, max),
		RandomFloat64InRange(min, max),
	)
}

func RandomColor() *Color {
	return NewColor(RandomFloat64(), RandomFloat64(), RandomFloat64())
}

func RandomColorInRange(min, max float64) *Color {
	return NewColor(
		RandomFloat64InRange(min, max),
		RandomFloat64InRange(min, max),
		RandomFloat64InRange(min, max),
	)
}

func RandomInUnitSphere() *Vec3 {
	for {
		p := RandomVec3InRange(1, -1)

		if p.LengthSquared() < 1 {
			return p
		}
	}
}

func RandomUnitVector() *Vec3 {
	return UnitVector(RandomInUnitSphere())
}

func RandomInHemisphere(normal Vec3) *Vec3 {
	inUnitSphere := RandomInUnitSphere()

	if Dot(inUnitSphere, &normal) > 0 {
		return inUnitSphere
	} else {
		return inUnitSphere.Negative()
	}
}

func RandomInUnitDisk() *Vec3 {
	for {
		p := NewVec3(
			RandomFloat64InRange(-1, 1),
			RandomFloat64InRange(-1, 1),
			0,
		)

		if p.LengthSquared() < 1 {
			return p
		}
	}
}

func Reflect(v, n *Vec3) *Vec3 {
	return v.Substract(n.Multiply(2 * Dot(v, n)))
}

func Refract(uv, n *Vec3, etaiOverEtat float64) *Vec3 {
	cosTheta := math.Min(Dot(uv.Negative(), n), 1)
	rOutPerp := uv.Add(n.Multiply(cosTheta)).Multiply(etaiOverEtat)
	rOutParallel := n.Multiply(
		-math.Sqrt(math.Abs(1 - rOutPerp.LengthSquared())),
	)

	return rOutPerp.Add(rOutParallel)
}
