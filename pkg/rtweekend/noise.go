package rtweekend

type NoiseTexture struct {
	noise Perlin
	scale float64
}

/* Returns a poiner of NoiseTexture
   Params
       scale float64   Scale to the noise
*/
func NewNoiseTexture(scale float64) *NoiseTexture {
	nt := new(NoiseTexture)
	nt.noise = *NewPerlin()
	nt.scale = scale

	return nt
}

func (nt NoiseTexture) Value(u, v float64, p *Point3) *Color {
	return NewColor(1, 1, 1).Multiply(
		0.5 * (1 + nt.noise.Turb(*p.Multiply(nt.scale))),
	)
}
