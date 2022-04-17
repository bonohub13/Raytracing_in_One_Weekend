package rtweekendlib

import ()

// Getters for HitRecord
func (hr *HitRecord) P() *Point3 {
	return &hr.p
}

func (hr *HitRecord) Normal() *Vec3 {
	return &hr.normal
}

func (hr *HitRecord) T() float64 {
	return hr.t
}

func (hr *HitRecord) FrontFace() bool {
	return hr.frontFace
}

// Setters for HitRecord
func (hr *HitRecord) SetP(p *Point3) {
	hr.p = *p
}

func (hr *HitRecord) SetNormal(normal *Vec3) {
	hr.normal = *normal
}

func (hr *HitRecord) SetT(t float64) {
	hr.t = t
}

func (hr *HitRecord) SetFrontFace(frontFace bool) {
	hr.frontFace = frontFace
}
