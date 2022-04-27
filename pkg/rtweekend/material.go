package rtweekend

type Material interface {
	Scatter(r_in *Ray, rec *HitRecord, attenuation *Color, scattered *Ray) bool
}

type Lambertian struct {
	albedo Color
}

type Metal struct {
	albedo Color
}

func NewLambertian(albedo Color) *Lambertian {
	return &Lambertian{albedo}
}

func NewMetal(albedo Color) *Metal {
	return &Metal{albedo}
}

func (l Lambertian) Scatter(
	r_in *Ray,
	rec *HitRecord,
	attenuation *Color,
	scattered *Ray,
) bool {
	scatterDirection := rec.normal.Add(RandomUnitVector())

	if scatterDirection.NearZero() {
		scatterDirection = &rec.normal
	}
	*scattered = *NewRay(rec.p, *scatterDirection)
	*attenuation = l.albedo

	return true
}

func (m Metal) Scatter(
	r_in *Ray,
	rec *HitRecord,
	attenuation *Color,
	scattered *Ray,
) bool {
	reflected := Reflect(UnitVector(&r_in.dir), &rec.normal)
	*scattered = *NewRay(rec.p, *reflected)
	*attenuation = m.albedo

	return Dot(&scattered.dir, &rec.normal) > 0
}
