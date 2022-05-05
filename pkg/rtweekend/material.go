package rtweekend

import (
	"math"
)

// Interface for material to raytrace
type Material interface {
    Emitted(u, v float64, p *Point3) *Color
	Scatter(r_in *Ray, rec *HitRecord, attenuation *Color, scattered *Ray) bool
}

// Material using the Lambertian method to draw object
type Lambertian struct {
	albedo interface{}
}

// Material with metalic charateristics
type Metal struct {
	albedo Color
	fuzz   float64
}

// Material that is dialectric
type Dialectric struct {
	ir float64
}

/* Returns a pointer to Lambertian object
   Params
       albedo Color    Color for texture in Lambertian object
*/
func NewLambertianFromColor(albedo Color) *Lambertian {
	return &Lambertian{*NewSolidColor(albedo)}
}

/* Returns a pointer to Lambertian object
   Params
       albedo Texture  Texture for Lambertian object
*/
func NewLambertian(texture Texture) *Lambertian {
	return &Lambertian{texture}
}

/* Returns a pointer to Metal object
   Params
       albedo      Color       Color for metalic object
       fuzziness   float64     Adding fuzziness to metalic object
*/
func NewMetal(albedo Color, fuzziness float64) *Metal {
	return &Metal{albedo, fuzziness}
}

func NewDialectric(indexOfRefraction float64) *Dialectric {
	return &Dialectric{indexOfRefraction}
}

func (d *Dialectric) reflectance(cosine, refIdx float64) float64 {
	r0 := (1 - refIdx) / (1 + refIdx)
	r0 *= r0

	return r0 + (1-r0)*math.Pow((1-cosine), 5)
}

func (l Lambertian) Emitted(u, v float64, p *Point3) *Color {
    return NewColor(0, 0, 0)
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
	*scattered = *NewRay(rec.p, *scatterDirection, r_in.tm)
	*attenuation = *l.albedo.(Texture).Value(rec.u, rec.v, &rec.p)

	return true
}

func (m Metal) Emitted(u, v float64, p *Point3) *Color {
    return NewColor(0, 0, 0)
}

func (m Metal) Scatter(
	r_in *Ray,
	rec *HitRecord,
	attenuation *Color,
	scattered *Ray,
) bool {
	reflected := Reflect(UnitVector(&r_in.dir), &rec.normal)
	*scattered = *NewRay(
		rec.p,
		*reflected.Add(RandomInUnitSphere().Multiply(m.fuzz)),
		r_in.tm,
	)
	*attenuation = m.albedo

	return Dot(&scattered.dir, &rec.normal) > 0
}

func (d Dialectric) Emitted(u, v float64, p *Point3) *Color {
    return NewColor(0, 0, 0)
}

func (d Dialectric) Scatter(
	r_in *Ray,
	rec *HitRecord,
	attenuation *Color,
	scattered *Ray,
) bool {
	var direction *Vec3
	refractionRatio := d.ir

	if rec.frontFace {
		refractionRatio = 1.0 / d.ir
	}

	unitDirection := UnitVector(&r_in.dir)
	cosTheta := math.Min(Dot(unitDirection.Negative(), &rec.normal), 1.0)
	sinTheta := math.Sqrt(1.0 - cosTheta*cosTheta)
	cannotRefract := refractionRatio*sinTheta > 1

	if cannotRefract ||
		(d.reflectance(cosTheta, refractionRatio) > RandomFloat64()) {
		direction = Reflect(unitDirection, &rec.normal)
	} else {
		direction = Refract(unitDirection, &rec.normal, refractionRatio)
	}

	*attenuation = *NewColor(1, 1, 1)
	*scattered = *NewRay(rec.p, *direction, r_in.tm)

	return true
}
