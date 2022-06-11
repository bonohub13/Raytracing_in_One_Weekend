package librt

type Ray struct {
	origin    Point3
	direction Vec3
}

func NewRay(origin Point3, direction Vec3) *Ray {
	return &Ray{
		origin,
		direction,
	}
}

func (r *Ray) Origin() Point3 {
	return r.origin
}

func (r *Ray) Direction() Vec3 {
	return r.direction
}

func (r *Ray) At(t float64) *Point3 {
	return r.origin.Add(r.direction.MultFloat64(t))
}
