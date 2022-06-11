package librt

func Dot(u *Vec3, v *Vec3) float64 {
	retval := 0.0

	for i := range u.e {
		retval += u.e[i] * v.e[i]
	}

	return retval
}

func Cross(u *Vec3, v *Vec3) *Vec3 {
	retval := &Vec3{}

	for i := range retval.e {
		retval.e[i] = u.e[(i+1)%3]*v.e[(i+2)%3] - v.e[(i+1)%3]*u.e[(i+2)%3]
	}

	return retval
}

func UnitVector(v *Vec3) *Vec3 {
	return v.DivFloat64(v.Length())
}

func RandomVec3() *Vec3 {
	return &Vec3{[3]float64{RandomFloat64(), RandomFloat64(), RandomFloat64()}}
}

func RandomVec3InRange(min, max float64) *Vec3 {
	return &Vec3{[3]float64{
		RandomFloat64InRange(min, max),
		RandomFloat64InRange(min, max),
		RandomFloat64InRange(min, max),
	}}
}

func RandomInUnitSphere() *Vec3 {
	for {
		p := RandomVec3InRange(-1, 1)

		if p.LengthSquared() < 1 {
			return p
		}
	}
}
