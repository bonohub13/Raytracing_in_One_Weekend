package librt

type HitRecord struct {
	p         Point3
	normal    Vec3
	t         float64
	frontFace bool
}

func (hr *HitRecord) SetFaceNormal(r *Ray, outwardNormal *Vec3) {
	hr.frontFace = Dot(&r.direction, outwardNormal) < 0
	if hr.frontFace {
		hr.normal = *outwardNormal
	} else {
		hr.normal = *outwardNormal.Neg()
	}
}
