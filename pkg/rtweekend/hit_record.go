package rtweekend

type HitRecord struct {
	p         Point3
	normal    Vec3
	material  interface{}
	t         float64
	u         float64
	v         float64
	frontFace bool
}

func NewHitRecord(p Point3, normal Vec3, t float64, frontFace bool) *HitRecord {
	hr := new(HitRecord)

	hr.p = p
	hr.normal = normal
	hr.t = t
	hr.frontFace = frontFace
	hr.u, hr.v = 0, 0

	return hr
}

func (hr *HitRecord) Material() interface{} { return hr.material }

func (hr *HitRecord) SetFaceNormal(r *Ray, outwardNormal *Vec3) {
	hr.frontFace = Dot(r.Direction(), outwardNormal) < 0
	if hr.frontFace {
		hr.normal = *outwardNormal
	} else {
		hr.normal = *outwardNormal.Negative()
	}
}