package rtweekend

type Texture interface {
	Value(u, v float64, p *Point3) *Color
}
