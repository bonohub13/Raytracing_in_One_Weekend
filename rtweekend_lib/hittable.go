package rtweekendlib

import ()

type HitRecord struct {
	p         Point3
	normal    Vec3
	t         float64
	frontFace bool
}

type Hittable interface {
	Hit(r *Ray, t_min, t_max float64, rec *HitRecord) bool
}

func NewHitRecord(p Point3, normal Vec3, t float64, frontFace bool) *HitRecord {
	hr := new(HitRecord)

	hr.p = p
	hr.normal = normal
	hr.t = t
	hr.frontFace = frontFace

	return hr
}

func (hr *HitRecord) SetFaceNormal(r *Ray, outwardNormal *Vec3) {
	hr.frontFace = Dot(r.Direction(), outwardNormal) < 0
	if hr.frontFace {
		hr.normal = *outwardNormal
	} else {
		hr.normal = *outwardNormal.Negative()
	}
}
